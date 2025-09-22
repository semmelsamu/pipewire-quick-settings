"""Dataclasses used by the PipeWire quick settings GUI."""
from __future__ import annotations

from dataclasses import dataclass
from typing import Optional


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
