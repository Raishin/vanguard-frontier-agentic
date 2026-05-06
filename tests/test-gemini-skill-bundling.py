#!/usr/bin/env python3
"""TDD test: Gemini CLI skill bundling via export-marketplace-agents.mjs.

Asserts that --platform gemini bundles companion skills into .gemini/skills/<skill>/SKILL.md.
Agent under test: azure-cosmosdb-platform-operator-agent (companion: azure-cosmosdb-platform-operator).

Run: python3 tests/test-gemini-skill-bundling.py
Exit 0 = all assertions passed.
Exit 1 = one or more assertions failed.
"""

from __future__ import annotations

import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SCRIPT = ROOT / "scripts" / "export-marketplace-agents.mjs"

AGENT_ID = "azure-cosmosdb-platform-operator-agent"
SKILL_NAME = "azure-cosmosdb-platform-operator"
EXPECTED_SKILL_REL = f".gemini/skills/{SKILL_NAME}/SKILL.md"

PASS = 0
FAIL = 0


def pass_(msg: str) -> None:
    global PASS
    print(f"  PASS: {msg}")
    PASS += 1


def fail(msg: str) -> None:
    global FAIL
    print(f"  FAIL: {msg}")
    FAIL += 1


def run_export(tmpdir: Path) -> subprocess.CompletedProcess:
    return subprocess.run(
        [
            "node",
            str(SCRIPT),
            "--platform", "gemini",
            "--agents", AGENT_ID,
            "--repo", str(tmpdir),
            "--force",
        ],
        capture_output=True,
        text=True,
        cwd=str(ROOT),
    )


def test_skill_bundled_to_gemini_path() -> None:
    print(f"\n[test] gemini skill bundling → {EXPECTED_SKILL_REL}")
    with tempfile.TemporaryDirectory() as tmpdir:
        result = run_export(Path(tmpdir))

        if result.returncode != 0:
            fail(f"exporter exited {result.returncode}: {result.stderr.strip()}")
            return

        skill_md = Path(tmpdir) / EXPECTED_SKILL_REL
        if skill_md.is_file():
            pass_(f"SKILL.md present at {EXPECTED_SKILL_REL}")
        else:
            fail(f"SKILL.md NOT found at {EXPECTED_SKILL_REL}")
            print(f"    stdout: {result.stdout.strip()}")
            print(f"    stderr: {result.stderr.strip()}")

        # Also confirm the "not yet supported" notice is NOT emitted
        if "not yet supported" in result.stderr:
            fail("stderr still contains 'not yet supported' notice for gemini platform")
        else:
            pass_("no 'not yet supported' notice in stderr")

        # Confirm bundled count in stderr
        if "Bundled" in result.stderr and "companion skill" in result.stderr:
            pass_("bundled summary line present in stderr")
        else:
            fail(f"bundled summary line missing from stderr: {result.stderr.strip()!r}")


def main() -> None:
    print(f"=== Gemini CLI skill bundling test ===")
    print(f"Script : {SCRIPT}")
    print(f"Agent  : {AGENT_ID}")
    print(f"Expect : {EXPECTED_SKILL_REL}")

    test_skill_bundled_to_gemini_path()

    print(f"\nResults: {PASS} passed, {FAIL} failed")
    if FAIL:
        sys.exit(1)


if __name__ == "__main__":
    main()
