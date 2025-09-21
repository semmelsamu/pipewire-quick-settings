"""Command-line interface for PipeWire quick settings."""
from __future__ import annotations

from typing import Any, Dict, List

from .display import table
from pipewire_parsers import parse_card, parse_profiles, parse_sinks
from pw_client import pw_dump, set_default_sink, set_profile


def choose_sink(sinks: List[Dict[str, Any]]) -> int:
    table("Output Sinks", sinks, ["id", "description", "state"])
    return int(input("Select Sink > ").strip())


def choose_profile(card_id: int, profiles: List[Dict[str, Any]]) -> int:
    table(f"Profiles for Card {card_id}", profiles, ["index", "name", "description", "available"])
    return int(input("Select Profile Index > ").strip())


def run_cli() -> None:
    dump = pw_dump()

    sinks = parse_sinks(dump)
    chosen_sink = choose_sink(sinks)
    set_default_sink(chosen_sink)

    try:
        sink = next(s for s in sinks if s["id"] == chosen_sink)
    except StopIteration as exc:
        raise RuntimeError(f"Selected sink {chosen_sink} not found") from exc

    card_id = sink.get("device.id")

    card = parse_card(dump, card_id)
    if card is None:
        raise RuntimeError(f"No card found for id {card_id}")

    table("Card", [card], ["id", "description", "profile"])

    profiles = parse_profiles(card)
    chosen_profile = choose_profile(card_id, profiles)
    set_profile(card_id, chosen_profile)
