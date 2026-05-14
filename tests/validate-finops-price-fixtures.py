#!/usr/bin/env python3
"""Grader for finops-cloud-price-advisor v0.2.0 integration test fixtures.

Validates structure and provenance labels of the expected output fixtures
against their paired inputs. No network calls are made — all checks are
deterministic against static JSON files.

Exit codes:
  0 — all fixtures pass
  1 — one or more fixtures fail
"""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FIXTURE_DIR = ROOT / "tests" / "fixtures" / "finops-cloud-price-advisor"
INPUTS_DIR = FIXTURE_DIR / "inputs"
EXPECTED_DIR = FIXTURE_DIR / "expected"
TAXONOMY_PATH = FIXTURE_DIR / "taxonomy.json"

VALID_PROVIDERS = {"aws", "azure", "oci", "scaleway", "gandi", "alibaba", "tencent", "comparative"}
VALID_CURRENCIES = {"USD", "EUR", "CNY"}
VALID_PROVENANCE_LABELS = {"live-price", "documentation-based", "assumed", "excluded"}
REQUIRED_TAXONOMY_PROVIDERS = {"aws", "azure", "oci", "scaleway", "gandi", "alibaba", "tencent"}

# Credential patterns — must be wrapped in <FAKE> when present in fixtures.
# Each tuple is (name, compiled pattern).
_CREDENTIAL_PATTERNS: list[tuple[str, re.Pattern[str]]] = [
    ("AWS access key", re.compile(r"AKIA[0-9A-Z]{16}")),
    ("Alibaba access key", re.compile(r"LTAI[0-9A-Za-z]{12,20}")),
    ("Tencent secret ID", re.compile(r"AKID[0-9A-Za-z]{13,28}")),
]


def _load_json(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def _input_has_unmarked_credential(inp: dict) -> list[str]:
    """Return a list of violation descriptions for any real-looking credential
    patterns that are not wrapped in <FAKE> tags.

    The raw string value is never echoed to output — only the field name and
    pattern name are returned to avoid leaking credential-shaped strings into
    CI logs.
    """
    violations: list[str] = []
    raw = json.dumps(inp)
    for name, pattern in _CREDENTIAL_PATTERNS:
        for match in pattern.finditer(raw):
            start = max(0, match.start() - 7)
            # Check a small window around the match for the <FAKE> marker.
            window = raw[start : match.end() + 7]
            if "<FAKE>" not in window:
                violations.append(f"unmarked {name} pattern found in input JSON")
    return violations


def _validate_expected(inp: dict, exp: dict) -> list[str]:
    """Return a list of failure reasons for a single fixture pair."""
    failures: list[str] = []
    fixture_id = inp.get("id", "?")

    # --- provider ---
    provider = exp.get("provider")
    if provider is None:
        failures.append("expected: missing 'provider' field")
    elif provider not in VALID_PROVIDERS:
        failures.append(f"expected: provider {provider!r} not in valid set {sorted(VALID_PROVIDERS)}")

    # --- currency ---
    currency = exp.get("currency")
    if currency is None:
        failures.append("expected: missing 'currency' field")
    elif currency not in VALID_CURRENCIES:
        failures.append(f"expected: currency {currency!r} not in {sorted(VALID_CURRENCIES)}")

    # --- provenance_label ---
    provenance_label = exp.get("provenance_label")
    if provenance_label is None:
        failures.append("expected: missing 'provenance_label' field")
    elif provenance_label not in VALID_PROVENANCE_LABELS:
        failures.append(
            f"expected: provenance_label {provenance_label!r} not in {sorted(VALID_PROVENANCE_LABELS)}"
        )

    # --- key_stored must be false ---
    key_stored = exp.get("key_stored")
    if key_stored is not False:
        failures.append(f"expected: key_stored must be false, got {key_stored!r}")

    # --- CNY fixtures must declare requires_usd_conversion: true ---
    if currency == "CNY":
        if exp.get("requires_usd_conversion") is not True:
            failures.append("expected: CNY fixture must have requires_usd_conversion: true")

    # --- Gandi with-key fixture: disclaimer_required and key_stored checks ---
    auth_mode = inp.get("auth_mode") or exp.get("auth_mode")
    if inp.get("provider") == "gandi" and auth_mode == "user-provided":
        if exp.get("disclaimer_required") is not True:
            failures.append("expected: Gandi user-provided-key fixture must have disclaimer_required: true")
        if exp.get("key_stored") is not False:
            failures.append("expected: Gandi user-provided-key fixture must have key_stored: false")

    # --- Input credential sweep (skip for adversarial fixtures — intentional payloads) ---
    if not inp.get("intent"):
        cred_violations = _input_has_unmarked_credential(inp)
        for v in cred_violations:
            failures.append(f"input: {v}")

    # --- Fixture 004: user_key_marker must carry <FAKE> prefix ---
    if fixture_id == "004":
        marker = inp.get("user_key_marker", "")
        if not marker.startswith("<FAKE>"):
            failures.append("input 004: user_key_marker must start with <FAKE>")

    return failures


def _validate_taxonomy() -> list[str]:
    """Check that taxonomy.json covers all required providers and counts."""
    failures: list[str] = []
    if not TAXONOMY_PATH.exists():
        return ["taxonomy.json not found"]
    taxonomy = _load_json(TAXONOMY_PATH)
    coverage = set(taxonomy.get("provider_coverage", []))
    missing = REQUIRED_TAXONOMY_PROVIDERS - coverage
    if missing:
        failures.append(f"taxonomy.json provider_coverage missing: {sorted(missing)}")
    extra = coverage - REQUIRED_TAXONOMY_PROVIDERS
    if extra:
        failures.append(f"taxonomy.json provider_coverage has unexpected entries: {sorted(extra)}")
    # Verify adversarial_count is declared when adversarial fixtures exist
    if taxonomy.get("adversarial_count", 0) == 0:
        adv_inputs = list(INPUTS_DIR.glob("adv-*.json")) if INPUTS_DIR.is_dir() else []
        if adv_inputs:
            failures.append("taxonomy.json missing adversarial_count but adversarial fixtures exist")
    return failures


def main() -> int:
    passes = 0
    fails = 0

    # Validate taxonomy first.
    taxonomy_errors = _validate_taxonomy()
    for err in taxonomy_errors:
        print(f"FAIL: taxonomy — {err}")
        fails += 1
    if not taxonomy_errors:
        print("PASS: taxonomy — provider_coverage validated")

    # Discover input fixtures in sorted order.
    if not INPUTS_DIR.is_dir():
        print(f"ERROR: inputs directory not found at {INPUTS_DIR}", file=sys.stderr)
        return 1

    input_files = sorted(INPUTS_DIR.glob("*.json"))
    if not input_files:
        print("ERROR: no input fixtures found", file=sys.stderr)
        return 1

    for inp_path in input_files:
        stem = inp_path.stem
        exp_path = EXPECTED_DIR / f"{stem}.json"

        if not exp_path.exists():
            print(f"FAIL: {stem} — missing expected file {exp_path.name}")
            fails += 1
            continue

        try:
            inp = _load_json(inp_path)
            exp = _load_json(exp_path)
        except json.JSONDecodeError as exc:
            print(f"FAIL: {stem} — JSON parse error: {exc}")
            fails += 1
            continue

        errors = _validate_expected(inp, exp)
        if errors:
            for err in errors:
                print(f"FAIL: {stem} — {err}")
            fails += 1
        else:
            print(f"PASS: {stem}")
            passes += 1

    total = passes + fails
    print(f"\nResults: {passes}/{total} fixtures PASS")
    return 0 if fails == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
