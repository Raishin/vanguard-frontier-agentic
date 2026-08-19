#!/usr/bin/env python3
"""Upsert agent and skill metadata.json entries into the catalog JSON files.

Adds a catalog entry for any agent/skill whose id is missing, AND refreshes the
cataloged fields of any id whose adjacent ``metadata.json`` has since diverged
(summary, official_docs, security_notes, harnesses, version, companion_skills,
…). Runs as a strict no-op when the catalog already matches every metadata.json,
so it is safe to re-run at any point in the generation workflow.

It does NOT prune: a catalog entry whose ``metadata.json`` was deleted is left
untouched — removal is a deliberate, separate operation, never a side effect of
a sync.

Scope it. ``--provider <id>`` (repeatable) restricts the sync to one provider's
assets and is what a board generator should document as its regeneration step::

    python3 scripts/update-catalog-new-agents.py --provider typescript

An unscoped run walks every provider, and the merge below is
``{**catalog_entry, **projected_metadata}`` — projected keys win. That is
correct when ``metadata.json`` is the newer side, but it is not always: some
committed metadata files are *older* than the catalog and understate what is on
disk (e.g. ionos/ovhcloud/scaleway agents declare two harnesses each while all
seven adapter files exist beside them). An unscoped run silently rewrites those
catalog entries from the stale side, which then propagates into every generated
inventory downstream — the Kiro Powers set is derived from cataloged harnesses,
so whole providers can drop out of it. No gate catches that.

So: scope every routine run to what you actually changed. Reserve the unscoped
run for a deliberate, reviewed catalog reconciliation, and read its full diff
before committing it.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

CATALOG_AGENTS = ROOT / "catalog" / "agents.json"
CATALOG_SKILLS = ROOT / "catalog" / "skills.json"

def metadata_to_catalog_entry(m: dict, kind: str) -> dict:
    entry: dict = {}
    for key in ("id", "name", "type", "provider", "harnesses", "summary",
                "source_type", "official_docs", "security_notes",
                "last_verified", "path", "version", "execution_tier",
                # Lifecycle is load-bearing for consumers, not decoration: an asset
                # marked deprecated in its metadata but absent from the catalog reads
                # as current to everything that only sees catalog/agents.json.
                "lifecycle"):
        if key in m:
            entry[key] = m[key]
    # Preserve agent→skill edges so catalog consumers (e.g. the TUI dependency
    # graph, which reads catalog/agents.json directly) see the same companion
    # linkage as the adjacent metadata.json.
    if kind == "agent" and "companion_skills" in m:
        entry["companion_skills"] = m["companion_skills"]
    # Normalise path — strip trailing slash
    if "path" in entry and isinstance(entry["path"], str):
        entry["path"] = entry["path"].rstrip("/")
    if kind == "skill" and "author" in m:
        entry["author"] = m["author"]
    return entry


def sync_catalog(catalog: list[dict], glob_pat: str, kind: str) -> tuple[list[str], list[str]]:
    """Upsert projected metadata entries into ``catalog`` in place.

    New ids are appended; existing ids are refreshed only when their projected
    form differs from what the catalog already holds (dict equality ignores key
    order, so a re-sync fires on a real value change, never on formatting). An
    already-synced tree yields ``([], [])`` and leaves ``catalog`` untouched.
    """
    by_id = {e["id"]: e for e in catalog}
    added: list[str] = []
    updated: list[str] = []
    for meta_path in sorted(ROOT.glob(glob_pat)):
        m = json.loads(meta_path.read_text(encoding="utf-8"))
        if m.get("type") != kind:
            continue
        entry = metadata_to_catalog_entry(m, kind)
        cur = by_id.get(m["id"])
        if cur is None:
            catalog.append(entry)
            by_id[entry["id"]] = entry
            added.append(entry["id"])
        else:
            # Merge, never clobber. The projection above covers only the metadata-sourced
            # fields; a committed catalog entry legitimately carries additional ones that no
            # metadata.json supplies (author, harness_variants, lifecycle, category,
            # oauth_scopes, mcp_servers, run_as_permissions, companion_agents, …). A
            # clear()+update() "true sync" therefore DELETED those on every re-run — it
            # rewrote hundreds of untouched entries and dropped fields that
            # tests/validate-catalog.py requires, so the very next `npm run validate`
            # failed. Projected keys win; unmanaged keys are preserved, which is what makes
            # the documented "strict no-op when already in sync" actually true.
            merged = {**cur, **entry}
            if merged != cur:
                cur.clear()
                cur.update(merged)
                updated.append(entry["id"])
    if added or updated:
        catalog.sort(key=lambda x: x["id"])
    return added, updated


def _write(catalog: list[dict], path: Path) -> None:
    path.write_text(
        json.dumps(catalog, indent=2, ensure_ascii=False) + "\n",
        encoding="utf-8",
    )


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument(
        "--provider",
        action="append",
        metavar="ID",
        help="Restrict the sync to one provider's assets (repeatable). Omit to "
             "walk every provider — see the module docstring before doing that.",
    )
    args = parser.parse_args()

    agents_catalog: list[dict] = json.loads(CATALOG_AGENTS.read_text(encoding="utf-8"))
    skills_catalog: list[dict] = json.loads(CATALOG_SKILLS.read_text(encoding="utf-8"))

    if args.provider:
        agent_globs = [f"agents/{p}/*/metadata.json" for p in args.provider]
        skill_globs = [f"skills/{p}/*/metadata.json" for p in args.provider]
        print(f"Scoped to provider(s): {', '.join(sorted(args.provider))}")
    else:
        agent_globs = ["agents/**/metadata.json"]
        skill_globs = ["skills/**/metadata.json"]
        print(
            "WARNING: unscoped run — every provider is in scope, and a stale "
            "metadata.json will overwrite a newer catalog entry. Read the full "
            "diff before committing. Use --provider <id> for routine runs."
        )

    a_added: list[str] = []
    a_updated: list[str] = []
    for pat in agent_globs:
        added, updated = sync_catalog(agents_catalog, pat, "agent")
        a_added += added
        a_updated += updated

    s_added: list[str] = []
    s_updated: list[str] = []
    for pat in skill_globs:
        added, updated = sync_catalog(skills_catalog, pat, "skill")
        s_added += added
        s_updated += updated

    for kind, added, updated in (
        ("agent", a_added, a_updated),
        ("skill", s_added, s_updated),
    ):
        for entry_id in added:
            print(f"  + {kind}: {entry_id}")
        for entry_id in updated:
            print(f"  ~ {kind}: {entry_id} (metadata re-synced)")

    if a_added or a_updated:
        _write(agents_catalog, CATALOG_AGENTS)
        print(f"Wrote {len(agents_catalog)} agents to {CATALOG_AGENTS.relative_to(ROOT)}")
    else:
        print("Agents catalog already in sync.")

    if s_added or s_updated:
        _write(skills_catalog, CATALOG_SKILLS)
        print(f"Wrote {len(skills_catalog)} skills to {CATALOG_SKILLS.relative_to(ROOT)}")
    else:
        print("Skills catalog already in sync.")


if __name__ == "__main__":
    main()
