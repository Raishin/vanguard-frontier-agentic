#!/usr/bin/env python3
"""Reject npm package lifecycle scripts that execute on install.

Lifecycle hooks like `preinstall`, `install`, `postinstall`, `prepare`,
`prepublish`, and `prepublishOnly` run automatically on `npm install`
and `npm publish`. They are the same execution primitive that was
abused in the 2024 xz-utils-style supply-chain incidents, and they are
the only general-purpose code execution surface in an npm package's
metadata.

This package ships only documentation and JSON. There is no legitimate
reason for any install-time script. This validator ensures the
package.json (and any future workspace package.json files) cannot
silently grow one.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

# Hooks that npm runs implicitly during install/publish. This list is the
# union of the npm 10/11 documented lifecycle scripts that execute
# without explicit invocation.
FORBIDDEN_HOOKS = {
    "preinstall",
    "install",
    "postinstall",
    "preuninstall",
    "uninstall",
    "postuninstall",
    "prepare",
    "prepublish",
    "prepublishOnly",
    "prepack",
    "postpack",
}


def package_json_files() -> list[Path]:
    paths = [ROOT / "package.json"]
    # Look for any future workspace packages.
    for candidate in ROOT.rglob("package.json"):
        if "node_modules" in candidate.parts:
            continue
        if candidate not in paths:
            paths.append(candidate)
    return paths


def main() -> int:
    errors: list[str] = []
    files_checked = 0
    for path in package_json_files():
        if not path.is_file():
            continue
        files_checked += 1
        try:
            data = json.loads(path.read_text(encoding="utf-8"))
        except Exception as exc:  # noqa: BLE001
            errors.append(f"{path.relative_to(ROOT)}: invalid JSON: {exc}")
            continue

        scripts = data.get("scripts") or {}
        present = sorted(set(scripts.keys()) & FORBIDDEN_HOOKS)
        if present:
            errors.append(
                f"{path.relative_to(ROOT)}: forbidden lifecycle scripts present: "
                f"{present}. This package must not run code on install/publish."
            )

    if errors:
        print("ERROR: lifecycle-script policy violation", file=sys.stderr)
        for err in errors:
            print(f"  - {err}", file=sys.stderr)
        return 1

    print(f"OK: no forbidden lifecycle scripts in {files_checked} package.json file(s)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
