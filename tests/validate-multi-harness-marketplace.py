#!/usr/bin/env python3
"""Validate Cursor plugin manifest and GitHub Copilot CLI marketplace.

The Claude Code plugin manifest has its own validator
(tests/validate-plugin-manifest.py). This validator covers the other
two harnesses that ship a plugin/marketplace manifest:

  - Cursor: .cursor-plugin/plugin.json
      Source: cursor.com/docs/reference/plugins
      Required: name. Custom paths for agents/skills/rules supported.
      We enumerate cursor adapter paths explicitly and check that every
      catalog agent with `harnesses: [cursor, ...]` is represented.

  - GitHub Copilot CLI: .github/plugin/marketplace.json
      Source: copilot-cli docs (context7 /github/copilot-cli) and
      GitHub Docs ("Creating a plugin marketplace for GitHub Copilot CLI").
      Required: plugins[] array with id + source + description.
      We declare the repo as a single plugin with source: "./".

Gates:
  1. Both manifests exist and parse as JSON.
  2. Cursor manifest: every path resolves, version matches package.json,
     every cursor-enabled catalog agent is represented.
  3. Cursor generator (scripts/generate-cursor-plugin.mjs) is in sync.
  4. Copilot marketplace: declares vanguard-frontier-agentic plugin with
     source "./"; version matches package.json.
"""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
CURSOR_MANIFEST = REPO / ".cursor-plugin" / "plugin.json"
COPILOT_MARKETPLACE = REPO / ".github" / "plugin" / "marketplace.json"
CATALOG = REPO / "catalog" / "agents.json"
PKG = REPO / "package.json"
CURSOR_GENERATOR = REPO / "scripts" / "generate-cursor-plugin.mjs"


def fail(msg: str) -> None:
    print(f"FAIL [multi-harness-marketplace] {msg}", file=sys.stderr)


def path_is_inside_repo(path_value: str) -> bool:
    # Note 1: Cursor's manifest is a package boundary. A path that leaves the
    # repository is not just a broken link; it changes what the installer may
    # load as trusted plugin content.
    try:
        resolved = (REPO / path_value).resolve()
    except OSError:
        # Note 2: Treat filesystem resolution problems as validation failures
        # instead of trying to guess the installer's behavior.
        return False
    # Note 3: Checking parents after resolve() catches traversal attempts such
    # as "../outside.agent.md" even when the target file exists.
    return resolved == REPO or REPO in resolved.parents


def validate_cursor(pkg: dict, catalog: list) -> list[str]:
    errors: list[str] = []
    if not CURSOR_MANIFEST.exists():
        return [".cursor-plugin/plugin.json is missing"]

    manifest = json.loads(CURSOR_MANIFEST.read_text())

    if manifest.get("name") != "vanguard-frontier-agentic":
        errors.append("cursor plugin name must be 'vanguard-frontier-agentic'")
    if manifest.get("version") != pkg.get("version"):
        errors.append(
            f"cursor plugin version {manifest.get('version')!r} != package.json {pkg.get('version')!r}",
        )

    manifest_paths = manifest.get("agents") or []
    # Note 4: Containment comes before existence for the same reason as in the
    # Claude validator: an outside path can exist locally and still be an
    # invalid plugin artifact.
    escaping = [p for p in manifest_paths if not isinstance(p, str) or not path_is_inside_repo(p)]
    if escaping:
        errors.append(
            f"{len(escaping)} cursor manifest paths escape the repository: e.g. {escaping[0]}",
        )
    missing = [p for p in manifest_paths if p not in escaping and not (REPO / p).is_file()]
    if missing:
        errors.append(
            f"{len(missing)} cursor manifest paths do not resolve: e.g. {missing[0]}",
        )

    catalog_paths = set()
    for entry in catalog:
        if entry.get("type") != "agent":
            continue
        if "cursor" not in (entry.get("harnesses") or []):
            continue
        adapter = (entry.get("harness_variants") or {}).get(
            "cursor",
        ) or f"{entry['path']}/harnesses/cursor.agent.md"
        # Note 5: This validates the source of truth, not only the generated
        # output, so a stale manifest cannot mask unsafe catalog metadata.
        if not path_is_inside_repo(adapter):
            errors.append(f"{entry.get('id', '<unknown>')}: cursor adapter path escapes repository: {adapter}")
            continue
        catalog_paths.add(f"./{adapter}")

    manifest_set = set(manifest_paths)
    dropped = catalog_paths - manifest_set
    extra = manifest_set - catalog_paths
    if dropped:
        errors.append(
            f"{len(dropped)} cursor-enabled catalog agents absent from cursor manifest; e.g. {sorted(dropped)[0]}",
        )
    if extra:
        errors.append(
            f"{len(extra)} paths in cursor manifest not in catalog; e.g. {sorted(extra)[0]}",
        )

    # Generator drift
    result = subprocess.run(
        ["node", str(CURSOR_GENERATOR), "--check"],
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        errors.append((result.stderr or result.stdout).strip())

    return errors


def validate_copilot(pkg: dict) -> list[str]:
    errors: list[str] = []
    if not COPILOT_MARKETPLACE.exists():
        return [".github/plugin/marketplace.json is missing"]

    marketplace = json.loads(COPILOT_MARKETPLACE.read_text())

    if marketplace.get("name") != "vanguard-frontier-agentic":
        errors.append("copilot marketplace name must be 'vanguard-frontier-agentic'")
    if marketplace.get("version") != pkg.get("version"):
        errors.append(
            f"copilot marketplace version {marketplace.get('version')!r} != package.json {pkg.get('version')!r}",
        )

    plugins = marketplace.get("plugins") or []
    if not plugins:
        errors.append("copilot marketplace must declare at least one plugin")

    vfa_plugin = next(
        (p for p in plugins if p.get("id") == "vanguard-frontier-agentic"),
        None,
    )
    if vfa_plugin is None:
        errors.append("copilot marketplace must declare 'vanguard-frontier-agentic' plugin")
    else:
        if vfa_plugin.get("source") != "./":
            errors.append(
                "copilot plugin source must be './' so the repo root is the plugin root",
            )
        if not vfa_plugin.get("description"):
            errors.append("copilot plugin requires a non-empty description")

    return errors


def main() -> int:
    pkg = json.loads(PKG.read_text())
    catalog = json.loads(CATALOG.read_text())

    errors: list[str] = []
    errors.extend(validate_cursor(pkg, catalog))
    errors.extend(validate_copilot(pkg))

    if errors:
        for err in errors:
            fail(err)
        return 1

    cursor_count = len(json.loads(CURSOR_MANIFEST.read_text()).get("agents") or [])
    print(
        f"OK: cursor plugin valid ({cursor_count} agents) and copilot marketplace valid (1 plugin)",
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
