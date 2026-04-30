#!/usr/bin/env python3
"""Run Terraform analyzer scripts against real fixture plans."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

CASES = [
    {
        "name": "azurerm-order-only",
        "script": ROOT / "skills/terraform/terraform-azurerm-set-diff-analyzer/scripts/analyze_plan.py",
        "fixture": ROOT / "skills/terraform/terraform-azurerm-set-diff-analyzer/fixtures/order-only-application-gateway.json",
        "exit_code": 0,
        "summary": {"order_only_count": 1, "actual_set_changes_count": 0},
        "attribute_path": "rewrite_rule_set.rewrite_rule",
        "has_real_changes": False,
    },
    {
        "name": "azurerm-actual-change",
        "script": ROOT / "skills/terraform/terraform-azurerm-set-diff-analyzer/scripts/analyze_plan.py",
        "fixture": ROOT / "skills/terraform/terraform-azurerm-set-diff-analyzer/fixtures/actual-request-routing-rule-change.json",
        "exit_code": 1,
        "summary": {"order_only_count": 0, "actual_set_changes_count": 1},
        "attribute_path": "request_routing_rule",
        "has_real_changes": True,
    },
    {
        "name": "oci-order-only",
        "script": ROOT / "skills/terraform/terraform-oci-set-diff-analyzer/scripts/analyze_plan.py",
        "fixture": ROOT / "skills/terraform/terraform-oci-set-diff-analyzer/fixtures/order-only-routing-policy.json",
        "exit_code": 0,
        "summary": {"order_only_count": 1, "actual_set_changes_count": 0},
        "attribute_path": "rules",
        "has_real_changes": False,
    },
    {
        "name": "oci-actual-change",
        "script": ROOT / "skills/terraform/terraform-oci-set-diff-analyzer/scripts/analyze_plan.py",
        "fixture": ROOT / "skills/terraform/terraform-oci-set-diff-analyzer/fixtures/actual-routing-policy-action-change.json",
        "exit_code": 1,
        "summary": {"order_only_count": 0, "actual_set_changes_count": 1},
        "attribute_path": "rules.actions",
        "has_real_changes": True,
    },
    {
        "name": "aws-order-only",
        "script": ROOT / "skills/terraform/terraform-aws-inline-diff-analyzer/scripts/analyze_plan.py",
        "fixture": ROOT / "skills/terraform/terraform-aws-inline-diff-analyzer/fixtures/order-only-wafv2-rules.json",
        "exit_code": 0,
        "summary": {"order_only_count": 1, "actual_set_changes_count": 0},
        "attribute_path": "rule",
        "has_real_changes": False,
    },
    {
        "name": "aws-actual-change",
        "script": ROOT / "skills/terraform/terraform-aws-inline-diff-analyzer/scripts/analyze_plan.py",
        "fixture": ROOT / "skills/terraform/terraform-aws-inline-diff-analyzer/fixtures/actual-route-target-change.json",
        "exit_code": 1,
        "summary": {"order_only_count": 0, "actual_set_changes_count": 1},
        "attribute_path": "route",
        "has_real_changes": True,
    },
]


def flatten_paths(changes: list[dict]) -> list[str]:
    paths: list[str] = []
    for change in changes:
        path = change.get("path")
        if path:
            paths.append(path)
        paths.extend(flatten_paths(change.get("nested_changes", [])))
    return paths


def run_json(script: Path, fixture: Path) -> tuple[int, dict]:
    proc = subprocess.run(
        [sys.executable, str(script), str(fixture), "--format", "json"],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )
    if proc.returncode != 0:
        raise AssertionError(f"json run failed: {proc.stderr or proc.stdout}")
    try:
        return proc.returncode, json.loads(proc.stdout)
    except json.JSONDecodeError as exc:
        raise AssertionError(f"invalid json output: {exc}\n{proc.stdout}") from exc


def run_exit_code(script: Path, fixture: Path) -> int:
    proc = subprocess.run(
        [sys.executable, str(script), str(fixture), "--exit-code", "--format", "summary"],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )
    return proc.returncode


def main() -> int:
    errors: list[str] = []
    for case in CASES:
        try:
            _, payload = run_json(case["script"], case["fixture"])
            summary = payload["summary"]
            for key, expected in case["summary"].items():
                if summary.get(key) != expected:
                    raise AssertionError(
                        f"{key} expected {expected} got {summary.get(key)}"
                    )
            if payload.get("has_real_changes") != case["has_real_changes"]:
                raise AssertionError(
                    f"has_real_changes expected {case['has_real_changes']} got {payload.get('has_real_changes')}"
                )
            paths = []
            for resource in payload.get("resources", []):
                paths.extend(flatten_paths(resource.get("set_changes", [])))
            if case["attribute_path"] not in paths:
                raise AssertionError(
                    f"expected attribute path {case['attribute_path']!r} not found in {paths}"
                )
            exit_code = run_exit_code(case["script"], case["fixture"])
            if exit_code != case["exit_code"]:
                raise AssertionError(
                    f"exit_code expected {case['exit_code']} got {exit_code}"
                )
        except Exception as exc:
            errors.append(f"{case['name']}: {exc}")

    if errors:
        for err in errors:
            print(f"ERROR: {err}", file=sys.stderr)
        return 1

    print(f"OK: validated {len(CASES)} Terraform analyzer fixture runs")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
