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
    Return list of audio sink nodes with id, description, and state.
    """
    sinks = []
    for obj in dump:
        if obj.get("type") == "PipeWire:Interface:Node":
            props = obj.get("info", {}).get("props", {})
            media_class = props.get("media.class", "")
            if media_class.startswith("Audio/Sink"):
                sinks.append({
                    "id": obj["id"],
                    "description": props.get("node.description") or props.get("node.name"),
                    "state": obj.get("info", {}).get("state", "unknown")
                })
    return sinks

def parse_ports(dump, sink_id):
    """
    Return list of output ports for a given sink id.
    """
    ports = []
    for obj in dump:
        if obj.get("type") == "PipeWire:Interface:Port":
            info = obj.get("info", {})
            props = info.get("props", {})
            if props.get("node.id") == sink_id and info.get("direction") == "output":
                ports.append({
                    "id": obj["id"],
                    "name": props.get("port.name"),
                    "alias": props.get("port.alias"),
                    "group": props.get("port.group"),
                })
    return ports

# ------------------------------------------------------------------------------
# Main Script
# ------------------------------------------------------------------------------

def main():
    dump = pw_dump()
    
    sinks = parse_sinks(dump)
    
    table("Output Sinks", sinks, ["id", "description", "state"])
    
    return

    print_section("Playback devices (Audio/Sink nodes)", sinks, ["node.id", "node.description", "state"])
    
    new_output = int(input("Set output: >"))
    
    show_sink_ports(find_sink(sinks, new_output))
    
    new_port = int(input("\nSet port index: > "))
    
    subprocess.run(f"wpctl set-default {new_output}", shell=True)
    subprocess.run(f"wpctl set-profile {new_output} {new_port}", shell=True)

if __name__ == "__main__":
    main()