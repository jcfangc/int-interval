"""Check that committed Rust sources match xtask code generation output."""

from __future__ import annotations

import subprocess
from pathlib import Path


def main() -> None:
    root = Path(__file__).resolve().parents[3]
    manifest = root / "xtask" / "codegen" / "Cargo.toml"

    subprocess.run(
        [
            "cargo",
            "run",
            "--locked",
            "--quiet",
            "--manifest-path",
            str(manifest),
            "--",
            "check",
        ],
        cwd=root,
        check=True,
    )


if __name__ == "__main__":
    main()
