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
"""

from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

CATALOG_AGENTS = ROOT / "catalog" / "agents.json"
CATALOG_SKILLS = ROOT / "catalog" / "skills.json"

CATALOG_FIELDS_AGENT = {
    "id", "name", "type", "provider", "summary", "path",
    "harnesses", "last_verified", "official_docs", "security_notes",
    "source_type", "version", "companion_skills",
}
CATALOG_FIELDS_SKILL = CATALOG_FIELDS_AGENT | {"author"}


def metadata_to_catalog_entry(m: dict, kind: str) -> dict:
    entry: dict = {}
    for key in ("id", "name", "type", "provider", "harnesses", "summary",
                "source_type", "official_docs", "security_notes",
                "last_verified", "path", "version"):
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
        elif cur != entry:
            # Replace contents in place so the canonical key order is restored
            # and any dropped field is removed — a true sync, not a merge.
            cur.clear()
            cur.update(entry)
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
    agents_catalog: list[dict] = json.loads(CATALOG_AGENTS.read_text(encoding="utf-8"))
    skills_catalog: list[dict] = json.loads(CATALOG_SKILLS.read_text(encoding="utf-8"))

    a_added, a_updated = sync_catalog(agents_catalog, "agents/**/metadata.json", "agent")
    s_added, s_updated = sync_catalog(skills_catalog, "skills/**/metadata.json", "skill")

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
