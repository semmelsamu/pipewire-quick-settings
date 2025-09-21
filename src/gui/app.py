"""Gtk-based GUI scaffold for PipeWire quick settings."""
from __future__ import annotations

import gi

gi.require_version("Gtk", "4.0")

from gi.repository import Gio, Gtk


class QuickSettingsWindow(Gtk.ApplicationWindow):
    """Main application window with static placeholder content."""

    def __init__(self, app: Gtk.Application) -> None:
        super().__init__(application=app, title="Pipewire Quick Settings")
        self.set_default_size(420, 0)
        self.set_resizable(False)

        root = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=15)
        root.set_margin_top(20)
        root.set_margin_bottom(20)
        root.set_margin_start(20)
        root.set_margin_end(20)
        self.set_child(root)

        title = Gtk.Label(label="Pipewire Quick Settings")
        title.get_style_context().add_class("title-4")
        title.set_valign(Gtk.Align.START)
        root.append(title)

        slider_row = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=12)
        root.append(slider_row)

        mute_button = Gtk.Button(label="M")
        mute_button.set_hexpand(False)
        mute_button.set_size_request(48, 48)
        slider_row.append(mute_button)

        volume_adjustment = Gtk.Adjustment(lower=0.0, upper=1.0, step_increment=0.01, page_increment=0.1, value=0.65)
        slider = Gtk.Scale(orientation=Gtk.Orientation.HORIZONTAL, adjustment=volume_adjustment)
        slider.set_hexpand(True)
        slider.set_draw_value(False)
        slider_row.append(slider)

        dropdown_row = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=10)
        root.append(dropdown_row)

        device_button = Gtk.Button(label="Output Device Name")
        device_button.set_hexpand(True)
        dropdown_row.append(device_button)

        card_button = Gtk.Button(label="Card")
        card_button.set_hexpand(False)
        dropdown_row.append(card_button)


class QuickSettingsApplication(Gtk.Application):
    """Gtk application wrapper that presents the static placeholder window."""

    def __init__(self) -> None:
        super().__init__(application_id="dev.pipewire.quicksettings", flags=Gio.ApplicationFlags.FLAGS_NONE)

    def do_activate(self) -> None:  # type: ignore[override]
        window = self.props.active_window
        if window is None:
            window = QuickSettingsWindow(self)
        window.present()


def run_gui() -> None:
    """Start the Gtk application using placeholder data."""
    app = QuickSettingsApplication()
    app.run()
