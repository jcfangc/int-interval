#!/usr/bin/env python3
"""Rewrite Divan bench names from op/impl/case to op/case/impl."""

from __future__ import annotations

import re
import sys
from pathlib import Path

IMPLS = {
    "int_interval",
    "rust_intervals",
    "std_range",
    "bit_string",
    "bitvec_simd",
}

BENCH_NAME = re.compile(r'(#\[divan::bench\(name\s*=\s*")([^"]+)("\)\])')


def rewrite_name(name: str) -> str:
    parts = name.split("/")
    if len(parts) < 3:
        return name

    op, impl, *case = parts
    if impl not in IMPLS:
        return name

    return "/".join([op, *case, impl])


def rewrite_file(path: Path) -> bool:
    old = path.read_text(encoding="utf-8")

    def repl(match: re.Match[str]) -> str:
        return f"{match.group(1)}{rewrite_name(match.group(2))}{match.group(3)}"

    new = BENCH_NAME.sub(repl, old)
    if new == old:
        return False

    path.write_text(new, encoding="utf-8")
    return True


def main() -> None:
    paths = [Path(arg) for arg in sys.argv[1:]] or list(Path("benches").glob("*.rs"))

    changed = [path for path in paths if path.is_file() and rewrite_file(path)]

    for path in changed:
        print(path)


if __name__ == "__main__":
    main()
