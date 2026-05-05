#!/usr/bin/env python3
"""Apply least-privilege `allowed-tools` to every SKILL.md frontmatter.

Taxonomy (matched against skill id, first match wins):
  maestro                  -> Agent Skill Read Grep Glob
  *-patch-executor,
  *-fix-operator,
  *-corrector,
  *-deployment-hotfix-*,
  *-remediation-operator,
  *-rollout-corrector,
  *-pipeline-fix-operator  -> Read Edit Write MultiEdit Grep Glob
  *-developer,
  *-agentcore,
  aws-generative-ai-developer,
  *-application-developer  -> Read Edit Write MultiEdit Grep Glob Bash
  *-live-*-guard,
  velero-backup-restore-guard
                           -> Read Grep Glob WebFetch
  *-investigator,
  *-responder,
  *-advisor,
  *-coordinator,
  *-watch-coordinator,
  *-analyst,
  *-planner,
  finops-cloud-price-advisor
                           -> Read Grep Glob WebFetch
  default (review/governor/
  auditor/mapper/architect/
  reviewer/steward/selector
  /skill-designer)         -> Read Grep Glob

Skip if `allowed-tools` is already declared (idempotent).
"""

from __future__ import annotations

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SKILLS_DIR = ROOT / "skills"

PATCHERS = (
    "-patch-executor", "-fix-operator", "-corrector",
    "-deployment-hotfix-operator", "-remediation-operator",
    "-rollout-corrector", "-pipeline-fix-operator",
)

DEVELOPERS = (
    "-developer", "-application-developer",
)

LIVE_GUARDS = ("-live-",)

INVESTIGATORS = (
    "-investigator", "-responder", "-advisor", "-coordinator",
    "-analyst", "-planner",
)


def classify(skill_id: str) -> str:
    if skill_id.endswith("-maestro") or skill_id == "kubernetes-maestro" or skill_id == "terraform-maestro":
        return "Agent Skill Read Grep Glob"
    for s in PATCHERS:
        if skill_id.endswith(s):
            return "Read Edit Write MultiEdit Grep Glob"
    if skill_id == "aws-agentcore" or skill_id == "aws-generative-ai-developer":
        return "Read Edit Write MultiEdit Grep Glob Bash"
    for s in DEVELOPERS:
        if skill_id.endswith(s):
            return "Read Edit Write MultiEdit Grep Glob Bash"
    for s in LIVE_GUARDS:
        if s in skill_id and skill_id.endswith("-guard"):
            return "Read Grep Glob WebFetch"
    if skill_id == "velero-backup-restore-guard":
        return "Read Grep Glob WebFetch"
    for s in INVESTIGATORS:
        if skill_id.endswith(s):
            return "Read Grep Glob WebFetch"
    if skill_id == "finops-cloud-price-advisor":
        return "Read Grep Glob WebFetch"
    return "Read Grep Glob"


def apply(skill_md: Path) -> tuple[bool, str]:
    text = skill_md.read_text(encoding="utf-8")
    if not text.startswith("---\n"):
        return False, "no frontmatter"

    end = text.find("\n---", 4)
    if end == -1:
        return False, "unterminated frontmatter"

    fm_block = text[4:end]
    if "\nallowed-tools:" in "\n" + fm_block:
        return False, "already has allowed-tools"

    skill_id = skill_md.parent.name
    tools = classify(skill_id)

    # Insert allowed-tools immediately after the description block,
    # before the metadata: block (if any) or before the closing ---.
    lines = fm_block.split("\n")
    insert_at = None
    for i, line in enumerate(lines):
        if line.startswith("metadata:"):
            insert_at = i
            break
    if insert_at is None:
        insert_at = len(lines)

    lines.insert(insert_at, f"allowed-tools: {tools}")
    new_fm = "\n".join(lines)
    new_text = "---\n" + new_fm + text[end:]
    skill_md.write_text(new_text, encoding="utf-8")
    return True, tools


def main() -> int:
    files = sorted(SKILLS_DIR.glob("*/*/SKILL.md"))
    summary: dict[str, int] = {}
    skipped = 0
    for f in files:
        ok, info = apply(f)
        if ok:
            summary[info] = summary.get(info, 0) + 1
        else:
            skipped += 1
            if info != "already has allowed-tools":
                print(f"WARN: {f.relative_to(ROOT)}: {info}", file=sys.stderr)

    total = sum(summary.values())
    print(f"Applied allowed-tools to {total} skills (skipped {skipped})")
    for tools, count in sorted(summary.items(), key=lambda x: -x[1]):
        print(f"  {count:3d}  {tools}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
