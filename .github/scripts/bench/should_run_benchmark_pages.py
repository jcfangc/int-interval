#!/usr/bin/env python3
"""Decide whether benchmark Pages should run.

Rules:
- workflow_dispatch always runs.
- push runs only when root Cargo.toml package.version changes.
- missing previous commit runs conservatively.
"""

from __future__ import annotations

import os
import subprocess
import sys
import tomllib


ZERO_SHA = "0" * 40


def cargo_package_version(rev: str) -> str:
    try:
        raw = subprocess.check_output(
            ["git", "show", f"{rev}:Cargo.toml"],
            stderr=subprocess.DEVNULL,
        )
    except subprocess.CalledProcessError:
        return ""

    data = tomllib.loads(raw.decode("utf-8"))
    return str(data.get("package", {}).get("version", "")).strip()


def write_output(should_run: bool) -> None:
    output = os.environ.get("GITHUB_OUTPUT")
    line = f"should_run={'true' if should_run else 'false'}\n"

    if output:
        with open(output, "a", encoding="utf-8") as f:
            f.write(line)
    else:
        sys.stdout.write(line)


def main() -> None:
    event_name = os.environ.get("GITHUB_EVENT_NAME", "")
    before = os.environ.get("GITHUB_EVENT_BEFORE", "")
    after = os.environ.get("GITHUB_SHA", "")

    if event_name == "workflow_dispatch":
        print("manual dispatch: run benchmark pages")
        write_output(True)
        return

    if not before or before == ZERO_SHA:
        print("missing previous commit: run benchmark pages")
        write_output(True)
        return

    old = cargo_package_version(before)
    new = cargo_package_version(after)
    changed = bool(new) and old != new

    print(f"old package.version: {old or '<missing>'}")
    print(f"new package.version: {new or '<missing>'}")
    print(f"version changed: {changed}")

    write_output(changed)


if __name__ == "__main__":
    main()
