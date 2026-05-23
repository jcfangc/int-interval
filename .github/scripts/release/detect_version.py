"""Detect whether the root crate version changed in the current commit."""

from __future__ import annotations

import os
import subprocess
import tomllib
from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
MANIFEST = ROOT / "Cargo.toml"


def run_git(*args: str) -> str:
    """Run git in the repository root and return stdout."""
    return subprocess.run(
        ["git", *args],
        cwd=ROOT,
        check=True,
        capture_output=True,
        text=True,
    ).stdout


def read_version(content: bytes) -> str:
    """Read package.version from Cargo.toml bytes."""
    manifest = tomllib.loads(content.decode())
    try:
        return manifest["package"]["version"]
    except KeyError as err:
        raise RuntimeError("missing [package].version in Cargo.toml") from err


def current_version() -> str:
    return read_version(MANIFEST.read_bytes())


def previous_version() -> str:
    exists = (
        subprocess.run(
            ["git", "rev-parse", "--verify", "--quiet", "HEAD^1"],
            cwd=ROOT,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        ).returncode
        == 0
    )

    if not exists:
        return ""

    content = run_git("show", "HEAD^1:Cargo.toml").encode()
    return read_version(content)


def write_output(name: str, value: str) -> None:
    output = os.environ.get("GITHUB_OUTPUT")
    if output is None:
        print(f"{name}={value}")
        return

    with Path(output).open("a", encoding="utf-8") as file:
        file.write(f"{name}={value}\n")


def main() -> None:
    current = current_version()
    previous = previous_version()
    changed = bool(previous) and current != previous

    write_output("version", current)
    write_output("prev_version", previous)
    write_output("changed", str(changed).lower())

    print(f"current version: {current}")
    print(f"previous version: {previous or '<none>'}")
    print(f"version changed: {str(changed).lower()}")


if __name__ == "__main__":
    main()
