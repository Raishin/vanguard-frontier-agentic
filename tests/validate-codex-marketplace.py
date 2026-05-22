#!/usr/bin/env python3
"""Validate the Codex marketplace + plugins.

Per the Codex docs (context7 /openai/codex,
github.com/openai/codex/blob/main/codex-rs/skills/src/assets/samples/
plugin-creator/references/plugin-json-spec.md):

  - Marketplace registry file: <repo-root>/.agents/plugins/marketplace.json
  - Plugin manifest file: <plugin-root>/.codex-plugin/plugin.json
  - Plugin folder name MUST equal plugin.json `name` (kebab-case)
  - marketplace.json plugins[] entries MUST include `policy.installation`,
    `policy.authentication`, and `category` — even at defaults

Install command (verified via context7):
  codex plugin marketplace add Raishin/vanguard-frontier-agentic

Gates:
  1. .agents/plugins/marketplace.json exists and is well-formed.
  2. Declares 'vanguard-frontier-agentic' as marketplace name.
  3. Every plugins[] entry has: name, source.{source, path}, policy.{
     installation, authentication}, category.
  4. Every referenced plugin path has .codex-plugin/plugin.json.
  5. Every plugin.json has required fields (name, version, description)
     and name is kebab-case and matches the plugin folder name.
  6. The 'vanguard-frontier-agentic' plugin's version matches package.json.
"""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
MARKETPLACE = REPO / ".agents" / "plugins" / "marketplace.json"
PKG = REPO / "package.json"

KEBAB_RE = re.compile(r"^[a-z][a-z0-9-]*$")
ALLOWED_INSTALL = {"NOT_AVAILABLE", "AVAILABLE", "INSTALLED_BY_DEFAULT", "DISABLED", "HIDDEN"}
ALLOWED_AUTH = {"ON_INSTALL", "ON_USE", "OPTIONAL", "NONE"}


def fail(msg: str) -> None:
    print(f"FAIL [codex-marketplace] {msg}", file=sys.stderr)


def main() -> int:
    if not MARKETPLACE.exists():
        fail(".agents/plugins/marketplace.json is missing")
        return 1

    pkg = json.loads(PKG.read_text())
    marketplace = json.loads(MARKETPLACE.read_text())

    errors: list[str] = []

    if marketplace.get("name") != "vanguard-frontier-agentic":
        errors.append("marketplace name must be 'vanguard-frontier-agentic'")

    plugins = marketplace.get("plugins") or []
    if not plugins:
        errors.append("marketplace must declare at least one plugin")

    names_seen = set()
    for idx, plugin in enumerate(plugins):
        prefix = f"plugins[{idx}]"
        name = plugin.get("name")
        if not name or not isinstance(name, str):
            errors.append(f"{prefix}: missing or non-string name")
            continue
        if not KEBAB_RE.match(name):
            errors.append(f"{prefix}: name {name!r} must be lowercase kebab-case")
        if name in names_seen:
            errors.append(f"{prefix}: duplicate plugin name {name!r}")
        names_seen.add(name)

        source = plugin.get("source") or {}
        if source.get("source") != "local":
            errors.append(f"{prefix} ({name}): source.source must be 'local' for in-repo plugins")
        path = source.get("path")
        if not path or not isinstance(path, str):
            errors.append(f"{prefix} ({name}): source.path is required")
            continue

        # The path is relative to the marketplace root. Marketplace root is
        # the directory containing .agents/, which is REPO.
        plugin_dir = (REPO / path).resolve()
        if not plugin_dir.is_dir():
            errors.append(f"{prefix} ({name}): source path {path!r} does not resolve to a directory")
            continue

        if plugin_dir.name != name:
            errors.append(
                f"{prefix} ({name}): plugin folder name {plugin_dir.name!r} must equal plugin name {name!r} (Codex spec)",
            )

        plugin_manifest = plugin_dir / ".codex-plugin" / "plugin.json"
        if not plugin_manifest.is_file():
            errors.append(
                f"{prefix} ({name}): {plugin_manifest.relative_to(REPO)} is missing",
            )
            continue

        try:
            manifest = json.loads(plugin_manifest.read_text())
        except json.JSONDecodeError as e:
            errors.append(f"{prefix} ({name}): plugin.json is not valid JSON: {e}")
            continue

        for required in ("name", "version", "description"):
            if not manifest.get(required):
                errors.append(f"{prefix} ({name}): plugin.json missing required field {required!r}")
        if manifest.get("name") != name:
            errors.append(
                f"{prefix} ({name}): plugin.json name {manifest.get('name')!r} must equal marketplace entry name {name!r}",
            )

        # Version parity and install-surface checks for the primary plugin.
        if name == "vanguard-frontier-agentic":
            if manifest.get("version") != pkg.get("version"):
                errors.append(
                    f"{prefix} ({name}): plugin.json version {manifest.get('version')!r} != package.json {pkg.get('version')!r}",
                )
            if manifest.get("skills") != "./skills/":
                errors.append(
                    f"{prefix} ({name}): plugin.json must declare skills './skills/' for the bundled Codex install skill",
                )
            install_skill = plugin_dir / "skills" / "vanguard-frontier-agentic-install" / "SKILL.md"
            if not install_skill.is_file():
                errors.append(
                    f"{prefix} ({name}): bundled install skill is missing at {install_skill.relative_to(REPO)}",
                )

        skills_path = manifest.get("skills")
        if skills_path is not None:
            if not isinstance(skills_path, str) or not skills_path.startswith("./"):
                errors.append(f"{prefix} ({name}): plugin.json skills path must be a './'-prefixed string")
            else:
                skills_dir = (plugin_dir / skills_path).resolve()
                if not str(skills_dir).startswith(str(plugin_dir.resolve())):
                    errors.append(f"{prefix} ({name}): plugin.json skills path must stay inside plugin root")
                elif not skills_dir.is_dir():
                    errors.append(f"{prefix} ({name}): plugin.json skills directory {skills_path!r} is missing")
                elif not any(skills_dir.glob("*/SKILL.md")):
                    errors.append(f"{prefix} ({name}): plugin.json skills directory has no */SKILL.md entries")

        policy = plugin.get("policy") or {}
        install = policy.get("installation")
        auth = policy.get("authentication")
        if install not in ALLOWED_INSTALL:
            errors.append(
                f"{prefix} ({name}): policy.installation must be one of {sorted(ALLOWED_INSTALL)}, got {install!r}",
            )
        if auth not in ALLOWED_AUTH:
            errors.append(
                f"{prefix} ({name}): policy.authentication must be one of {sorted(ALLOWED_AUTH)}, got {auth!r}",
            )
        if not plugin.get("category"):
            errors.append(f"{prefix} ({name}): category is required (Codex marketplace spec)")

    if errors:
        for err in errors:
            fail(err)
        return 1

    print(
        f"OK: codex marketplace valid ({len(plugins)} plugins declared, vanguard-frontier-agentic version {pkg.get('version')})",
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
