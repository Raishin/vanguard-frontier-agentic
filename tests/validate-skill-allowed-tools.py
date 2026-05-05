#!/usr/bin/env python3
"""Validate that every SKILL.md declares an allowed-tools frontmatter field.

The `allowed-tools` field aligns each skill with the Claude Code skills spec
(https://code.claude.com/docs/en/skills) and makes the tool surface explicit.
It is a pre-approval list (not a deny-list); harness deny rules in
settings.json are still the enforcement boundary, but declaring the field
here makes intent reviewable.

Cross-platform note: SKILL.md is a Claude Code artifact in this repo
(skills/<provider>/<name>/SKILL.md). Other harnesses do not consume SKILL.md
frontmatter, so this field is harmless for non-Claude exports.

Validation rules:
  1. Every SKILL.md must contain an `allowed-tools` key in YAML frontmatter.
  2. The value must be either a non-empty space-separated string or a
     non-empty YAML list of strings.
  3. Each token must match the recognised tool grammar:
       Bare tool name:   ^[A-Z][A-Za-z0-9]+$         (Read, Edit, Bash)
       Constrained tool: ^[A-Z][A-Za-z0-9]+\(.+\)$    (Bash(git add *))
       Skill/Agent invocation tokens are also allowed.
  4. At least one token must be present.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SKILLS_DIR = ROOT / "skills"

TOKEN_RE = re.compile(r"^[A-Z][A-Za-z0-9]+(\([^)]+\))?$")


def parse_frontmatter(text: str) -> dict[str, str] | None:
    if not text.startswith("---\n"):
        return None
    end = text.find("\n---", 4)
    if end == -1:
        return None
    block = text[4:end]
    fm: dict[str, str] = {}
    current_key: str | None = None
    current_lines: list[str] = []
    for line in block.splitlines():
        if not line.startswith(" ") and ":" in line:
            if current_key is not None:
                fm[current_key] = "\n".join(current_lines).strip()
            key, _, rest = line.partition(":")
            current_key = key.strip()
            current_lines = [rest.strip()]
        else:
            current_lines.append(line)
    if current_key is not None:
        fm[current_key] = "\n".join(current_lines).strip()
    return fm


def tokenize_allowed_tools(value: str) -> list[str]:
    """Split a space-separated allowed-tools value, respecting parentheses."""
    tokens: list[str] = []
    buf: list[str] = []
    depth = 0
    for ch in value:
        if ch == "(":
            depth += 1
            buf.append(ch)
        elif ch == ")":
            depth -= 1
            buf.append(ch)
        elif ch.isspace() and depth == 0:
            if buf:
                tokens.append("".join(buf))
                buf = []
        else:
            buf.append(ch)
    if buf:
        tokens.append("".join(buf))
    return tokens


def validate_skill(skill_md: Path) -> list[str]:
    text = skill_md.read_text(encoding="utf-8")
    fm = parse_frontmatter(text)
    if fm is None:
        return [f"{skill_md}: no YAML frontmatter found"]

    if "allowed-tools" not in fm:
        return [f"{skill_md}: missing required 'allowed-tools' frontmatter field"]

    raw = fm["allowed-tools"].strip()
    if raw.startswith("[") and raw.endswith("]"):
        inner = raw[1:-1].strip()
        tokens = [t.strip().strip("'\"") for t in inner.split(",") if t.strip()]
    else:
        tokens = tokenize_allowed_tools(raw)

    errors: list[str] = []
    if not tokens:
        errors.append(f"{skill_md}: 'allowed-tools' is empty")
        return errors

    for tok in tokens:
        if not TOKEN_RE.match(tok):
            errors.append(
                f"{skill_md}: invalid allowed-tools token '{tok}' "
                f"(expected ToolName or ToolName(constraint))"
            )
    return errors


def main() -> int:
    skill_files = sorted(SKILLS_DIR.glob("*/*/SKILL.md"))
    if not skill_files:
        print("ERROR: no SKILL.md files found", file=sys.stderr)
        return 2

    all_errors: list[str] = []
    for skill_md in skill_files:
        all_errors.extend(validate_skill(skill_md))

    if all_errors:
        print(f"FAIL: {len(all_errors)} allowed-tools issue(s) across "
              f"{len(skill_files)} skill(s):", file=sys.stderr)
        for err in all_errors[:20]:
            print(f"  - {err}", file=sys.stderr)
        if len(all_errors) > 20:
            print(f"  ... and {len(all_errors) - 20} more", file=sys.stderr)
        return 1

    print(f"OK: validated allowed-tools on {len(skill_files)} skills")
    return 0


if __name__ == "__main__":
    sys.exit(main())
