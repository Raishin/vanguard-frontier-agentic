#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Terraform Plan Analyzer for AWS inline or repeated-block attributes.

Analyzes terraform plan JSON output to distinguish between:
- Likely inline-block churn or collection reflow
- Actual additions/deletions/modifications

Usage:
    terraform show -json plan.tfplan | python analyze_plan.py
    python analyze_plan.py plan.json
    python analyze_plan.py plan.json --format json --exit-code
"""

from __future__ import annotations

import argparse
import json
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, Dict, List, Optional, Set

EXIT_NO_CHANGES = 0
EXIT_ORDER_ONLY = 0
EXIT_SET_CHANGES = 1
EXIT_RESOURCE_REPLACE = 2
EXIT_ERROR = 3

DEFAULT_ATTRIBUTES_PATH = (
    Path(__file__).parent.parent / "references" / "aws_inline_attributes.json"
)


class Config:
    ignore_case: bool = False
    quiet: bool = False
    verbose: bool = False
    warnings: List[str] = []


CONFIG = Config()


def warn(message: str) -> None:
    CONFIG.warnings.append(message)
    if CONFIG.verbose:
        print(f"Warning: {message}", file=sys.stderr)


def load_set_attributes(path: Optional[Path] = None) -> Dict[str, Dict[str, Any]]:
    attributes_path = path or DEFAULT_ATTRIBUTES_PATH
    try:
        with open(attributes_path, "r", encoding="utf-8") as f:
            data = json.load(f)
        return data.get("resources", {})
    except FileNotFoundError:
        warn(f"Attributes file not found: {attributes_path}")
        return {}
    except json.JSONDecodeError as e:
        print(f"Error: Invalid JSON in attributes file: {e}", file=sys.stderr)
        sys.exit(EXIT_ERROR)


AWS_INLINE_ATTRIBUTES: Dict[str, Any] = {}


def get_attr_config(attr_def: Any) -> tuple:
    if attr_def is None:
        return (None, {})
    if isinstance(attr_def, str):
        return (attr_def, {})
    if isinstance(attr_def, dict):
        key_attr = attr_def.get("_key")
        nested_attrs = {k: v for k, v in attr_def.items() if k != "_key"}
        return (key_attr, nested_attrs)
    return (None, {})


@dataclass
class SetAttributeChange:
    attribute_name: str
    path: str = ""
    order_only_count: int = 0
    added: List[str] = field(default_factory=list)
    removed: List[str] = field(default_factory=list)
    modified: List[tuple] = field(default_factory=list)
    nested_changes: List["SetAttributeChange"] = field(default_factory=list)
    is_primitive: bool = False
    primitive_added: List[Any] = field(default_factory=list)
    primitive_removed: List[Any] = field(default_factory=list)


@dataclass
class ResourceChange:
    address: str
    resource_type: str
    actions: List[str] = field(default_factory=list)
    set_changes: List[SetAttributeChange] = field(default_factory=list)
    other_changes: List[str] = field(default_factory=list)
    is_replace: bool = False
    is_create: bool = False
    is_delete: bool = False


@dataclass
class AnalysisResult:
    resources: List[ResourceChange] = field(default_factory=list)
    order_only_count: int = 0
    actual_set_changes_count: int = 0
    replace_count: int = 0
    create_count: int = 0
    delete_count: int = 0
    other_changes_count: int = 0
    warnings: List[str] = field(default_factory=list)


def get_element_key(element: Dict[str, Any], key_attr: Optional[str]) -> str:
    if key_attr and key_attr in element:
        val = element[key_attr]
        if CONFIG.ignore_case and isinstance(val, str):
            return val.lower()
        return str(val)
    return str(hash(json.dumps(element, sort_keys=True)))


def normalize_value(val: Any) -> Any:
    if val == "" or val is None:
        return None
    if isinstance(val, list) and len(val) == 0:
        return None
    if isinstance(val, float) and val.is_integer():
        return int(val)
    return val


def normalize_for_comparison(val: Any) -> Any:
    val = normalize_value(val)
    if CONFIG.ignore_case and isinstance(val, str):
        return val.lower()
    return val


def values_equivalent(before_val: Any, after_val: Any) -> bool:
    return normalize_for_comparison(before_val) == normalize_for_comparison(after_val)


def compare_elements(
    before: Dict[str, Any], after: Dict[str, Any], nested_attrs: Dict[str, Any] = None
) -> tuple:
    nested_attrs = nested_attrs or {}
    simple_diffs = {}
    nested_set_attrs = []

    all_keys = set(before.keys()) | set(after.keys())
    for key in all_keys:
        before_val = before.get(key)
        after_val = after.get(key)
        if key in nested_attrs:
            if before_val != after_val:
                nested_set_attrs.append((key, before_val, after_val, nested_attrs[key]))
        elif not values_equivalent(before_val, after_val):
            simple_diffs[key] = {"before": before_val, "after": after_val}

    return (simple_diffs, nested_set_attrs)


def analyze_primitive_set(
    before_list: Optional[List[Any]],
    after_list: Optional[List[Any]],
    attr_name: str,
    path: str = "",
) -> SetAttributeChange:
    full_path = f"{path}.{attr_name}" if path else attr_name
    change = SetAttributeChange(
        attribute_name=attr_name, path=full_path, is_primitive=True
    )

    before_set = set(before_list) if before_list else set()
    after_set = set(after_list) if after_list else set()
    if CONFIG.ignore_case:
        before_normalized = {v.lower() if isinstance(v, str) else v for v in before_set}
        after_normalized = {v.lower() if isinstance(v, str) else v for v in after_set}
    else:
        before_normalized = before_set
        after_normalized = after_set

    removed = before_normalized - after_normalized
    added = after_normalized - before_normalized
    if removed:
        change.primitive_removed = list(removed)
    if added:
        change.primitive_added = list(added)

    common = before_normalized & after_normalized
    if common and not removed and not added:
        change.order_only_count = len(common)

    return change


def analyze_set_attribute(
    before_list: Optional[List[Dict[str, Any]]],
    after_list: Optional[List[Dict[str, Any]]],
    key_attr: Optional[str],
    attr_name: str,
    nested_attrs: Dict[str, Any] = None,
    path: str = "",
    after_unknown: Optional[Dict[str, Any]] = None,
) -> SetAttributeChange:
    nested_attrs = nested_attrs or {}
    full_path = f"{path}.{attr_name}" if path else attr_name
    change = SetAttributeChange(attribute_name=attr_name, path=full_path)

    if before_list and len(before_list) > 0 and not isinstance(before_list[0], dict):
        return analyze_primitive_set(before_list, after_list, attr_name, path)
    if after_list and len(after_list) > 0 and not isinstance(after_list[0], dict):
        return analyze_primitive_set(before_list, after_list, attr_name, path)

    before_map = {
        get_element_key(elem, key_attr): elem for elem in (before_list or []) if elem
    }
    after_map = {
        get_element_key(elem, key_attr): elem for elem in (after_list or []) if elem
    }

    all_keys = set(before_map.keys()) | set(after_map.keys())
    for key in all_keys:
        if key not in before_map:
            change.added.append(key)
        elif key not in after_map:
            change.removed.append(key)
        else:
            simple_diffs, nested_set_attrs = compare_elements(
                before_map[key], after_map[key], nested_attrs
            )
            if not simple_diffs and not nested_set_attrs:
                change.order_only_count += 1
            else:
                if simple_diffs:
                    change.modified.append((key, simple_diffs))
                for nested_attr_name, nested_before, nested_after, nested_def in nested_set_attrs:
                    nested_key_attr, nested_nested_attrs = get_attr_config(nested_def)
                    nested_unknown = None
                    if after_unknown and key in after_unknown:
                        unknown_elem = after_unknown[key]
                        if isinstance(unknown_elem, dict):
                            nested_unknown = unknown_elem.get(nested_attr_name)
                    nested_change = analyze_set_attribute(
                        nested_before,
                        nested_after,
                        nested_key_attr,
                        nested_attr_name,
                        nested_nested_attrs,
                        path=full_path,
                        after_unknown=nested_unknown,
                    )
                    if (
                        nested_change.order_only_count > 0
                        or nested_change.added
                        or nested_change.removed
                        or nested_change.modified
                        or nested_change.nested_changes
                    ):
                        change.nested_changes.append(nested_change)

    return change


def is_collection_attribute(value: Any) -> bool:
    return isinstance(value, list)


def analyze_resource_change(rc: Dict[str, Any]) -> ResourceChange:
    address = rc.get("address", "<unknown>")
    resource_type = rc.get("type", "")
    actions = rc.get("change", {}).get("actions", [])
    before = rc.get("change", {}).get("before") or {}
    after = rc.get("change", {}).get("after") or {}
    after_unknown = rc.get("change", {}).get("after_unknown") or {}

    result = ResourceChange(
        address=address,
        resource_type=resource_type,
        actions=actions,
        is_replace=actions == ["delete", "create"],
        is_create=actions == ["create"],
        is_delete=actions == ["delete"],
    )

    attr_config_map = AWS_INLINE_ATTRIBUTES.get(resource_type, {})
    for attr_name, attr_def in attr_config_map.items():
        before_val = before.get(attr_name)
        after_val = after.get(attr_name)
        if not is_collection_attribute(before_val) and not is_collection_attribute(after_val):
            continue
        key_attr, nested_attrs = get_attr_config(attr_def)
        change = analyze_set_attribute(
            before_val,
            after_val,
            key_attr,
            attr_name,
            nested_attrs,
            after_unknown=after_unknown.get(attr_name) if isinstance(after_unknown, dict) else None,
        )
        if (
            change.order_only_count > 0
            or change.added
            or change.removed
            or change.modified
            or change.nested_changes
        ):
            result.set_changes.append(change)

    return result


def analyze_plan(plan: Dict[str, Any]) -> AnalysisResult:
    result = AnalysisResult()
    for rc in plan.get("resource_changes", []):
        if rc.get("type") not in AWS_INLINE_ATTRIBUTES:
            continue
        analyzed = analyze_resource_change(rc)
        if analyzed.set_changes or analyzed.is_replace or analyzed.is_create or analyzed.is_delete:
            result.resources.append(analyzed)
            if analyzed.is_replace:
                result.replace_count += 1
            if analyzed.is_create:
                result.create_count += 1
            if analyzed.is_delete:
                result.delete_count += 1
            for sc in analyzed.set_changes:
                if sc.added or sc.removed or sc.modified or sc.nested_changes:
                    result.actual_set_changes_count += 1
                elif sc.order_only_count > 0:
                    result.order_only_count += 1
    result.warnings = CONFIG.warnings
    return result


def to_markdown(result: AnalysisResult) -> str:
    if not result.resources:
        return "No supported AWS inline-diff changes detected."
    lines = [
        "# Terraform AWS Inline Diff Analysis",
        "",
        f"- Order-only / likely inline noise: {result.order_only_count}",
        f"- Actual inline changes: {result.actual_set_changes_count}",
        f"- Replacements: {result.replace_count}",
        "",
    ]
    for resource in result.resources:
        lines.append(f"## {resource.address} ({resource.resource_type})")
        lines.append(f"- Actions: {', '.join(resource.actions) if resource.actions else 'unknown'}")
        for sc in resource.set_changes:
            lines.append(f"- Attribute: `{sc.path}`")
            if sc.order_only_count:
                lines.append(f"  - likely order-only elements: {sc.order_only_count}")
            if sc.added:
                lines.append(f"  - added keys: {', '.join(sc.added)}")
            if sc.removed:
                lines.append(f"  - removed keys: {', '.join(sc.removed)}")
            if sc.modified:
                lines.append(f"  - modified elements: {len(sc.modified)}")
            if sc.nested_changes:
                lines.append(f"  - nested changes: {len(sc.nested_changes)}")
        lines.append("")
    return "\n".join(lines).strip()


def to_summary(result: AnalysisResult) -> str:
    return (
        f"🟢 {result.order_only_count} likely inline-noise | "
        f"🟡 {result.actual_set_changes_count} inline changes | "
        f"🔴 {result.replace_count} replacements"
    )


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("plan_json", nargs="?", help="Path to Terraform plan JSON file")
    parser.add_argument("--format", "-f", choices=["markdown", "json", "summary"], default="markdown")
    parser.add_argument("--exit-code", "-e", action="store_true")
    parser.add_argument("--quiet", "-q", action="store_true")
    parser.add_argument("--verbose", "-v", action="store_true")
    parser.add_argument("--ignore-case", action="store_true")
    parser.add_argument("--attributes", type=Path, help="Path to custom attribute definition file")
    return parser.parse_args()


def read_plan(path: Optional[str]) -> Dict[str, Any]:
    try:
        if path:
            return json.loads(Path(path).read_text(encoding="utf-8"))
        return json.load(sys.stdin)
    except FileNotFoundError:
        print(f"Error: Plan file not found: {path}", file=sys.stderr)
        sys.exit(EXIT_ERROR)
    except json.JSONDecodeError as exc:
        print(f"Error: Invalid plan JSON: {exc}", file=sys.stderr)
        sys.exit(EXIT_ERROR)


def main() -> int:
    global AWS_INLINE_ATTRIBUTES
    args = parse_args()
    CONFIG.ignore_case = args.ignore_case
    CONFIG.quiet = args.quiet
    CONFIG.verbose = args.verbose
    AWS_INLINE_ATTRIBUTES = load_set_attributes(args.attributes)
    plan = read_plan(args.plan_json)
    result = analyze_plan(plan)

    if args.format == "json":
        print(
            json.dumps(
                {
                    "summary": {
                        "order_only_count": result.order_only_count,
                        "actual_set_changes_count": result.actual_set_changes_count,
                        "replace_count": result.replace_count,
                        "create_count": result.create_count,
                        "delete_count": result.delete_count,
                    },
                    "has_real_changes": (
                        result.actual_set_changes_count > 0
                        or result.replace_count > 0
                        or result.create_count > 0
                        or result.delete_count > 0
                    ),
                    "resources": [resource.__dict__ for resource in result.resources],
                    "warnings": result.warnings,
                },
                indent=2,
                default=lambda o: o.__dict__,
            )
        )
    elif args.format == "summary":
        print(to_summary(result))
    else:
        print(to_markdown(result))

    if not args.exit_code:
        return EXIT_NO_CHANGES
    if result.replace_count:
        return EXIT_RESOURCE_REPLACE
    if result.actual_set_changes_count:
        return EXIT_SET_CHANGES
    return EXIT_ORDER_ONLY


if __name__ == "__main__":
    raise SystemExit(main())
