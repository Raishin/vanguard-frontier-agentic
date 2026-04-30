#!/usr/bin/env python3
"""Validate Terraform skill and agent tier contracts."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SKILLS = ROOT / "skills" / "terraform"
AGENTS = ROOT / "agents" / "terraform"
SKILL_CATALOG = ROOT / "catalog" / "skills.json"
AGENT_CATALOG = ROOT / "catalog" / "agents.json"

EXPECTED = {
    "terraform-reviewer": {
        "keywords": ["plan", "backend", "workspace", "state", "drift", "provider"],
        "agent": "terraform-reviewer-agent",
        "sandbox": "read-only",
    },
    "terraform-repo-patch-operator": {
        "keywords": ["module", "backend", "workspace", "plan-safety", "repo-write", "live apply"],
        "agent": "terraform-repo-patch-operator-agent",
        "sandbox": "workspace-write",
    },
    "terraform-live-apply-guard": {
        "keywords": ["backend", "workspace", "lock", "identity", "saved plan", "approval"],
        "agent": "terraform-live-apply-guard-agent",
        "sandbox": "workspace-write",
    },
}

SPECIALIZED_SKILLS = {
    "terraform-azurerm-set-diff-analyzer": {
        "keywords": ["azurerm", "set-type", "plan json", "application gateway", "nsg", "real changes"],
        "required_files": [
            "references/workflow-and-output.md",
            "references/safety-checklist.md",
            "references/official-sources.md",
            "references/azurerm_set_attributes.md",
            "references/azurerm_set_attributes.json",
            "fixtures/order-only-application-gateway.json",
            "fixtures/actual-request-routing-rule-change.json",
            "scripts/README.md",
            "scripts/analyze_plan.py",
            "metadata.json",
        ],
    },
    "terraform-oci-set-diff-analyzer": {
        "keywords": ["oci", "plan json", "repeated-block", "load balancer", "route table", "real changes"],
        "required_files": [
            "references/workflow-and-output.md",
            "references/safety-checklist.md",
            "references/official-sources.md",
            "references/oci_set_attributes.md",
            "references/oci_set_attributes.json",
            "fixtures/order-only-routing-policy.json",
            "fixtures/actual-routing-policy-action-change.json",
            "scripts/README.md",
            "scripts/analyze_plan.py",
            "metadata.json",
        ],
    },
    "terraform-aws-inline-diff-analyzer": {
        "keywords": ["aws", "plan json", "listener rule", "security group", "route table", "wafv2"],
        "required_files": [
            "references/workflow-and-output.md",
            "references/safety-checklist.md",
            "references/official-sources.md",
            "references/aws_inline_attributes.md",
            "references/aws_inline_attributes.json",
            "fixtures/order-only-wafv2-rules.json",
            "fixtures/actual-route-target-change.json",
            "scripts/README.md",
            "scripts/analyze_plan.py",
            "metadata.json",
        ],
    }
}

def load_json(path: Path):
    return json.loads(path.read_text(encoding="utf-8"))

def frontmatter(path: Path) -> str:
    text = path.read_text(encoding="utf-8")
    if not text.startswith("---\n"):
        raise AssertionError(f"{path}: missing opening frontmatter fence")
    parts = text.split("---", 2)
    if len(parts) < 3:
        raise AssertionError(f"{path}: missing closing frontmatter fence")
    return parts[1]

def scalar(fm: str, key: str) -> str:
    match = re.search(rf"^{re.escape(key)}:\s*(.*)$", fm, re.MULTILINE)
    if not match:
        raise AssertionError(f"frontmatter missing {key}")
    return match.group(1).strip().strip('"').strip("'")

def nested(fm: str, key: str) -> str:
    match = re.search(rf"^  {re.escape(key)}:\s*(.*)$", fm, re.MULTILINE)
    if not match:
        raise AssertionError(f"metadata missing {key}")
    return match.group(1).strip().strip('"').strip("'")

def main() -> int:
    errors: list[str] = []
    skill_catalog = {item['id']: item for item in load_json(SKILL_CATALOG) if item.get('provider') == 'terraform'}
    agent_catalog = {item['id']: item for item in load_json(AGENT_CATALOG) if item.get('provider') == 'terraform'}
    for sid, cfg in EXPECTED.items():
        try:
            skill_dir = SKILLS / sid
            fm = frontmatter(skill_dir / 'SKILL.md')
            desc = scalar(fm, 'description').lower()
            if nested(fm, 'author') != 'github: Raishin':
                raise AssertionError('unexpected metadata.author')
            for kw in cfg['keywords']:
                if kw.lower() not in desc:
                    raise AssertionError(f'missing keyword {kw!r}')
            if sid not in skill_catalog:
                raise AssertionError('missing from skills catalog')
            if cfg['agent'] not in agent_catalog:
                raise AssertionError('missing agent from agents catalog')
            codex = (AGENTS / cfg['agent'] / 'harnesses' / 'codex.toml').read_text(encoding='utf-8')
            if f'sandbox_mode = "{cfg["sandbox"]}"' not in codex:
                raise AssertionError('unexpected codex sandbox mode')
            if sid == 'terraform-live-apply-guard':
                for term in ('saved plan', 'lock', 'explicit human approval', 'backend, workspace'):
                    if term not in codex:
                        raise AssertionError(f'missing guarded live term {term!r}')
        except Exception as exc:
            errors.append(f'{sid}: {exc}')
    for sid, cfg in SPECIALIZED_SKILLS.items():
        try:
            skill_dir = SKILLS / sid
            fm = frontmatter(skill_dir / 'SKILL.md')
            desc = scalar(fm, 'description').lower()
            if nested(fm, 'author') != 'github: Raishin':
                raise AssertionError('unexpected metadata.author')
            for kw in cfg['keywords']:
                if kw.lower() not in desc:
                    raise AssertionError(f'missing keyword {kw!r}')
            if sid not in skill_catalog:
                raise AssertionError('missing from skills catalog')
            for rel in cfg['required_files']:
                path = skill_dir / rel
                if not path.exists():
                    raise AssertionError(f'missing required file {rel}')
        except Exception as exc:
            errors.append(f'{sid}: {exc}')
    if errors:
        for err in errors:
            print(f'ERROR: {err}', file=sys.stderr)
        return 1
    print(f'OK: validated {len(EXPECTED)} Terraform tier contracts and {len(SPECIALIZED_SKILLS)} specialized Terraform skills')
    return 0

if __name__ == '__main__':
    raise SystemExit(main())
