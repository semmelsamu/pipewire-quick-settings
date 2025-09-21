"""Interactions with PipeWire command-line tools."""
from __future__ import annotations

import json
import subprocess
from typing import Any, Dict, List


PW_DUMP_CMD = ["pw-dump"]
WPCTL_CMD = "wpctl"


def pw_dump() -> List[Dict[str, Any]]:
    """Return the JSON structure produced by ``pw-dump``."""
    process = subprocess.run(PW_DUMP_CMD, capture_output=True, text=True, check=True)
    return json.loads(process.stdout)


def set_default_sink(sink_id: int) -> None:
    """Set the default PipeWire sink via ``wpctl``."""
    subprocess.run([WPCTL_CMD, "set-default", str(sink_id)], check=False)


def set_profile(card_id: int, profile_index: int) -> None:
    """Set the profile for a specific card via ``wpctl``."""
    subprocess.run([WPCTL_CMD, "set-profile", str(card_id), str(profile_index)], check=False)
