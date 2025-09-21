"""Command-line interface for PipeWire quick settings."""
from __future__ import annotations

import builtins
from typing import Any, Dict, List

from .util import table, select_option
from .cli import change_sink, change_profile


def cli():
    options = [
        { "id": 0, "description": "Exit" },
        { "id": 1, "description": "Set default sink" },
        { "id": 2, "description": "Change sink profile" },
    ]
    
    table("Pipewire Quick Settings CLI", options)
    
    option = select_option("Select option")
    
    match option:
        case 0:
            return False
        
        case 1: 
            change_sink()
            
        case 2:
            change_profile()
        
        case _:
            print("Invalid option")
            
    return True
    

def cli_loop() -> None:
    while cli():
        continue
