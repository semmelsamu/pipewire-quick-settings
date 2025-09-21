"""Command-line interface for PipeWire quick settings."""
from __future__ import annotations

import builtins
from typing import Any, Dict, List

from .display import table
from .cli import change_sink, change_card
from .input import select_option

def cli():
    options = [
        { "id": 0, "description": "Exit" },
        { "id": 1, "description": "Set default sink" },
        { "id": 2, "description": "Set card for current sink" },
    ]
    
    table("Pipewire Quick Settings CLI", options)
    
    option = select_option("Select option")
    
    match option:
        case 0:
            return False
        
        case 1: 
            change_sink()
            
        case 2:
            change_card()
        
        case _:
            print("Invalid option")
            
    return True
    

def cli_loop() -> None:
    while cli():
        continue