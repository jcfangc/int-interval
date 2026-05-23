"""Create and push release tags for real or optional dry-run releases."""

from __future__ import annotations

import os
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]


def run_git(*args: str, check: bool = True) -> subprocess.CompletedProcess[str]:
    """Run git in the repository root."""
    return subprocess.run(
        ["git", *args],
        cwd=ROOT,
        check=check,
        text=True,
        capture_output=True,
    )


def release_tag() -> str | None:
    """Return the tag to create, or None when dry-run tagging is disabled."""
    version = os.environ["RELEASE_VERSION"]
    event = os.environ.get("GITHUB_EVENT_NAME", "")
    mode = os.environ.get("RELEASE_MODE", "dryrun")
    tag_dryrun = os.environ.get("TAG_DRYRUN", "false")

    if event == "push":
        return f"v{version}"

    if event != "workflow_dispatch":
        raise RuntimeError(f"unsupported release event: {event}")

    if mode == "real":
        return f"v{version}"

    if mode != "dryrun":
        raise RuntimeError(f"unsupported RELEASE_MODE: {mode}")

    if tag_dryrun == "true":
        run_id = os.environ["GITHUB_RUN_ID"]
        return f"v{version}-dryrun.{run_id}"

    return None


def main() -> None:
    tag = release_tag()
    if tag is None:
        print("skipping tag creation for dry-run release")
        return

    run_git("fetch", "--tags", "origin")

    exists = (
        run_git(
            "rev-parse",
            "--verify",
            "--quiet",
            f"refs/tags/{tag}",
            check=False,
        ).returncode
        == 0
    )

    if exists:
        raise RuntimeError(f"tag already exists: {tag}")

    run_git("config", "user.name", "github-actions[bot]")
    run_git("config", "user.email", "github-actions[bot]@users.noreply.github.com")
    run_git("tag", "-a", tag, "-m", f"Release {tag}")
    run_git("push", "origin", f"refs/tags/{tag}")

    print(f"created and pushed tag: {tag}")


if __name__ == "__main__":
    main()
