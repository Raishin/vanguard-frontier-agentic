#!/usr/bin/env python3
"""Validate marketplace catalogs and asset metadata without external dependencies."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

CATALOGS = {
    "skill": ROOT / "catalog" / "skills.json",
    "agent": ROOT / "catalog" / "agents.json",
    "rule": ROOT / "catalog" / "rules.json",
    "mcp-reference": ROOT / "catalog" / "mcp-references.json",
}

ALLOWED_PROVIDERS = {"aws", "azure", "oracle", "oci", "gcp", "kubernetes", "terraform", "multi-cloud", "generic"}
ALLOWED_HARNESSES = {"codex", "copilot", "claude-code", "cursor", "gemini", "kiro", "other"}
ALLOWED_SOURCE_TYPES = {"original", "adapted", "reference-only"}
REQUIRED_COMMON = {
    "id",
    "name",
    "type",
    "provider",
    "harnesses",
    "summary",
    "source_type",
    "official_docs",
    "security_notes",
    "last_verified",
    "path",
}
REQUIRED_MCP = {"official_project_url", "vendor", "auth_model", "install_example", "unofficial_warning"}
ID_RE = re.compile(r"^[a-z0-9][a-z0-9-]*$")
DATE_RE = re.compile(r"^\d{4}-\d{2}-\d{2}$")
SEMVER_RE = re.compile(r"^\d+\.\d+\.\d+$")
URL_RE = re.compile(r"^https?://")
SECRET_PATTERNS = [
    re.compile(r"AKIA[0-9A-Z]{16}"),
    re.compile(r"ASIA[0-9A-Z]{16}"),
    re.compile(r"-----BEGIN (RSA |EC |OPENSSH |DSA )?PRIVATE KEY-----"),
    re.compile(r"(?i)(api[_-]?key|secret|token|password)\s*[:=]\s*['\"][^'\"]{12,}['\"]"),
]


def load_json(path: Path):
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except Exception as exc:
        raise AssertionError(f"{path}: invalid JSON: {exc}") from exc


def assert_true(condition: bool, message: str) -> None:
    if not condition:
        raise AssertionError(message)


def validate_item(item: dict, expected_type: str) -> None:
    missing = REQUIRED_COMMON - item.keys()
    if expected_type in {"skill", "agent"}:
        missing |= {"version"} - item.keys()
    if expected_type == "mcp-reference":
        missing |= REQUIRED_MCP - item.keys()
    assert_true(not missing, f"{item.get('id', '<unknown>')}: missing fields {sorted(missing)}")
    assert_true(item["type"] == expected_type, f"{item['id']}: expected type {expected_type}, got {item['type']}")
    assert_true(ID_RE.match(item["id"]) is not None, f"{item['id']}: invalid id format")
    if expected_type in {"skill", "agent"}:
        assert_true(SEMVER_RE.match(item["version"]) is not None, f"{item['id']}: invalid version {item['version']}")
    assert_true(item["provider"] in ALLOWED_PROVIDERS, f"{item['id']}: invalid provider {item['provider']}")
    assert_true(item["source_type"] in ALLOWED_SOURCE_TYPES, f"{item['id']}: invalid source_type {item['source_type']}")
    assert_true(DATE_RE.match(item["last_verified"]) is not None, f"{item['id']}: invalid last_verified")
    assert_true(isinstance(item["harnesses"], list) and item["harnesses"], f"{item['id']}: harnesses must be non-empty")
    bad_harnesses = set(item["harnesses"]) - ALLOWED_HARNESSES
    assert_true(not bad_harnesses, f"{item['id']}: invalid harnesses {sorted(bad_harnesses)}")
    assert_true(isinstance(item["official_docs"], list) and item["official_docs"], f"{item['id']}: official_docs must be non-empty")
    for url in item["official_docs"]:
        assert_true(URL_RE.match(url) is not None, f"{item['id']}: official_doc is not URL: {url}")
    assert_true(len(item["summary"]) >= 20, f"{item['id']}: summary too short")
    assert_true(len(item["security_notes"]) >= 20, f"{item['id']}: security_notes too short")
    target = ROOT / item["path"]
    assert_true(target.exists(), f"{item['id']}: path does not exist: {item['path']}")


def validate_metadata_file(item: dict) -> None:
    target = ROOT / item["path"]
    if target.is_dir():
        metadata_files = list(target.glob("metadata.json"))
    else:
        metadata_files = list(target.parent.glob(f"{target.stem}.metadata.json"))
    assert_true(metadata_files, f"{item['id']}: no metadata file beside asset")
    metadata = load_json(metadata_files[0])
    assert_true(metadata["id"] == item["id"], f"{item['id']}: metadata id mismatch in {metadata_files[0]}")
    if item["type"] in {"skill", "agent"}:
        assert_true(metadata.get("version") == item["version"], f"{item['id']}: metadata version mismatch in {metadata_files[0]}")


def validate_no_obvious_secrets() -> None:
    checked_suffixes = {".md", ".json", ".py", ".toml", ".yaml", ".yml"}
    for path in ROOT.rglob("*"):
        if ".git" in path.parts or path.is_dir() or path.suffix not in checked_suffixes:
            continue
        text = path.read_text(encoding="utf-8", errors="ignore")
        for pattern in SECRET_PATTERNS:
            assert_true(pattern.search(text) is None, f"possible secret pattern in {path.relative_to(ROOT)}")


def main() -> int:
    errors: list[str] = []
    seen_ids: set[str] = set()
    for expected_type, catalog_path in CATALOGS.items():
        try:
            items = load_json(catalog_path)
            assert_true(isinstance(items, list), f"{catalog_path}: catalog must be a list")
            for item in items:
                validate_item(item, expected_type)
                assert_true(item["id"] not in seen_ids, f"duplicate id: {item['id']}")
                seen_ids.add(item["id"])
                validate_metadata_file(item)
        except AssertionError as exc:
            errors.append(str(exc))
    try:
        validate_no_obvious_secrets()
    except AssertionError as exc:
        errors.append(str(exc))

    if errors:
        for error in errors:
            print(f"ERROR: {error}", file=sys.stderr)
        return 1
    print(f"OK: validated {len(seen_ids)} catalog entries and scanned for obvious secrets")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
