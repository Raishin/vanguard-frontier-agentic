#!/usr/bin/env python3
"""Test: --platform copilot bundles companion skills into .github/skills/.

TDD RED/GREEN test for GitHub Copilot skill bundling in the exporter.

Run:
    python3 tests/test-copilot-skill-bundling.py
"""

from __future__ import annotations

import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EXPORTER = ROOT / "scripts" / "export-marketplace-agents.mjs"

# aws-agentcore-agent has a companion skill (aws-agentcore) via name-stripping
AGENT_ID = "aws-agentcore-agent"
SKILL_ID = "aws-agentcore"


def run_export(repo: Path, platform: str, agents: str) -> subprocess.CompletedProcess:
    return subprocess.run(
        [
            "node",
            str(EXPORTER),
            "--platform",
            platform,
            "--agents",
            agents,
            "--repo",
            str(repo),
        ],
        capture_output=True,
        text=True,
        cwd=str(ROOT),
    )


def test_copilot_skill_bundled_to_github_skills() -> None:
    """Companion skill must land in .github/skills/<skill-id>/SKILL.md."""
    with tempfile.TemporaryDirectory() as tmpdir:
        repo = Path(tmpdir)
        result = run_export(repo, "copilot", AGENT_ID)

        if result.returncode != 0:
            print("STDOUT:", result.stdout)
            print("STDERR:", result.stderr)
            raise AssertionError(
                f"Exporter exited with code {result.returncode}"
            )

        skill_md = repo / ".github" / "skills" / SKILL_ID / "SKILL.md"
        if not skill_md.exists():
            print("STDOUT:", result.stdout)
            print("STDERR:", result.stderr)
            raise AssertionError(
                f"Expected skill file not found: {skill_md}\n"
                f"Directory tree under .github/skills: "
                + str(list((repo / ".github" / "skills").rglob("*")) if (repo / ".github" / "skills").exists() else [])
            )

        # Also verify the agent itself was installed to .github/agents/
        agent_files = list((repo / ".github" / "agents").rglob("*.agent.md")) if (repo / ".github" / "agents").exists() else []
        if not agent_files:
            raise AssertionError(
                f"Expected agent file in .github/agents/ but found none"
            )

        print(f"PASS: skill bundled to {skill_md.relative_to(repo)}")
        print(f"PASS: agent installed to {agent_files[0].relative_to(repo)}")


def test_copilot_no_skills_flag_suppresses_bundling() -> None:
    """--no-skills must suppress companion skill bundling."""
    with tempfile.TemporaryDirectory() as tmpdir:
        repo = Path(tmpdir)
        result = subprocess.run(
            [
                "node",
                str(EXPORTER),
                "--platform",
                "copilot",
                "--agents",
                AGENT_ID,
                "--repo",
                str(repo),
                "--no-skills",
            ],
            capture_output=True,
            text=True,
            cwd=str(ROOT),
        )

        if result.returncode != 0:
            raise AssertionError(
                f"Exporter exited with code {result.returncode}\n"
                f"STDERR: {result.stderr}"
            )

        skill_md = repo / ".github" / "skills" / SKILL_ID / "SKILL.md"
        if skill_md.exists():
            raise AssertionError(
                f"Skill file should NOT exist with --no-skills: {skill_md}"
            )

        print("PASS: --no-skills suppresses skill bundling for copilot platform")


if __name__ == "__main__":
    failures = []
    for test_fn in [
        test_copilot_skill_bundled_to_github_skills,
        test_copilot_no_skills_flag_suppresses_bundling,
    ]:
        try:
            test_fn()
        except AssertionError as exc:
            failures.append(f"FAIL [{test_fn.__name__}]: {exc}")
            print(f"FAIL [{test_fn.__name__}]: {exc}")

    if failures:
        sys.exit(1)
    else:
        print("All tests passed.")
        sys.exit(0)
