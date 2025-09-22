"""Provides a snapshot of PipeWire state for the GUI."""
from __future__ import annotations

from typing import Any, Dict, List, Optional, Tuple

from pipewire_parsers import (
    get_current_profile,
    get_current_sink,
    parse_card,
    parse_profiles,
    parse_sinks,
)
from pw_client import pw_dump

from .models import ProfileItem, SinkItem


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
