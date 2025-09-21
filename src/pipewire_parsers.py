"""Utilities for extracting information from PipeWire dumps."""
from __future__ import annotations

from typing import Any, Dict, List, Optional


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
                "description": props.get("node.description") or props.get("node.name"),
                "state": obj.get("info", {}).get("state", "unknown"),
                "device.id": props.get("device.id"),
            }
        )
    return sinks


def parse_card(dump: List[Dict[str, Any]], card_id: int) -> Optional[Dict[str, Any]]:
    """Return the card (device) identified by ``card_id`` from the dump, if present."""
    for obj in dump:
        if obj.get("id") != card_id or obj.get("type") != "PipeWire:Interface:Device":
            continue
        props = obj.get("info", {}).get("props", {})
        params = obj.get("info", {}).get("params", {})
        active_profile = None
        if params.get("Profile"):
            profile = params["Profile"][0]
            active_profile = profile.get("description") or profile.get("name")
        return {
            "id": obj["id"],
            "description": props.get("device.description") or props.get("device.nick"),
            "profile": active_profile or "unknown",
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
