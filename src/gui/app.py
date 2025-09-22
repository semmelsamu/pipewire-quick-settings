"""Gtk-based GUI for PipeWire quick settings."""
from __future__ import annotations

from dataclasses import dataclass
from typing import Any, Dict, List, Optional, Tuple

import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gio, Gtk

from pipewire_parsers import (
    get_current_profile,
    get_current_sink,
    parse_card,
    parse_profiles,
    parse_sinks,
)
from pw_client import pw_dump


@dataclass
class SinkItem:
    """Representation of a PipeWire sink for display."""

    id: int
    description: str
    device_id: Optional[int]
    volume: Optional[float]
    volume_linear: Optional[float]
    mute: Optional[bool]
    name: Optional[str] = None

    @property
    def display_name(self) -> str:
        return self.description or self.name or f"Sink {self.id}"


@dataclass
class ProfileItem:
    """Representation of a PipeWire profile for display."""

    index: int
    name: Optional[str]
    description: Optional[str]
    available: Optional[str] = None

    @property
    def display_name(self) -> str:
        return self.description or self.name or f"Profile {self.index}"


class PipewireSnapshot:
    """Captures the PipeWire state needed for the GUI."""

    def __init__(self) -> None:
        self.dump: List[Dict[str, Any]] = []
        self.sinks: List[SinkItem] = []
        self.sink_by_id: Dict[int, SinkItem] = {}
        self.default_sink_id: Optional[int] = None
        self.refresh()

    def refresh(self) -> None:
        data = pw_dump()
        self.dump = data
        raw_sinks = parse_sinks(data)

        sinks: List[SinkItem] = []
        lookup: Dict[int, SinkItem] = {}
        for raw in raw_sinks:
            sink = self._to_sink_item(raw)
            if sink is None:
                continue
            sinks.append(sink)
            lookup[sink.id] = sink

        self.sinks = sinks
        self.sink_by_id = lookup

        current = get_current_sink(data)
        if current is not None:
            try:
                self.default_sink_id = int(current["id"])
            except (KeyError, TypeError, ValueError):
                self.default_sink_id = None
        elif sinks:
            self.default_sink_id = sinks[0].id
        else:
            self.default_sink_id = None

    def _to_sink_item(self, raw: Dict[str, Any]) -> Optional[SinkItem]:
        sink_id = raw.get("id")
        if sink_id is None:
            return None
        try:
            sink_id_int = int(sink_id)
        except (TypeError, ValueError):
            return None

        description = raw.get("description") or raw.get("name") or f"Sink {sink_id_int}"

        device_raw = raw.get("device.id")
        device_id: Optional[int] = None
        if device_raw is not None:
            try:
                device_id = int(device_raw)
            except (TypeError, ValueError):
                device_id = None

        volume = raw.get("volume") if isinstance(raw.get("volume"), (int, float)) else None
        volume_linear = raw.get("volume_linear") if isinstance(raw.get("volume_linear"), (int, float)) else None

        mute_raw = raw.get("mute")
        if isinstance(mute_raw, bool):
            mute = mute_raw
        elif mute_raw is None:
            mute = None
        else:
            mute = bool(mute_raw)

        return SinkItem(
            id=sink_id_int,
            description=str(description),
            device_id=device_id,
            volume=float(volume) if volume is not None else None,
            volume_linear=float(volume_linear) if volume_linear is not None else None,
            mute=mute,
            name=raw.get("name"),
        )

    def get_profiles(self, sink_id: int) -> Tuple[List[ProfileItem], Optional[int]]:
        sink = self.sink_by_id.get(sink_id)
        if sink is None or sink.device_id is None:
            return [], None

        card = parse_card(self.dump, sink.device_id)
        if card is None:
            return [], None

        profile_items: List[ProfileItem] = []
        for profile in parse_profiles(card):
            index = profile.get("index")
            if index is None:
                continue
            try:
                index_int = int(index)
            except (TypeError, ValueError):
                continue
            profile_items.append(
                ProfileItem(
                    index=index_int,
                    name=profile.get("name"),
                    description=profile.get("description"),
                    available=profile.get("available"),
                )
            )

        active = get_current_profile(card)
        active_index: Optional[int] = None
        if active is not None:
            idx = active.get("index")
            if idx is not None:
                try:
                    active_index = int(idx)
                except (TypeError, ValueError):
                    active_index = None

        return profile_items, active_index


class QuickSettingsWindow(Gtk.ApplicationWindow):
    """Main application window bound to PipeWire state."""

    def __init__(self, app: Gtk.Application) -> None:
        super().__init__(application=app, title="Pipewire Quick Settings")
        self.set_default_size(420, 240)
        self.set_resizable(False)

        self.snapshot = PipewireSnapshot()
        self._ignore_sink_signal = False
        self.sink_ids: List[int] = []
        self.profile_items: List[ProfileItem] = []

        root = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=20)
        root.set_margin_top(24)
        root.set_margin_bottom(24)
        root.set_margin_start(24)
        root.set_margin_end(24)
        self.set_child(root)

        title = Gtk.Label(label="Pipewire Quick Settings")
        title.get_style_context().add_class("title-2")
        title.set_valign(Gtk.Align.START)
        root.append(title)

        slider_row = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=12)
        root.append(slider_row)

        self.mute_toggle = Gtk.ToggleButton(label="M")
        self.mute_toggle.set_hexpand(False)
        self.mute_toggle.set_size_request(48, 48)
        self.mute_toggle.set_sensitive(False)
        slider_row.append(self.mute_toggle)

        self.volume_adjustment = Gtk.Adjustment(lower=0.0, upper=1.5, step_increment=0.01, page_increment=0.1, value=0.0)
        self.volume_scale = Gtk.Scale(orientation=Gtk.Orientation.HORIZONTAL, adjustment=self.volume_adjustment)
        self.volume_scale.set_hexpand(True)
        self.volume_scale.set_draw_value(False)
        self.volume_scale.set_sensitive(False)
        slider_row.append(self.volume_scale)

        dropdown_row = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=12)
        root.append(dropdown_row)

        self.sink_model = Gtk.StringList.new([])  # type: ignore[arg-type]
        self.sink_dropdown = Gtk.DropDown(model=self.sink_model)
        self.sink_dropdown.set_hexpand(True)
        self.sink_dropdown.connect("notify::selected", self.on_sink_selected)
        dropdown_row.append(self.sink_dropdown)

        self.profile_model = Gtk.StringList.new([])  # type: ignore[arg-type]
        self.profile_dropdown = Gtk.DropDown(model=self.profile_model)
        self.profile_dropdown.set_hexpand(False)
        dropdown_row.append(self.profile_dropdown)

        self.populate_from_snapshot()

    def populate_from_snapshot(self) -> None:
        sink_labels = [sink.display_name for sink in self.snapshot.sinks]
        self.sink_ids = [sink.id for sink in self.snapshot.sinks]

        sink_model = Gtk.StringList.new(sink_labels)  # type: ignore[arg-type]
        self.sink_model = sink_model
        self.sink_dropdown.set_model(sink_model)

        if not self.sink_ids:
            self.sink_dropdown.set_selected(Gtk.INVALID_LIST_POSITION)
            self.clear_details()
            return

        selected_index = self._index_for_sink(self.snapshot.default_sink_id)
        if selected_index is None:
            selected_index = 0

        self._ignore_sink_signal = True
        self.sink_dropdown.set_selected(selected_index)
        self._ignore_sink_signal = False

        self.update_details_for_sink(self.sink_ids[selected_index])

    def _index_for_sink(self, sink_id: Optional[int]) -> Optional[int]:
        if sink_id is None:
            return None
        try:
            return self.sink_ids.index(sink_id)
        except ValueError:
            return None

    def on_sink_selected(self, dropdown: Gtk.DropDown, _param: Gio.ParamSpec) -> None:
        if self._ignore_sink_signal:
            return

        index = dropdown.get_selected()
        if index == Gtk.INVALID_LIST_POSITION:
            self.clear_details()
            return
        if index < 0 or index >= len(self.sink_ids):
            self.clear_details()
            return

        sink_id = self.sink_ids[index]
        self.update_details_for_sink(sink_id)

    def update_details_for_sink(self, sink_id: int) -> None:
        sink = self.snapshot.sink_by_id.get(sink_id)
        if sink is None:
            self.clear_details()
            return

        if sink.volume is not None:
            value = max(0.0, min(float(sink.volume), self.volume_adjustment.get_upper()))
            self.volume_adjustment.set_value(value)
            self.volume_scale.set_sensitive(True)
            self.volume_scale.set_tooltip_text(f"Volume: {int(value * 100)}%")
        else:
            self.volume_adjustment.set_value(0.0)
            self.volume_scale.set_sensitive(False)
            self.volume_scale.set_tooltip_text("Volume unavailable")

        if sink.mute is None:
            self.mute_toggle.set_active(False)
            self.mute_toggle.set_sensitive(False)
            self.mute_toggle.set_tooltip_text("Mute state unavailable")
        else:
            self.mute_toggle.set_sensitive(True)
            self.mute_toggle.set_active(bool(sink.mute))
            self.mute_toggle.set_tooltip_text("Muted" if sink.mute else "Unmuted")

        profiles, active_index = self.snapshot.get_profiles(sink_id)
        self.profile_items = profiles
        profile_labels = [profile.display_name for profile in profiles]
        profile_model = Gtk.StringList.new(profile_labels)  # type: ignore[arg-type]
        self.profile_model = profile_model
        self.profile_dropdown.set_model(profile_model)

        if not profiles:
            self.profile_dropdown.set_selected(Gtk.INVALID_LIST_POSITION)
            self.profile_dropdown.set_sensitive(False)
            return

        selected_profile = self._index_for_profile(active_index)
        if selected_profile is None:
            selected_profile = 0

        self.profile_dropdown.set_selected(selected_profile)
        self.profile_dropdown.set_sensitive(True)

    def _index_for_profile(self, profile_index: Optional[int]) -> Optional[int]:
        if profile_index is None:
            return None
        for pos, item in enumerate(self.profile_items):
            if item.index == profile_index:
                return pos
        return None

    def clear_details(self) -> None:
        self.volume_adjustment.set_value(0.0)
        self.volume_scale.set_sensitive(False)
        self.volume_scale.set_tooltip_text("Volume unavailable")
        self.mute_toggle.set_active(False)
        self.mute_toggle.set_sensitive(False)
        self.mute_toggle.set_tooltip_text("Mute state unavailable")

        self.profile_items = []
        empty_model = Gtk.StringList.new([])  # type: ignore[arg-type]
        self.profile_model = empty_model
        self.profile_dropdown.set_model(empty_model)
        self.profile_dropdown.set_selected(Gtk.INVALID_LIST_POSITION)
        self.profile_dropdown.set_sensitive(False)


class QuickSettingsApplication(Gtk.Application):
    """Gtk application wrapper that presents the PipeWire snapshot."""

    def __init__(self) -> None:
        super().__init__(application_id="dev.pipewire.quicksettings", flags=Gio.ApplicationFlags.FLAGS_NONE)

    def do_activate(self) -> None:  # type: ignore[override]
        window = self.props.active_window
        if window is None:
            window = QuickSettingsWindow(self)
        window.present()


def run_gui() -> None:
    """Start the Gtk application using live PipeWire data."""
    app = QuickSettingsApplication()
    app.run()
