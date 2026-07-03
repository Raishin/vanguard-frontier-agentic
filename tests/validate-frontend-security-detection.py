#!/usr/bin/env python3
"""Detection-proof gate for frontend `category: security` skills.

WHAT THIS GATE IS
-----------------
A deterministic, LLM-free regression lint for the concrete-sink security
skills under `skills/frontend/`. For each covered skill it enforces two
independent guarantees against a hand-authored fixture corpus at
`tests/fixtures/frontend-security-detection/<skill-id>/`:

  1. KEYWORD-LINKAGE LINT. Every sink keyword a detector claims the skill
     documents (e.g. `dangerouslySetInnerHTML`, `v-html`, `runtimeConfig.public`,
     `localStorage`, `HttpOnly`) must still appear VERBATIM in that skill's
     SKILL.md. This catches the failure mode where a prose/wording edit silently
     drops a documented sink and the skill quietly rots — the exact regression a
     taxonomy-only gate (reading a parallel regex file, never the skill prose)
     cannot see.

  2. RED/GREEN SMOKE TEST. Each detector's pattern must FIRE on a known-vulnerable
     fixture (`red/`) and stay SILENT on a safe-idiom lookalike (`green/`). This
     proves the declared pattern is non-empty, compiles, and distinguishes the
     dangerous construct from the safe alternative.

WHAT THIS GATE DOES **NOT** PROVE (read this before quoting it to anyone)
------------------------------------------------------------------------
  * It does NOT prove the LLM review agent detects anything. It lints the skill's
    declared sink lexicon and pattern smoke-health, not model behavior.
  * It does NOT prove detection GENERALIZES beyond these hand-picked fixtures.
  * It does NOT do dataflow. A regex cannot tell `DOMPurify.sanitize(x)` on one
    line from an unsanitized `innerHTML = y` three lines down. Any "recall/precision"
    figure derived from this corpus is CORPUS-LOCAL and must never appear in a
    buyer-facing or field-accuracy claim. Green fixtures are SAFE IDIOMS the
    pattern legitimately does not match — not proof of no-false-positives on
    real code.

In short: this is a keyword-regression lint + pattern smoke test. It prevents
silent rot and broken patterns. It is not a detection-accuracy benchmark, and is
documented as such on purpose.

CREATE-TIME GATE
----------------
Every frontend skill with `category: security` must EITHER ship a `detectors.json`
fixture dir OR be listed in EXEMPT below with a reason. This forces every future
security skill (PCI, GraphQL, React RSC, Next middleware, ...) to land with its
red/green corpus instead of as unproven prose.
"""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CATALOG_SKILLS = ROOT / "catalog" / "skills.json"
FIXTURE_ROOT = ROOT / "tests" / "fixtures" / "frontend-security-detection"

# Security-category frontend skills that are methodologies/architecture reviews
# with NO fixed, greppable sink lexicon to lint. Exemptions are explicit and
# reasoned — never silent — so the gate cannot rot by accident.
EXEMPT: dict[str, str] = {
    "enterprise-red-team-review": (
        "Adversarial red-team methodology skill: it teaches how to attack, not a "
        "fixed set of code sinks to flag. There is no stable sink lexicon to lint, "
        "so a red/green detector corpus would be fabricated proof."
    ),
}


def _fail(errors: list[str], msg: str) -> None:
    errors.append(msg)


def _security_frontend_skills() -> list[dict]:
    skills = json.loads(CATALOG_SKILLS.read_text(encoding="utf-8"))
    out = []
    for s in skills:
        if s.get("provider") != "frontend":
            continue
        skill_md = ROOT / s["path"] / "SKILL.md"
        if not skill_md.exists():
            continue
        m = re.search(r"^\s*category:\s*([A-Za-z0-9_-]+)\s*$", skill_md.read_text(encoding="utf-8"), re.MULTILINE)
        if m and m.group(1) == "security":
            out.append(s)
    return out


def _check_detector(skill_id: str, sdir: Path, skill_md_text: str, det: dict, errors: list[str]) -> None:
    did = det.get("id", "<no-id>")
    tag = f"{skill_id}:{did}"

    # --- structural ---
    regex_src = det.get("regex")
    if not isinstance(regex_src, str) or regex_src == "":
        _fail(errors, f"{tag}: detector 'regex' missing or empty")
        return
    try:
        pat = re.compile(regex_src)
    except re.error as e:
        _fail(errors, f"{tag}: regex does not compile: {e}")
        return
    # Anti-vacuity: a pattern that matches the empty string matches everything.
    if pat.search("") is not None:
        _fail(errors, f"{tag}: regex matches the empty string (vacuous) — tighten it")
        return

    # --- 1) keyword-linkage lint ---
    kws = det.get("skill_keywords")
    if not isinstance(kws, list) or not kws:
        _fail(errors, f"{tag}: 'skill_keywords' must be a non-empty list of literals that appear in SKILL.md")
    else:
        for kw in kws:
            if not isinstance(kw, str) or kw == "":
                _fail(errors, f"{tag}: skill_keywords entry must be a non-empty string")
            elif kw not in skill_md_text:
                _fail(errors, f"{tag}: documented sink keyword {kw!r} is NOT present verbatim in {skill_id}/SKILL.md (prose rot)")

    # --- 2) red/green smoke test ---
    for kind, must_match in (("red", True), ("green", False)):
        rel = det.get(kind)
        if not isinstance(rel, str) or not rel:
            _fail(errors, f"{tag}: detector '{kind}' fixture path missing")
            continue
        fpath = sdir / rel
        if not fpath.exists():
            _fail(errors, f"{tag}: {kind} fixture not found: {fpath.relative_to(ROOT)}")
            continue
        content = fpath.read_text(encoding="utf-8")
        if content.strip() == "":
            _fail(errors, f"{tag}: {kind} fixture is empty")
            continue
        hit = pat.search(content) is not None
        if must_match and not hit:
            _fail(errors, f"{tag}: pattern did NOT fire on red fixture {rel} (detector cannot catch its own known-vulnerable case)")
        if (not must_match) and hit:
            _fail(errors, f"{tag}: pattern FIRED on green fixture {rel} (flags the safe idiom — pattern too coarse or green mis-authored)")

    # red and green must be genuinely different artifacts
    red_rel, green_rel = det.get("red"), det.get("green")
    if isinstance(red_rel, str) and isinstance(green_rel, str):
        rp, gp = sdir / red_rel, sdir / green_rel
        if rp.exists() and gp.exists() and rp.read_text(encoding="utf-8") == gp.read_text(encoding="utf-8"):
            _fail(errors, f"{tag}: red and green fixtures are byte-identical")


def main() -> int:
    errors: list[str] = []
    sec_skills = _security_frontend_skills()
    sec_ids = {s["id"] for s in sec_skills}
    path_by_id = {s["id"]: s["path"] for s in sec_skills}

    covered = 0
    detector_total = 0

    # Create-time gate: every security-category frontend skill is covered or exempt.
    for s in sec_skills:
        sid = s["id"]
        sdir = FIXTURE_ROOT / sid
        det_json = sdir / "detectors.json"
        if not det_json.exists():
            if sid in EXEMPT:
                continue
            _fail(errors, f"{sid}: category:security frontend skill has no detector corpus "
                          f"(add tests/fixtures/frontend-security-detection/{sid}/detectors.json "
                          f"or add a reasoned EXEMPT entry)")
            continue
        if sid in EXEMPT:
            _fail(errors, f"{sid}: listed in EXEMPT but also ships a detector corpus — remove one")

        try:
            spec = json.loads(det_json.read_text(encoding="utf-8"))
        except json.JSONDecodeError as e:
            _fail(errors, f"{sid}: detectors.json is not valid JSON: {e}")
            continue
        if spec.get("skill_id") != sid:
            _fail(errors, f"{sid}: detectors.json skill_id {spec.get('skill_id')!r} does not match its directory")
        dets = spec.get("detectors")
        if not isinstance(dets, list) or not dets:
            _fail(errors, f"{sid}: detectors.json must declare a non-empty 'detectors' list")
            continue
        skill_md_text = (ROOT / path_by_id[sid] / "SKILL.md").read_text(encoding="utf-8")
        covered += 1
        for det in dets:
            detector_total += 1
            _check_detector(sid, sdir, skill_md_text, det, errors)

    # Reverse check: no orphan fixture dirs pointing at non-security / non-existent skills.
    if FIXTURE_ROOT.exists():
        for child in sorted(FIXTURE_ROOT.iterdir()):
            if child.is_dir() and child.name not in sec_ids:
                _fail(errors, f"{child.name}: fixture dir does not correspond to a category:security frontend skill")

    if errors:
        print(f"FAIL: frontend-security detection gate ({len(errors)} problem(s)):", file=sys.stderr)
        for e in errors:
            print(f"  - {e}", file=sys.stderr)
        return 1

    exempt_note = f", {len(EXEMPT)} exempt (methodology)" if EXEMPT else ""
    print(f"OK: frontend-security detection gate — {covered} skill(s) covered, "
          f"{detector_total} detector(s) linted (keyword-linkage + red/green smoke){exempt_note}. "
          f"Deterministic lint only; does NOT benchmark LLM detection accuracy (see docstring).")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
