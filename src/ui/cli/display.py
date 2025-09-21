"""Helpers for user-facing table output."""
from typing import Any, Dict, List


def table(title: str, rows: List[Dict[str, Any]], cols: List[str]) -> None:
    """Render a simple text table for a list of dictionaries."""
    print(f"\n=== {title} ({len(rows)}) ===")
    if not rows:
        return
    widths = {
        column: max(len(column), max((len(str(row.get(column, ""))) for row in rows), default=0))
        for column in cols
    }
    header = " | ".join(column.ljust(widths[column]) for column in cols)
    print(header)
    print("-" * len(header))
    for row in rows:
        print(" | ".join(str(row.get(column, "")).ljust(widths[column]) for column in cols))
