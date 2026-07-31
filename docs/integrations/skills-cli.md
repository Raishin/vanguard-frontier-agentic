# Skills CLI Integration Guide

This document covers the three supported install paths for Vanguard Frontier Agentic content, their trust postures, and verified command syntax. It is intended for engineers who want to understand what each path fetches, from where, and with what level of integrity assurance before running anything.

---

## Trust matrix

| Path | Source of truth | Versioning | Includes agents? | Includes skills? | Pinnable | Trust posture | Best for |
|------|----------------|------------|-----------------|-----------------|----------|---------------|----------|
| `npm i @raishin/vanguard-frontier-agentic` | npm registry (release pipeline) | SemVer, explicit | Yes (via exporter CLI) | Yes | Yes — lock to exact semver or lockfile | **Highest.** Published artifacts are signed via the release pipeline; you control the version in your lockfile. | Production teams, CI/CD enforcement, auditability |
| `npx vfa-export-agents` | npm registry (same package) | SemVer, explicit | Yes | Yes (default, opt out with `--no-skills`) | Yes — pin the package version | **High.** Same artifact as above; CLI is bundled in the npm package. | Engineers who want role-based or per-platform export with a single command |
| `npx skills add VincentChuWaiChow/vanguard-frontier-agentic` | GitHub raw HEAD (via `vercel-labs/skills` CLI) | None by default — fetches latest commit on the default branch | No (skills only) | Yes | Not documented — see [pinning note](#pinning-the-skills-cli-path) below | **Lower.** Third-party CLI pulls raw GitHub content without version pinning or integrity verification at install time. | Quick local exploration; not recommended for shared or CI environments |

---

## Path 1 — npm package (highest trust)

Install the versioned npm package. The exporter CLI (`vfa-export-agents`) is bundled in the package and handles agent and skill export.

```bash
# Install a specific version
npm install @raishin/vanguard-frontier-agentic@1.8.0

# Or install latest and lock via package-lock.json / npm-shrinkwrap.json
npm install @raishin/vanguard-frontier-agentic@latest
```

After install, run the exporter from the local package to avoid fetching a different version:

```bash
npx --no -- vfa-export-agents --platform claude-code --role cloud-security-engineer --repo .
```

See [`docs/release-versioning.md`](../release-versioning.md) for the versioning policy and the full SemVer bump table.

---

## Path 2 — vfa-export-agents CLI (high trust)

The exporter CLI is the canonical way to install agents and skills into a project. It resolves content from the same npm package artifact.

```bash
# See all available options
npx vfa-export-agents --help

# Discovery
npx vfa-export-agents --list             # every agent id
npx vfa-export-agents --list-roles       # every install role
npx vfa-export-agents --list-providers   # every distinct provider with agent counts

# Per-role install
npx vfa-export-agents --platform claude-code --role cloud-security-engineer --repo .

# Per-provider install (standalone — no --role required)
npx vfa-export-agents --platform claude-code --provider nvidia --repo .

# Role + provider filter (narrow a role to a single provider)
npx vfa-export-agents --platform claude-code --role cloud-security-engineer --provider azure --repo .

# Plan-only (no files written, prints "export agent: <id> [provider=<p>]")
npx vfa-export-agents --platform claude-code --provider nvidia --dry-run

# Install everything
npx vfa-export-agents --platform claude-code --all --repo .

# Opt out of companion skill bundle
npx vfa-export-agents --platform claude-code --role cloud-security-engineer --no-skills --repo .
```

### Selector precedence

`--agents`, `--role`, `--provider`, and `--all` are the four selector modes. They combine as follows:

| Combination | Behavior |
|---|---|
| `--agents <ids>` | Install exactly those agent ids. |
| `--role <r>` | Install every agent the role bundles. |
| `--role <r> --provider <p>` | Filter the role's agent list to provider `p`. |
| `--provider <p>` | Install every agent whose `provider == p` (standalone). |
| `--all` | Install every agent in the catalog. |
| `--dry-run` | Combine with any selector above to print the plan without writing. |

The CI gate `npm run validate:install-coverage` asserts (a) every agent in the catalog appears in at least one role, (b) every provider has at least one role-covered agent, (c) every role-referenced id exists in the catalog, and (d) each CLI selector mode behaves as documented. Adding an agent without adding it to at least one role will fail this gate.

---

## Path 3 — skills CLI (lower trust, third-party)

The `skills` CLI is an open-source, third-party tool maintained at [github.com/vercel-labs/skills](https://github.com/vercel-labs/skills). It pulls skill content directly from the GitHub repository at HEAD — it does not use the npm package, applies no version pin by default, and performs no integrity verification at install time.

Verified flag syntax as documented at [github.com/vercel-labs/skills](https://github.com/vercel-labs/skills):

| Flag | Long form | Description |
|------|-----------|-------------|
| `-l` | `--list` | List available skills in the source repo without installing |
| `-s` | `--skill <skills...>` | Install one or more specific skills by name; use `'*'` to install all |
| `-g` | `--global` | Install to user directory (`~/<agent>/skills/`) instead of project |
| `-a` | `--agent <agents...>` | Target one or more specific agent runtimes (e.g. `claude-code`) |
| `-y` | `--yes` | Skip confirmation prompts |
|      | `--copy` | Copy files instead of symlinking |
|      | `--all` | Install all skills to all agents without prompts |

```bash
# List every skill in this repo without installing
npx skills add VincentChuWaiChow/vanguard-frontier-agentic --list

# Install a specific skill into the current project
npx skills add VincentChuWaiChow/vanguard-frontier-agentic --skill aws-iac-patch-executor

# Install all skills globally for Claude Code
npx skills add VincentChuWaiChow/vanguard-frontier-agentic --skill '*' --global --agent claude-code

# Install all skills to all detected agents without prompts
npx skills add VincentChuWaiChow/vanguard-frontier-agentic --all --yes
```

> **Note on the README's flag usage:** The README historically used `--skill` (long form) and `-a` (short form). The CLI also accepts `-s` as the short form for `--skill` and `--agent` as the long form for `-a`. Both forms are documented at [github.com/vercel-labs/skills](https://github.com/vercel-labs/skills).

---

## Pinning the skills CLI path

The `vercel-labs/skills` CLI documentation does not describe a version-pin syntax for source repositories at install time. The tool fetches from the default branch at HEAD unless you specify a full GitHub URL pointing to a specific tree or commit, e.g.:

```
https://github.com/VincentChuWaiChow/vanguard-frontier-agentic/tree/<tag-or-sha>
```

Whether the CLI correctly resolves tree-scoped URLs is not confirmed in the published documentation. For environments where reproducibility matters, use Path 1 (npm) or Path 2 (vfa-export-agents) instead.

---

## Before you install

Regardless of which path you use, inspect content before running it in a shared or production environment.

### Inspect a SKILL.md frontmatter

Every skill in this repo is a `SKILL.md` file with YAML frontmatter. Before installing a skill globally or into a shared project, review:

1. **`name`** — the identifier the skill will be registered under.
2. **`description`** — what the skill does and when it is invoked.
3. **Body content** — the instructions the agent will follow. Read them as you would read a script you are about to run with elevated trust.

Example of what to look for:

```yaml
---
name: aws-iac-patch-executor
description: Reviews and applies IaC patches for AWS resources with approval gating.
---
# AWS IaC Patch Executor
...
```

Skills with `metadata.internal: true` in their frontmatter are hidden from discovery by default and only install when the `INSTALL_INTERNAL_SKILLS=1` environment variable is set.

### Check the references/ directory

Some skills include a `references/` subdirectory with source links, evidence templates, or supporting context. Review these before relying on a skill for compliance-sensitive work — they document the official sources the skill's guidance is based on.

### Telemetry

The `vercel-labs/skills` CLI collects anonymous usage data. Set `DISABLE_TELEMETRY=1` or `DO_NOT_TRACK=1` to opt out. Telemetry is automatically disabled in CI environments. This does not apply to paths 1 or 2, which do not collect telemetry.

---

## Choosing an install path

- **CI/CD pipelines and team-shared environments:** use Path 1 or Path 2. Pin the npm version. Check the lockfile into source control.
- **Local exploration and quick trials:** Path 3 is convenient but pulls from HEAD. Do not use it in automated pipelines.
- **Compliance-sensitive environments:** Path 1 only. Record the exact version in your dependency manifest and validate against `catalog/skill-manifest.json` for SHA-level integrity checking.

See [`docs/ci-cd-enforcement-pattern.md`](../ci-cd-enforcement-pattern.md) for CI/CD enforcement guidance.
