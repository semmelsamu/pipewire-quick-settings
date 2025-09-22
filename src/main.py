#!/usr/bin/env python3
"""Application entry point for PipeWire quick settings."""
from __future__ import annotations

import argparse
from typing import Sequence

from cli import cli_loop
from gui import run_gui


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="PipeWire quick settings")
    parser.add_argument(
        "--mode",
        choices=("cli", "gui"),
        default="cli",
        help="Start in CLI or GUI mode",
    )
    return parser


def main(argv: Sequence[str] | None = None) -> None:
    parser = build_parser()
    args = parser.parse_args(argv)

    if args.mode == "cli":
        cli_loop()
    else:
        run_gui()


if __name__ == "__main__":
    main()
