"""Gtk application entry point for PipeWire quick settings."""
from __future__ import annotations

import gi

gi.require_version("Gtk", "4.0")
from gi.repository import Gio, Gtk

from .window import QuickSettingsWindow


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
