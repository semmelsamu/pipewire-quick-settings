#!/usr/bin/env python3

import json, subprocess, sys
from typing import Any, Dict, List, Optional

# ------------------------------------------------------------------------------
# Utilities
# ------------------------------------------------------------------------------

def table(title: str, rows: List[Dict[str, Any]], cols: List[str]):
    print(f"\n=== {title} ({len(rows)}) ===")
    if not rows:
        return
    widths = {c: max(len(c), max((len(str(r.get(c, ""))) for r in rows), default=0)) for c in cols}
    header = " | ".join(c.ljust(widths[c]) for c in cols)
    print(header)
    print("-" * len(header))
    for r in rows:
        print(" | ".join(str(r.get(c, "")).ljust(widths[c]) for c in cols))

# ------------------------------------------------------------------------------
# Parsing logic
# ------------------------------------------------------------------------------

def pw_dump():
    proc = subprocess.run(["pw-dump"], capture_output=True, text=True, check=True)
    return json.loads(proc.stdout)
    

def parse_sinks(dump):
    """
    Gibt Sinks (Nodes mit Audio/Sink) zurück.
    """
    sinks = []
    for obj in dump:
        if obj.get("type") == "PipeWire:Interface:Node":
            props = obj.get("info", {}).get("props", {})
            if props.get("media.class", "").startswith("Audio/Sink"):
                sinks.append({
                    "id": obj["id"],
                    "description": props.get("node.description") or props.get("node.name"),
                    "state": obj.get("info", {}).get("state", "unknown"),
                    "device.id": props.get("device.id"),   # ← Verknüpfung zur Card
                })
    return sinks

def parse_card(dump, card_id: int):
    """
    Sucht die Card (Device) zu einer ID.
    """
    for obj in dump:
        if obj.get("id") == card_id and obj.get("type") == "PipeWire:Interface:Device":
            props = obj.get("info", {}).get("props", {})
            params = obj.get("info", {}).get("params", {})
            active_profile = None
            if params.get("Profile"):
                active_profile = params["Profile"][0].get("description") or params["Profile"][0].get("name")
            return {
                "id": obj["id"],
                "description": props.get("device.description") or props.get("device.nick"),
                "profile": active_profile or "unknown",
                "params": params,
            }
    return None

def parse_profiles(card):
    """
    Profile aus einem Card-Objekt extrahieren.
    """
    profiles = []
    for p in card.get("params", {}).get("EnumProfile", []):
        profiles.append({
            "index": p.get("index"),
            "name": p.get("name"),
            "description": p.get("description"),
            "available": p.get("available"),
        })
    return profiles


# ------------------------------------------------------------------------------
# Main Script
# ------------------------------------------------------------------------------

def main():
    dump = pw_dump()

    sinks = parse_sinks(dump)
    table("Output Sinks", sinks, ["id", "description", "state"])
    chosen_sink = int(input("Select Sink > ").strip())
    
    subprocess.run(["wpctl", "set-default", str(chosen_sink)], check=False)

    sink = next(s for s in sinks if s["id"] == chosen_sink)
    card_id = sink.get("device.id")
    
    card = parse_card(dump, card_id)
    profiles = parse_profiles(card)
    
    table("Card", [card], ["id", "description", "profile"])
    
    table(f"Profiles for Card {card_id}", profiles, ["index", "name", "description", "available"])
    chosen_profile = int(input("Select Profile Index > ").strip())
    
    subprocess.run(["wpctl", "set-profile", str(card_id), str(chosen_profile)], check=False)


if __name__ == "__main__":
    main()