---
layout: default
title: "Documentation"
permalink: /docs/
---

# 📊 Vanguard Frontier Agentic Documentation

Enterprise AI agent ecosystem providing **404 skills**, **426 agents**, and **32 providers** with first-class multi-harness support.

Package: `@raishin/vanguard-frontier-agentic` v2.6.0 (Apache-2.0)

---

## 🛰️ What This Is

A curated, validated catalog of AI agent skills and agents designed for enterprise cloud operations. Every asset passes 17 automated validation gates before merge. Every release ships with npm provenance, SLSA Build L3 attestations, and an SPDX SBOM.

Supported harnesses: Claude Code, Codex, GitHub Copilot, Cursor, Gemini CLI, Kiro.

---

## 🗂️ Documentation Map

### 🚀 Getting Started

| Page | Description |
|------|-------------|
| [Getting Started](getting-started/) | Installation, first use, CLI tools |
| [Configuration Reference](configuration/) | All settings, schemas, npm scripts |
| [FAQ](faq/) | Common questions answered with evidence |

### 🏗️ Architecture and Design

| Page | Description |
|------|-------------|
| [Architecture](architecture/) | Three-layer system, Mermaid diagrams, design rationale |
| [Governance](governance/) | ADR process, maintainer responsibilities, quality gates |
| [ADR-0001: Initial Architecture](adr/0001-initial-architecture/) | Three-layer Maestro decision record |
| [ADR-0002: Jekyll + GitHub Pages](adr/0002-documentation-site-with-jekyll-github-pages/) | Documentation platform choice |

### 📋 Operations

| Page | Description |
|------|-------------|
| [Deployment](deployment/) | npm publishing, OIDC, provenance, SLSA, SBOM |
| [Security](security/) | Supply chain, scanning, badges, threat model |
| [Testing](testing/) | All 17 validation gates, fuzz testing, routing scenarios |
| [Operations Runbook](operations-runbook/) | Release process, recovery, checklists |
| [Troubleshooting](troubleshooting/) | Common failures and their fixes |

### 🤝 Contributing

| Page | Description |
|------|-------------|
| [Contributing](contributing/) | How to add docs, conventions, local preview |
| [GitHub Pages Setup](github-pages/) | How the docs site is built and deployed |
| [Roadmap](roadmap/) | Current state, planned work, how to propose changes |

---

## ⚡ Quick Reference

```bash
# Install the package
npm install @raishin/vanguard-frontier-agentic

# Export agents for a specific platform
npx vfa-export-agents --platform claude --role cloud-security-engineer

# Run all validation gates
npm run validate

# Run fuzz tests
npm run test:fuzz
```

---

## 📊 Catalog Stats

| Metric | Count |
|--------|-------|
| Skills | 404 |
| Agents | 426 |
| Providers | 32 |
| MCP References | 3 |
| Rules | 1 |
| Install Roles | 21 |
| Validation Gates | 17 |
| Maestro Routing Scenarios | 357 |

---

## 🏛️ Enterprise Reviewer Notes

- Every claim in these docs is backed by a file path or command you can verify yourself
- Items marked `[NEEDS OWNER INPUT]` require maintainer clarification
- Security posture is documented in [security.md](security/) with adversarial framing
- Supply chain integrity is verifiable: `npm audit signatures` confirms provenance
