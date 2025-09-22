"""Gtk window for the PipeWire quick settings UI."""
from __future__ import annotations

from typing import List, Optional

import gi

gi.require_version("Gtk", "4.0")
from gi.repository import Gio, Gtk

from pw_client import set_default_sink, set_mute, set_profile, set_volume
from .models import ProfileItem
from .snapshot import PipewireSnapshot


class QuickSettingsWindow(Gtk.ApplicationWindow):
    """Main application window bound to PipeWire state."""

    def __init__(self, app: Gtk.Application) -> None:
        super().__init__(application=app, title="Pipewire Quick Settings")
        self.set_default_size(420, 240)
        self.set_resizable(False)

        self.snapshot = PipewireSnapshot()
        self.sink_ids: List[int] = []
        self.profile_items: List[ProfileItem] = []
        self.active_sink_id: Optional[int] = None

        self._ignore_sink_signal = False
        self._ignore_volume_signal = False
        self._ignore_mute_signal = False
        self._ignore_profile_signal = False

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
        self.mute_toggle.connect("toggled", self.on_mute_toggled)
        slider_row.append(self.mute_toggle)

        self.volume_adjustment = Gtk.Adjustment(lower=0.0, upper=1.5, step_increment=0.01, page_increment=0.1, value=0.0)
        self.volume_scale = Gtk.Scale(orientation=Gtk.Orientation.HORIZONTAL, adjustment=self.volume_adjustment)
        self.volume_scale.set_hexpand(True)
        self.volume_scale.set_draw_value(False)
        self.volume_scale.set_sensitive(False)
        self.volume_scale.connect("value-changed", self.on_volume_changed)
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
        self.profile_dropdown.connect("notify::selected", self.on_profile_selected)
        dropdown_row.append(self.profile_dropdown)

        self.populate_from_snapshot()

    def populate_from_snapshot(self, preferred_sink_id: Optional[int] = None) -> None:
        sink_labels = [sink.display_name for sink in self.snapshot.sinks]
        self.sink_ids = [sink.id for sink in self.snapshot.sinks]

        sink_model = Gtk.StringList.new(sink_labels)  # type: ignore[arg-type]
        self.sink_model = sink_model

        self._ignore_sink_signal = True
        self.sink_dropdown.set_model(sink_model)

        if not self.sink_ids:
            self.sink_dropdown.set_selected(Gtk.INVALID_LIST_POSITION)
            self._ignore_sink_signal = False
            self.clear_details()
            return

        if preferred_sink_id is not None:
            selected_index = self._index_for_sink(preferred_sink_id)
        else:
            selected_index = self._index_for_sink(self.snapshot.default_sink_id)
        if selected_index is None:
            selected_index = 0

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
        set_default_sink(sink_id)
        self.refresh_snapshot(sink_id)

    def update_details_for_sink(self, sink_id: int) -> None:
        self.active_sink_id = sink_id
        sink = self.snapshot.sink_by_id.get(sink_id)
        if sink is None:
            self.clear_details()
            return

        if sink.volume is not None:
            value = max(0.0, min(float(sink.volume), self.volume_adjustment.get_upper()))
            self._ignore_volume_signal = True
            self.volume_adjustment.set_value(value)
            self._ignore_volume_signal = False
            self.volume_scale.set_sensitive(True)
            self.volume_scale.set_tooltip_text(f"Volume: {int(value * 100)}%")
        else:
            self._ignore_volume_signal = True
            self.volume_adjustment.set_value(0.0)
            self._ignore_volume_signal = False
            self.volume_scale.set_sensitive(False)
            self.volume_scale.set_tooltip_text("Volume unavailable")

        if sink.mute is None:
            self._ignore_mute_signal = True
            self.mute_toggle.set_active(False)
            self._ignore_mute_signal = False
            self.mute_toggle.set_sensitive(False)
            self.mute_toggle.set_tooltip_text("Mute state unavailable")
        else:
            self._ignore_mute_signal = True
            self.mute_toggle.set_active(bool(sink.mute))
            self._ignore_mute_signal = False
            self.mute_toggle.set_sensitive(True)
            self.mute_toggle.set_tooltip_text("Muted" if sink.mute else "Unmuted")

        profiles, active_index = self.snapshot.get_profiles(sink_id)
        self.profile_items = profiles
        profile_labels = [profile.display_name for profile in profiles]
        profile_model = Gtk.StringList.new(profile_labels)  # type: ignore[arg-type]
        self.profile_model = profile_model

        self._ignore_profile_signal = True
        self.profile_dropdown.set_model(profile_model)

        if not profiles:
            self.profile_dropdown.set_selected(Gtk.INVALID_LIST_POSITION)
            self.profile_dropdown.set_sensitive(False)
            self._ignore_profile_signal = False
            return

        selected_profile = self._index_for_profile(active_index)
        if selected_profile is None:
            selected_profile = 0

        self.profile_dropdown.set_selected(selected_profile)
        self.profile_dropdown.set_sensitive(True)
        self._ignore_profile_signal = False

    def on_volume_changed(self, scale: Gtk.Scale) -> None:
        if self._ignore_volume_signal or self.active_sink_id is None:
            return
        if not self.volume_scale.get_sensitive():
            return

        value = scale.get_value()
        set_volume(self.active_sink_id, f"{value:.4f}")
        self.refresh_snapshot(self.active_sink_id)

    def on_mute_toggled(self, button: Gtk.ToggleButton) -> None:
        if self._ignore_mute_signal or self.active_sink_id is None:
            return
        if not self.mute_toggle.get_sensitive():
            return

        set_mute(self.active_sink_id, bool(button.get_active()))
        self.refresh_snapshot(self.active_sink_id)

    def on_profile_selected(self, dropdown: Gtk.DropDown, _param: Gio.ParamSpec) -> None:
        if self._ignore_profile_signal or self.active_sink_id is None:
            return

        index = dropdown.get_selected()
        if index == Gtk.INVALID_LIST_POSITION:
            return
        if index < 0 or index >= len(self.profile_items):
            return

        sink = self.snapshot.sink_by_id.get(self.active_sink_id)
        if sink is None or sink.device_id is None:
            return

        profile = self.profile_items[index]
        set_profile(sink.device_id, profile.index)
        self.refresh_snapshot(self.active_sink_id)

    def _index_for_profile(self, profile_index: Optional[int]) -> Optional[int]:
        if profile_index is None:
            return None
        for pos, item in enumerate(self.profile_items):
            if item.index == profile_index:
                return pos
        return None

    def clear_details(self) -> None:
        self.active_sink_id = None

        self._ignore_volume_signal = True
        self.volume_adjustment.set_value(0.0)
        self._ignore_volume_signal = False
        self.volume_scale.set_sensitive(False)
        self.volume_scale.set_tooltip_text("Volume unavailable")

        self._ignore_mute_signal = True
        self.mute_toggle.set_active(False)
        self._ignore_mute_signal = False
        self.mute_toggle.set_sensitive(False)
        self.mute_toggle.set_tooltip_text("Mute state unavailable")

        self.profile_items = []
        empty_model = Gtk.StringList.new([])  # type: ignore[arg-type]
        self.profile_model = empty_model

        self._ignore_profile_signal = True
        self.profile_dropdown.set_model(empty_model)
        self.profile_dropdown.set_selected(Gtk.INVALID_LIST_POSITION)
        self._ignore_profile_signal = False
        self.profile_dropdown.set_sensitive(False)

    def refresh_snapshot(self, preferred_sink_id: Optional[int]) -> None:
        self.snapshot.refresh()
        self.populate_from_snapshot(preferred_sink_id)
