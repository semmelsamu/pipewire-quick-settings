"""Utilities for extracting information from PipeWire dumps."""
from __future__ import annotations

from typing import Any, Dict, List, Optional


def _get_default_sink_name(dump: List[Dict[str, Any]]) -> Optional[str]:
    """Return the default audio sink name from PipeWire metadata."""
    for obj in dump:
        if obj.get("type") != "PipeWire:Interface:Metadata":
            continue
        if obj.get("props", {}).get("metadata.name") != "default":
            continue
        for item in obj.get("metadata", []):
            if item.get("key") == "default.audio.sink":
                value = item.get("value")
                if isinstance(value, dict):
                    return value.get("name")
                return value
    return None


def parse_sinks(dump: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
    """Extract sinks (Audio/Sink nodes) from the given ``pw-dump`` output."""
    sinks: List[Dict[str, Any]] = []
    for obj in dump:
        if obj.get("type") != "PipeWire:Interface:Node":
            continue
        props = obj.get("info", {}).get("props", {})
        if not props.get("media.class", "").startswith("Audio/Sink"):
            continue
        sinks.append(
            {
                "id": obj["id"],
                "name": props.get("node.name"),
                "description": props.get("node.description") or props.get("node.name"),
                "state": obj.get("info", {}).get("state", "unknown"),
                "device.id": props.get("device.id"),
            }
        )
    return sinks


def get_current_sink(dump: List[Dict[str, Any]]) -> Optional[Dict[str, Any]]:
    """Return the currently configured default sink, if it can be identified."""
    default_name = _get_default_sink_name(dump)
    if default_name is None:
        return None
    for sink in parse_sinks(dump):
        if sink.get("name") == default_name:
            return sink
    return None


def parse_card(dump: List[Dict[str, Any]], card_id: int) -> Optional[Dict[str, Any]]:
    """Return the card (device) identified by ``card_id`` from the dump, if present."""
    for obj in dump:
        if obj.get("id") != card_id or obj.get("type") != "PipeWire:Interface:Device":
            continue
        props = obj.get("info", {}).get("props", {})
        params = obj.get("info", {}).get("params", {})
        active_profile = None
        active_profile_index = None
        if params.get("Profile"):
            profile = params["Profile"][0]
            active_profile_index = profile.get("index")
            active_profile = profile.get("description") or profile.get("name")
        return {
            "id": obj["id"],
            "description": props.get("device.description") or props.get("device.nick"),
            "profile": active_profile or "unknown",
            "profile_index": active_profile_index,
            "params": params,
        }
    return None


def parse_profiles(card: Dict[str, Any]) -> List[Dict[str, Any]]:
    """Extract profiles information from a card dictionary."""
    profiles: List[Dict[str, Any]] = []
    for profile in card.get("params", {}).get("EnumProfile", []):
        profiles.append(
            {
                "index": profile.get("index"),
                "name": profile.get("name"),
                "description": profile.get("description"),
                "available": profile.get("available"),
            }
        )
    return profiles


def get_current_profile(card: Dict[str, Any]) -> Optional[Dict[str, Any]]:
    """Return the active profile entry for the provided card, if available."""
    active_index = card.get("profile_index")
    if active_index is None:
        return None
    for profile in parse_profiles(card):
        if profile.get("index") == active_index:
            return profile
    return None
