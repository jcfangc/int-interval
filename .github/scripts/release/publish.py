"""Publish the crate, or validate publication with cargo publish --dry-run."""

from __future__ import annotations

import os
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]


def is_real_release() -> bool:
    """Pushes to main publish for real; dispatch follows RELEASE_MODE."""
    event = os.environ.get("GITHUB_EVENT_NAME", "")
    mode = os.environ.get("RELEASE_MODE", "dryrun")

    if event == "push":
        return True

    if event == "workflow_dispatch":
        if mode not in {"dryrun", "real"}:
            raise RuntimeError(f"unsupported RELEASE_MODE: {mode}")
        return mode == "real"

    raise RuntimeError(f"unsupported release event: {event}")


def main() -> None:
    command = ["cargo", "publish", "--locked"]

    if is_real_release():
        print("publishing crate to crates.io")
    else:
        print("validating crate publication with --dry-run")
        command.append("--dry-run")

    subprocess.run(command, cwd=ROOT, check=True)


if __name__ == "__main__":
    main()
