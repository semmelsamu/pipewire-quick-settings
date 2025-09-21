"""Helpers for user input."""

from typing import Any, Dict, List
from .display import table

def choose_sink(sinks: List[Dict[str, Any]]) -> int:
    table("Output Sinks", sinks, ["id", "description", "state"])
    return int(input("Select Sink > ").strip())


def choose_profile(card_id: int, profiles: List[Dict[str, Any]]) -> int:
    table(f"Profiles for Card {card_id}", profiles, ["index", "name", "description", "available"])
    return int(input("Select Profile Index > ").strip())