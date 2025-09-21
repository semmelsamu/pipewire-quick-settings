"""Helpers for user input."""

from typing import Any, Dict, List
from .display import table

def select_option(text: str):
    return int(input(text + " > ").strip())

def choose_sink(sinks: List[Dict[str, Any]]) -> int:
    table("Output Sinks", sinks)
    return select_option("Select sink")

def choose_profile(card_id: int, profiles: List[Dict[str, Any]]) -> int:
    table(f"Profiles for Card {card_id}", profiles)
    return select_option("Select profile")
