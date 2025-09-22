"""Helpers for user I/O."""
from typing import Any, Dict, List


def select_option(text: str):
    return int(input(text + " > ").strip())


def table(title: str, rows: List[Dict[str, Any]]) -> None:
    """Render a table covering all keys present in the provided rows."""
    print(f"\n==== {title}  ====")
    if not rows:
        return

    columns: List[str] = []
    for row in rows:
        for key in row.keys():
            if key not in columns:
                columns.append(key)

    widths = {
        column: max(
            len(column),
            max((len(str(row.get(column, ""))) for row in rows), default=0),
        )
        for column in columns
    }
    header = " | ".join(column.ljust(widths[column]) for column in columns)
    print(header)
    print("-" * len(header))
    for row in rows:
        print(" | ".join(str(row.get(column, "")).ljust(widths[column]) for column in columns))
    print()