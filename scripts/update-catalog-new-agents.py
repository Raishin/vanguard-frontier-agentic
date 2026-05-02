#!/usr/bin/env python3
"""Add all new agent and skill metadata.json entries to catalog JSON files."""

from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

CATALOG_AGENTS = ROOT / "catalog" / "agents.json"
CATALOG_SKILLS = ROOT / "catalog" / "skills.json"

CATALOG_FIELDS_AGENT = {
    "id", "name", "type", "provider", "summary", "path",
    "harnesses", "last_verified", "official_docs", "security_notes",
    "source_type", "version",
}
CATALOG_FIELDS_SKILL = CATALOG_FIELDS_AGENT | {"author"}


def metadata_to_catalog_entry(m: dict, kind: str) -> dict:
    entry: dict = {}
    for key in ("id", "name", "type", "provider", "harnesses", "summary",
                "source_type", "official_docs", "security_notes",
                "last_verified", "path", "version"):
        if key in m:
            entry[key] = m[key]
    # Normalise path — strip trailing slash
    if "path" in entry and isinstance(entry["path"], str):
        entry["path"] = entry["path"].rstrip("/")
    if kind == "skill" and "author" in m:
        entry["author"] = m["author"]
    return entry


def main() -> None:
    agents_catalog: list[dict] = json.loads(CATALOG_AGENTS.read_text(encoding="utf-8"))
    skills_catalog: list[dict] = json.loads(CATALOG_SKILLS.read_text(encoding="utf-8"))

    existing_agent_ids = {e["id"] for e in agents_catalog}
    existing_skill_ids = {e["id"] for e in skills_catalog}

    new_agents: list[dict] = []
    for meta_path in sorted(ROOT.glob("agents/**/metadata.json")):
        m = json.loads(meta_path.read_text(encoding="utf-8"))
        if m.get("type") != "agent":
            continue
        if m["id"] not in existing_agent_ids:
            entry = metadata_to_catalog_entry(m, "agent")
            new_agents.append(entry)
            print(f"  + agent: {entry['id']}")

    new_skills: list[dict] = []
    for meta_path in sorted(ROOT.glob("skills/**/metadata.json")):
        m = json.loads(meta_path.read_text(encoding="utf-8"))
        if m.get("type") != "skill":
            continue
        if m["id"] not in existing_skill_ids:
            entry = metadata_to_catalog_entry(m, "skill")
            new_skills.append(entry)
            print(f"  + skill: {entry['id']}")

    if new_agents:
        agents_catalog.extend(new_agents)
        agents_catalog.sort(key=lambda x: x["id"])
        CATALOG_AGENTS.write_text(
            json.dumps(agents_catalog, indent=2, ensure_ascii=False) + "\n",
            encoding="utf-8",
        )
        print(f"\nWrote {len(agents_catalog)} agents to {CATALOG_AGENTS.relative_to(ROOT)}")
    else:
        print("No new agents to add.")

    if new_skills:
        skills_catalog.extend(new_skills)
        skills_catalog.sort(key=lambda x: x["id"])
        CATALOG_SKILLS.write_text(
            json.dumps(skills_catalog, indent=2, ensure_ascii=False) + "\n",
            encoding="utf-8",
        )
        print(f"Wrote {len(skills_catalog)} skills to {CATALOG_SKILLS.relative_to(ROOT)}")
    else:
        print("No new skills to add.")


if __name__ == "__main__":
    main()
