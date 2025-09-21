"""CLI flows."""

from pw_client import pw_dump, set_default_sink, set_profile
from .util import table, select_option
from pipewire_parsers import parse_card, parse_profiles, parse_sinks

def change_sink() -> None:
    dump = pw_dump()
    sinks = parse_sinks(dump)

    table("Output Sinks", sinks)
    chosen_sink = select_option("Select sink")
    
    set_default_sink(chosen_sink)

def change_card() -> None:
    dump = pw_dump()
    sinks = parse_sinks(dump)
    
    try:
        sink = next(s for s in sinks if s["id"] == chosen_sink)
    except StopIteration as exc:
        raise RuntimeError(f"Selected sink {chosen_sink} not found") from exc

    card_id = sink.get("device.id")

    card = parse_card(dump, card_id)
    if card is None:
        raise RuntimeError(f"No card found for id {card_id}")

    table("Card", [card])

    profiles = parse_profiles(card)
    table(f"Profiles for Card {card_id}", profiles)
    
    chosen_profile = select_option("Select profile")
    
    set_profile(card_id, chosen_profile)
