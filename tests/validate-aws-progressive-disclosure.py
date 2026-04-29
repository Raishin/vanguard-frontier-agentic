#!/usr/bin/env python3
"""Validate AWS skills use progressive disclosure via references/."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
AWS_DIR = ROOT / "skills" / "aws"
CATALOG = ROOT / "catalog" / "skills.json"
REQUIRED_REFERENCES = {
    "references/workflow-and-output.md",
    "references/official-sources.md",
    "references/safety-checklist.md",
}
AGENTCORE_EXTRA_TERMS = [
    "runtime",
    "Memory",
    "Gateway",
    "Identity",
    "Observability",
    "Browser",
    "Code Interpreter",
    "MCP",
]


def err(msg: str, errors: list[str]) -> None:
    errors.append(msg)


def main() -> int:
    errors: list[str] = []
    catalog = {item["id"]: item for item in json.loads(CATALOG.read_text()) if item.get("provider") == "aws"}
    for skill_dir in sorted(p for p in AWS_DIR.iterdir() if p.is_dir()):
        sid = skill_dir.name
        skill = skill_dir / "SKILL.md"
        text = skill.read_text(encoding="utf-8")
        if sid not in catalog:
            err(f"{sid}: missing from catalog/skills.json", errors)
        for rel in REQUIRED_REFERENCES:
            ref = skill_dir / rel
            if not ref.exists():
                err(f"{sid}: missing {rel}", errors)
            elif len(ref.read_text(encoding="utf-8").strip()) < 200:
                err(f"{sid}: {rel} is too thin for useful lazy loading", errors)
            if f"]({rel})" not in text:
                err(f"{sid}: SKILL.md does not link {rel}", errors)
        if len(text.splitlines()) > 90:
            err(f"{sid}: SKILL.md has {len(text.splitlines())} lines; too heavy for trigger-time loading", errors)
        if not re.search(r"Load these only when needed", text, re.IGNORECASE):
            err(f"{sid}: missing lazy-load instruction", errors)
        if sid == "aws-agentcore":
            joined = "\n".join((skill_dir / rel).read_text(encoding="utf-8") for rel in REQUIRED_REFERENCES if (skill_dir / rel).exists()) + text
            for term in AGENTCORE_EXTRA_TERMS:
                if term.lower() not in joined.lower():
                    err(f"{sid}: AgentCore references missing {term}", errors)
            if "Do not hardcode credentials" not in joined and "Do not ask for" not in joined:
                err(f"{sid}: AgentCore references missing credential safety language", errors)
    if errors:
        for item in errors:
            print(f"ERROR: {item}", file=sys.stderr)
        return 1
    print(f"OK: validated progressive references for {len([p for p in AWS_DIR.iterdir() if p.is_dir()])} AWS skills")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
