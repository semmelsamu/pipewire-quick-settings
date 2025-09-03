#!/usr/bin/env python3
"""
Parse live PipeWire `pw-dump` JSON and list:
- Programs that OUTPUT audio (Stream/Output/Audio)
- Playback devices (Audio/Sink)
- Programs that INPUT/capture audio (Stream/Input/Audio)
- Capture devices (Audio/Source)

Usage:
    python pw_parse.py
"""

import json, subprocess, sys
from typing import Any, Dict, List

def get(d: Dict[str, Any], *path: str, default=None):
    cur = d
    for p in path:
        if not isinstance(cur, dict) or p not in cur:
            return default
        cur = cur[p]
    return cur

def collect(items: List[Dict[str, Any]]):
    nodes = [o for o in items if o.get("type") == "PipeWire:Interface:Node"]
    clients = {o.get("id"): o for o in items if o.get("type") == "PipeWire:Interface:Client"}

    def node_row(n: Dict[str, Any]) -> Dict[str, Any]:
        props = get(n, "info", "props", default={}) or {}
        client_id = props.get("client.id")
        client = clients.get(client_id, {})
        cprops = get(client, "info", "props", default={}) or {}
        return {
            "node.id": n.get("id"),
            "media.class": props.get("media.class"),
            "node.name": props.get("node.name"),
            "node.description": props.get("node.description") or props.get("node.nick"),
            "application.name": props.get("application.name") or cprops.get("application.name"),
            "application.process.binary": props.get("application.process.binary") or cprops.get("application.process.binary"),
            "client.id": client_id,
            "state": get(n, "info", "state"),
        }

    stream_out = []
    stream_in = []
    sinks = []
    sources = []

    for n in nodes:
        media_class = get(n, "info", "props", "media.class")
        if not isinstance(media_class, str):
            continue
        row = node_row(n)
        if media_class.startswith("Stream/Output/Audio"):
            stream_out.append(row)
        if media_class.startswith("Stream/Input/Audio"):
            stream_in.append(row)
        if media_class.startswith("Audio/Sink"):
            sinks.append(row)
        if media_class.startswith("Audio/Source"):
            sources.append(row)

    return stream_out, sinks, stream_in, sources

def main():
    try:
        proc = subprocess.run(["pw-dump"], capture_output=True, text=True, check=True)
    except FileNotFoundError:
        print("Error: pw-dump not found. Please install PipeWire tools.")
        sys.exit(1)
    except subprocess.CalledProcessError as e:
        print("Error running pw-dump:", e)
        sys.exit(1)

    try:
        items = json.loads(proc.stdout)
    except json.JSONDecodeError as e:
        print("Error parsing pw-dump output:", e)
        sys.exit(1)

    stream_out, sinks, stream_in, sources = collect(items)

    def print_section(title: str, rows: List[Dict[str, Any]], cols: List[str]):
        print(f"\n=== {title} ({len(rows)}) ===")
        if not rows:
            return
        widths = {c: max(len(c), max((len(str(r.get(c, ""))) for r in rows), default=0)) for c in cols}
        header = " | ".join(c.ljust(widths[c]) for c in cols)
        print(header)
        print("-" * len(header))
        for r in rows:
            print(" | ".join(str(r.get(c, "")).ljust(widths[c]) for c in cols))

    cols_stream = ["node.id", "application.name", "application.process.binary", "node.name", "node.description", "media.class", "client.id", "state"]
    cols_devnode = ["node.id", "node.name", "node.description", "media.class", "state"]

    print_section("Programs outputting audio (Stream/Output/Audio)", stream_out, cols_stream)
    print_section("Playback devices (Audio/Sink nodes)", sinks, cols_devnode)
    print_section("Programs capturing audio (Stream/Input/Audio)", stream_in, cols_stream)
    print_section("Capture devices (Audio/Source nodes)", sources, cols_devnode)

if __name__ == "__main__":
    main()