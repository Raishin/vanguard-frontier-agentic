#!/usr/bin/env python3
"""Validate the Claude Code plugin manifest and marketplace declaration.

Gates:
  1. .claude-plugin/marketplace.json is well-formed and declares the
     vanguard-frontier-agentic plugin with source: "./".
  2. .claude-plugin/plugin.json is in sync with catalog/agents.json
     (delegated to scripts/generate-plugin-manifest.mjs --check).
  3. Every path in plugin.json's `agents` array resolves to a real file.
  4. Every claude-code-enabled agent in the catalog is represented in the
     manifest — no silent drops.
  5. plugin.json `version` matches package.json `version` — otherwise
     installers see one number while the marketplace advertises another.
"""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
MARKETPLACE = REPO / ".claude-plugin" / "marketplace.json"
PLUGIN = REPO / ".claude-plugin" / "plugin.json"
CATALOG = REPO / "catalog" / "agents.json"
PKG = REPO / "package.json"


def fail(msg: str) -> None:
    print(f"FAIL [plugin-manifest] {msg}", file=sys.stderr)


def path_is_inside_repo(path_value: str) -> bool:
    # Note 1: Validation repeats the generator's containment rule because
    # generated JSON can be edited by hand. Validators should protect the
    # committed artifact, not only the generator path that normally writes it.
    try:
        resolved = (REPO / path_value).resolve()
    except OSError:
        # Note 2: Unresolvable paths are unsafe for manifest purposes. Returning
        # False keeps the caller's error reporting simple and fail-closed.
        return False
    # Note 3: Path.parents is a clear containment test after resolve() has
    # collapsed "." and ".." segments and followed normal filesystem rules.
    return resolved == REPO or REPO in resolved.parents


def main() -> int:
    if not MARKETPLACE.exists():
        fail(".claude-plugin/marketplace.json is missing")
        return 1
    if not PLUGIN.exists():
        fail(".claude-plugin/plugin.json is missing")
        return 1

    marketplace = json.loads(MARKETPLACE.read_text())
    plugin = json.loads(PLUGIN.read_text())
    catalog = json.loads(CATALOG.read_text())
    pkg = json.loads(PKG.read_text())

    errors: list[str] = []

    # Marketplace shape
    if marketplace.get("name") != "vanguard-frontier-agentic":
        errors.append("marketplace.name must be 'vanguard-frontier-agentic'")
    plugins = marketplace.get("plugins") or []
    if not any(p.get("name") == "vanguard-frontier-agentic" for p in plugins):
        errors.append("marketplace.plugins must declare 'vanguard-frontier-agentic'")
    for p in plugins:
        if p.get("name") == "vanguard-frontier-agentic" and p.get("source") != "./":
            errors.append(
                "marketplace plugin source must be './' so the plugin root is the repo root",
            )

    # Version parity
    if plugin.get("version") != pkg.get("version"):
        errors.append(
            f"plugin.json version {plugin.get('version')!r} does not match package.json {pkg.get('version')!r}",
        )

    # Every agent path resolves
    manifest_paths = plugin.get("agents") or []
    # Note 4: We check containment before existence. A malicious "../x" could
    # point to a real file on a maintainer machine, but it still must not be
    # publishable as a plugin manifest entry.
    escaping = [p for p in manifest_paths if not isinstance(p, str) or not path_is_inside_repo(p)]
    if escaping:
        errors.append(f"{len(escaping)} manifest paths escape the repository: e.g. {escaping[0]}")
    missing = [p for p in manifest_paths if p not in escaping and not (REPO / p).is_file()]
    if missing:
        errors.append(f"{len(missing)} manifest paths do not resolve: e.g. {missing[0]}")

    # Every claude-code-enabled catalog agent is in the manifest
    catalog_paths = set()
    for entry in catalog:
        if entry.get("type") != "agent":
            continue
        harnesses = entry.get("harnesses") or []
        if "claude-code" not in harnesses:
            continue
        adapter = (entry.get("harness_variants") or {}).get(
            "claude-code",
        ) or f"{entry['path']}/harnesses/claude-code.agent.md"
        # Note 5: Catalog entries and generated manifests are checked
        # independently so drift cannot hide a bad source path behind a
        # currently clean generated plugin.json.
        if not path_is_inside_repo(adapter):
            errors.append(f"{entry.get('id', '<unknown>')}: claude-code adapter path escapes repository: {adapter}")
            continue
        catalog_paths.add(f"./{adapter}")

    manifest_set = set(manifest_paths)
    dropped = catalog_paths - manifest_set
    extra = manifest_set - catalog_paths
    if dropped:
        errors.append(
            f"{len(dropped)} claude-code-enabled agents in catalog are absent from plugin.json: e.g. {sorted(dropped)[0]}",
        )
    if extra:
        errors.append(
            f"{len(extra)} paths in plugin.json are not in the catalog: e.g. {sorted(extra)[0]}",
        )

    # Generator drift — re-run with --check
    result = subprocess.run(
        ["node", str(REPO / "scripts" / "generate-plugin-manifest.mjs"), "--check"],
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        errors.append(result.stderr.strip() or result.stdout.strip())

    if errors:
        for err in errors:
            fail(err)
        return 1

    print(
        f"OK: plugin manifest valid ({len(manifest_paths)} claude-code agents declared, "
        f"version {plugin.get('version')})",
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
