"""CLI flows."""

from pw_client import pw_dump, set_default_sink, set_profile, set_volume, set_mute
from .util import table, select_option
from pipewire_parsers import (
    get_current_profile,
    get_current_sink,
    parse_card,
    parse_profiles,
    parse_sinks,
)


def change_sink() -> None:
    dump = pw_dump()
    sinks = parse_sinks(dump)
    current_sink = get_current_sink(dump)

    if current_sink:
        print(
            f"Current default sink: {current_sink.get('description')} (id {current_sink.get('id')})"
        )
    else:
        print("Current default sink: unknown")

    table("Output Sinks", sinks)
    chosen_sink = select_option("Select sink")

    set_default_sink(chosen_sink)


def change_profile() -> None:
    dump = pw_dump()
    sinks = parse_sinks(dump)
    current_sink = get_current_sink(dump)

    if current_sink:
        print(
            f"Current default sink: {current_sink.get('description')} (id {current_sink.get('id')})"
        )
    else:
        print("Current default sink: unknown")

    table("Output Sinks", sinks)
    chosen_sink = select_option("Select sink")
    
    try:
        sink = next(s for s in sinks if s["id"] == chosen_sink)
    except StopIteration as exc:
        raise RuntimeError(f"Selected sink {chosen_sink} not found") from exc

    card_id = sink.get("device.id")

    card = parse_card(dump, card_id)
    
    if card is None:
        raise RuntimeError(f"No card found for id {card_id}")

    profiles = parse_profiles(card)
    current_profile = get_current_profile(card)

    if current_profile:
        description = current_profile.get("description") or current_profile.get("name")
        print(
            f"Current profile: {description} (index {current_profile.get('index')})"
        )
    else:
        print(f"Current profile: {card.get('profile')}")
    
    table(f"Profiles for Card {card_id}", profiles)
    chosen_profile = select_option("Select profile")

    set_profile(card_id, chosen_profile)


def change_volume() -> None:
    dump = pw_dump()
    sinks = parse_sinks(dump)
    current_sink = get_current_sink(dump)

    if current_sink:
        print(
            f"Current default sink: {current_sink.get('description')} (id {current_sink.get('id')})"
        )
    else:
        print("Current default sink: unknown")

    table("Output Sinks", sinks)
    chosen_sink = select_option("Select sink")

    try:
        sink = next(s for s in sinks if s["id"] == chosen_sink)
    except StopIteration as exc:
        raise RuntimeError(f"Selected sink {chosen_sink} not found") from exc

    volume = sink.get("volume")
    volume_linear = sink.get("volume_linear")
    if isinstance(volume, (int, float)):
        print(f"Current volume: {float(volume):.2f} (~{float(volume) * 100:.0f}%)")
        if isinstance(volume_linear, (int, float)):
            print(f"Raw PipeWire volume: {float(volume_linear):.3f}")
    elif volume is not None:
        print(f"Current volume: {volume}")
    else:
        print("Current volume: unknown")

    mute_state = sink.get("mute")
    if mute_state is not None:
        print(f"Muted: {'yes' if mute_state else 'no'}")

    raw_volume = input("Enter volume (e.g. 0.5 or 50%) > ").strip()

    if not raw_volume:
        print("Volume unchanged: no value entered")
        return

    volume_arg = raw_volume
    if raw_volume.endswith("%"):
        try:
            float(raw_volume[:-1])
        except ValueError as exc:
            raise RuntimeError(f"Invalid percentage volume '{raw_volume}'") from exc
    else:
        try:
            volume_value = float(raw_volume)
        except ValueError as exc:
            raise RuntimeError(f"Invalid volume '{raw_volume}'") from exc
        if volume_value < 0:
            raise RuntimeError("Volume must be non-negative")
        volume_arg = f"{volume_value}"

    set_volume(chosen_sink, volume_arg)


def change_mute() -> None:
    dump = pw_dump()
    sinks = parse_sinks(dump)
    current_sink = get_current_sink(dump)

    if current_sink:
        print(
            f"Current default sink: {current_sink.get('description')} (id {current_sink.get('id')})"
        )
    else:
        print("Current default sink: unknown")

    table("Output Sinks", sinks)
    chosen_sink = select_option("Select sink")

    try:
        sink = next(s for s in sinks if s["id"] == chosen_sink)
    except StopIteration as exc:
        raise RuntimeError(f"Selected sink {chosen_sink} not found") from exc

    mute_state = sink.get("mute")
    if mute_state is None:
        print("Current mute state: unknown")
    else:
        print(f"Current mute state: {'muted' if mute_state else 'unmuted'}")

    choice = input("Enter mute state (mute/unmute/toggle) > ").strip().lower()

    if not choice:
        print("Mute state unchanged: no value entered")
        return

    if choice in {"mute", "m", "on", "yes", "1"}:
        set_mute(chosen_sink, True)
    elif choice in {"unmute", "u", "off", "no", "0"}:
        set_mute(chosen_sink, False)
    elif choice in {"toggle", "t"}:
        set_mute(chosen_sink, "toggle")
    else:
        raise RuntimeError(f"Unrecognized mute option '{choice}'")
