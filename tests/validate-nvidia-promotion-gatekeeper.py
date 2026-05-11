#!/usr/bin/env python3
"""Replay the `nvidia-model-promotion-gatekeeper` golden fixtures.

For each fixture under tests/fixtures/nvidia-model-promotion-gatekeeper/*.json:

  1. Load the fixture (operator inputs + stubbed command outputs).
  2. Run the deterministic gate evaluator (this file, ~150 LOC).
  3. Validate the produced attestation against schemas/attestation.schema.json.
  4. Diff verdict + verdict_reasons against expected/<fixture>.json.

Exit non-zero if any fixture's attestation fails schema validation or its
verdict / reasons differ from expected. Prints a one-line summary per fixture.

The gate evaluator here is the *reference* implementation. Live LLM agents
must follow the same gate ordering and rule set described in
skills/nvidia/nvidia-model-promotion-gatekeeper/SKILL.md.
"""

from __future__ import annotations

import json
import re
import sys
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FIXTURE_DIR = ROOT / "tests" / "fixtures" / "nvidia-model-promotion-gatekeeper"
EXPECTED_DIR = FIXTURE_DIR / "expected"
SCHEMA_PATH = ROOT / "schemas" / "attestation.schema.json"

ALLOWED_REGISTRY_PREFIX = "nvcr.io/"
SECRET_FLAG_RE = re.compile(r"(--password|--token|--auth)=\S+", re.IGNORECASE)
NOW = datetime.now(timezone.utc)


def evaluate(fixture: dict) -> dict:
    """Pure gate evaluator. Consumes (inputs, stub_outputs); returns attestation dict."""
    inputs = fixture["inputs"]
    stubs = fixture["stub_outputs"]
    reasons: list[str] = []

    # Inputs completeness check.
    required = ("image_ref", "image_ref_pin", "current_prod_digest",
                "expected_signer_identity", "expected_oidc_issuer")
    missing = [k for k in required if not inputs.get(k)]
    inputs_incomplete = bool(missing) and inputs.get("mode") == "runtime"
    if inputs_incomplete:
        reasons.append("inputs_incomplete")

    # Registry allowlist gate (runs even if other gates would also fire).
    image_ref = inputs.get("image_ref", "")
    if not image_ref.startswith(ALLOWED_REGISTRY_PREFIX):
        reasons.append("unknown_registry")

    # Digest drift gate.
    if "unknown_registry" not in reasons:
        if stubs.get("crane_digest") != inputs.get("image_ref_pin"):
            reasons.append("digest_drift")

    # Signature gates.
    sig = stubs.get("cosign_verify", {}) or {}
    if "unknown_registry" not in reasons:
        if not sig.get("ok"):
            reasons.append("unsigned")
        else:
            if sig.get("signer_identity") != inputs.get("expected_signer_identity"):
                reasons.append("wrong_identity")
            if sig.get("issuer") != inputs.get("expected_oidc_issuer"):
                reasons.append("wrong_issuer")
            cert_not_after = sig.get("cert_not_after")
            if cert_not_after:
                try:
                    not_after = datetime.fromisoformat(cert_not_after.replace("Z", "+00:00"))
                    if not_after < NOW:
                        reasons.append("expired_cert")
                except ValueError:
                    reasons.append("expired_cert")

    # Rekor reachability.
    rekor_reachable = stubs.get("rekor_reachable", True)
    if not rekor_reachable:
        reasons.append("rekor_unreachable")

    # SBOM gate.
    sbom = stubs.get("cosign_verify_attestation_spdx", {}) or {}
    if "unknown_registry" not in reasons and not sbom.get("ok"):
        reasons.append("missing_sbom")

    # Model card gate.
    card = stubs.get("oras_discover_model_card", {}) or {}
    if "unknown_registry" not in reasons and not card.get("present"):
        reasons.append("missing_model_card")

    # CVE delta gate.
    if "unknown_registry" not in reasons:
        cand = stubs.get("grype_candidate", {}) or {}
        prod = stubs.get("grype_prod", {}) or {}
        new_critical = max(0, cand.get("critical", 0) - prod.get("critical", 0))
        new_high = max(0, cand.get("high", 0) - prod.get("high", 0))
        if new_critical > 0 or new_high > 0:
            reasons.append("cve_regression")

    # Stale attestation gate.
    ttl = inputs.get("attestation_ttl_hours", 24)
    if "unknown_registry" not in reasons and stubs.get("attestation_age_hours", 0) > ttl:
        reasons.append("stale_attestation")

    # Verdict resolution.
    if not reasons:
        verdict = "promote"
        reasons = ["all_gates_passed"]
        evidence_level = "live"
    elif reasons == ["rekor_unreachable"]:
        verdict = "manual-review"
        evidence_level = "partial"
    else:
        verdict = "block"
        evidence_level = "partial" if "rekor_unreachable" in reasons else "live"

    cand_grype = stubs.get("grype_candidate", {}) or {}
    prod_grype = stubs.get("grype_prod", {}) or {}
    attestation = {
        "attestation_version": "1.0.0",
        "agent": {
            "id": "nvidia-model-promotion-gatekeeper-agent",
            "version": "0.1.0",
            "execution_tier": "read-only-runtime",
        },
        "subject": {
            "image_ref": image_ref,
            "registry": image_ref.split("/", 1)[0] if "/" in image_ref else image_ref,
            "resolved_digest": stubs.get("crane_digest") if "unknown_registry" not in reasons else None,
            "current_prod_digest": inputs.get("current_prod_digest"),
        },
        "claims": {
            "signature": {
                "verified": bool(sig.get("ok")) and "wrong_identity" not in reasons
                            and "wrong_issuer" not in reasons and "expired_cert" not in reasons,
                "signer_identity": sig.get("signer_identity", ""),
                "issuer": sig.get("issuer", ""),
                "cert_not_after": sig.get("cert_not_after", "1970-01-01T00:00:00Z"),
                "rekor_log_index": sig.get("rekor_log_index", 0),
            },
            "sbom": {
                "present": bool(sbom.get("ok")),
                "format": sbom.get("format", "unknown"),
                "sha256": sbom.get("sha256", ""),
            },
            "model_card": {
                "present": bool(card.get("present")),
                "sha256": card.get("sha256", ""),
                "source": card.get("source", "missing"),
            },
            "cve_delta": {
                "vs_digest": inputs.get("current_prod_digest", ""),
                "new_critical": max(0, cand_grype.get("critical", 0) - prod_grype.get("critical", 0)),
                "new_high": max(0, cand_grype.get("high", 0) - prod_grype.get("high", 0)),
                "fixed_critical": max(0, prod_grype.get("critical", 0) - cand_grype.get("critical", 0)),
                "regressed": "cve_regression" in reasons,
            },
        },
        "evidence_level": evidence_level,
        "verdict": verdict,
        "verdict_reasons": reasons,
        "provenance": {
            "executed_commands": _scrub_commands(stubs.get("executed_commands", [])),
            "egress_hosts_contacted": stubs.get("egress_hosts_contacted",
                                                ["nvcr.io", "rekor.sigstore.dev", "fulcio.sigstore.dev"]
                                                if rekor_reachable and "unknown_registry" not in reasons
                                                else []),
            "runtime_mode": inputs.get("mode", "static"),
            "harness": "claude-code",
            "operator": "fixture-replay",
        },
        "timestamp": NOW.isoformat().replace("+00:00", "Z"),
        "nonce": "fixture-replay-nonce-1234567890",
    }

    # Drop None subject fields so the schema accepts the doc.
    attestation["subject"] = {k: v for k, v in attestation["subject"].items() if v is not None}
    return attestation


def _scrub_commands(cmds: list[str]) -> list[str]:
    return [SECRET_FLAG_RE.sub(lambda m: f"{m.group(1)}=<REDACTED>", c) for c in cmds]


def _load_schema():
    try:
        import jsonschema  # noqa: F401
    except ImportError:
        print("WARN: jsonschema not installed; skipping schema validation step.", file=sys.stderr)
        return None
    return json.loads(SCHEMA_PATH.read_text(encoding="utf-8"))


def main() -> int:
    if not FIXTURE_DIR.is_dir():
        print(f"ERROR: fixture dir not found: {FIXTURE_DIR}", file=sys.stderr)
        return 2

    schema = _load_schema()
    fixtures = sorted(p for p in FIXTURE_DIR.glob("*.json") if p.parent == FIXTURE_DIR)
    if not fixtures:
        print("ERROR: no fixtures found", file=sys.stderr)
        return 2

    failures = 0
    for fp in fixtures:
        fixture = json.loads(fp.read_text(encoding="utf-8"))
        name = fixture.get("name", fp.stem)
        expected = json.loads((EXPECTED_DIR / f"{name}.json").read_text(encoding="utf-8"))

        attestation = evaluate(fixture)

        # Schema check (optional dep).
        if schema is not None:
            from jsonschema import Draft202012Validator, ValidationError
            try:
                Draft202012Validator(schema).validate(attestation)
            except ValidationError as e:
                print(f"FAIL [{name}] schema: {e.message}")
                failures += 1
                continue

        # Verdict + reasons (set-equal on reasons).
        v_ok = attestation["verdict"] == expected["verdict"]
        r_ok = set(attestation["verdict_reasons"]) == set(expected["verdict_reasons"])
        ev_ok = attestation["evidence_level"] == expected["evidence_level"]
        if v_ok and r_ok and ev_ok:
            print(f"OK   [{name}] verdict={attestation['verdict']} reasons={attestation['verdict_reasons']}")
        else:
            print(f"FAIL [{name}] got verdict={attestation['verdict']} reasons={attestation['verdict_reasons']} "
                  f"evidence={attestation['evidence_level']} | "
                  f"expected verdict={expected['verdict']} reasons={expected['verdict_reasons']} "
                  f"evidence={expected['evidence_level']}")
            failures += 1

    if failures:
        print(f"\n{failures} fixture(s) failed", file=sys.stderr)
        return 1
    print(f"\nOK: {len(fixtures)} fixtures validated")
    return 0


if __name__ == "__main__":
    sys.exit(main())
