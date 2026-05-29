---
layout: default
title: "ADR-0002: Documentation Site with Jekyll and GitHub Pages"
permalink: /docs/adr/0002-documentation-site-with-jekyll-github-pages/
---

# ADR-0002: Documentation Site with Jekyll and GitHub Pages

## Status

Accepted

## Date

2024-12-01

## Context

The project needs a documentation site that:

1. Is discoverable (public URL, indexable by search engines)
2. Lives alongside the code (single repository, reviewed in PRs)
3. Requires minimal infrastructure (no servers, no databases, no DNS management)
4. Supports Markdown authoring (matches the project's existing content format)
5. Renders Mermaid diagrams (architectural documentation relies on these)
6. Is maintainable by a single developer without frontend expertise

### Options Evaluated

| Option | Pros | Cons |
|--------|------|------|
| **Jekyll + GitHub Pages** | GitHub-native, zero infra, Markdown-first, built-in CI | Limited to static, theme constraints, Ruby dependency for local dev |
| **Docusaurus** | Rich features, React-based, versioning | Requires Node build, separate deploy, heavier maintenance |
| **MkDocs (Material)** | Python-native, excellent search, clean | Requires Python build, separate deploy, mkdocs.yml config |
| **Custom Next.js/Astro site** | Full control, modern stack | Requires hosting, significantly more code to maintain |
| **GitHub Wiki** | Zero setup | Not indexable, poor SEO, no CI integration, limited formatting |
| **README-only** | Simplest | Does not scale beyond a single page, poor navigation |

### Constraints

- The project is an npm package with Node.js and Python tooling. Adding Ruby is acceptable for docs-only local preview but should not be a CI dependency for non-docs changes.
- The maintainer is a single developer; operational overhead must be minimal.
- The project already has a `.github/workflows/` directory with 10+ workflows; adding one more for docs is natural.
- Mermaid support is required for architecture documentation.

## Decision

Use Jekyll with the Minima theme, deployed via GitHub Pages using the official GitHub Actions workflow.

### Implementation Details

**Configuration:**
- `_config.yml` at repo root with `baseurl: "/vanguard-frontier-agentic"`
- `Gemfile` with `jekyll ~> 4.3` and `minima ~> 2.5`
- Theme: Minima (clean, readable, maintained by GitHub)
- Markdown processor: Kramdown

**Deployment:**
- Workflow: `.github/workflows/jekyll-gh-pages.yml`
- Source: GitHub Actions (not branch-based deployment)
- Environment: `github-pages`
- Trigger: Push to master affecting `docs/**`, `_config.yml`, `Gemfile`, `index.md`, or `*.md`
- Permissions: `pages: write`, `id-token: write` (OIDC for Pages API)

**Content:**
- Documentation pages in `docs/` with Jekyll front matter
- ADRs in `docs/adr/`
- Mermaid diagrams in fenced code blocks (GitHub Pages renders these natively)
- Relative links between pages

**Exclusions:**
- The `exclude` list in `_config.yml` prevents Jekyll from processing the entire npm package tree (agents, skills, node_modules, etc.)

## Consequences

### What becomes easier

- **Zero infrastructure**: No servers, no DNS, no hosting bills. GitHub handles everything.
- **PR-based workflow**: Docs changes go through the same PR/review/CI process as code.
- **Markdown authoring**: No special syntax or components needed. Plain Markdown works.
- **Mermaid support**: GitHub Pages renders Mermaid natively in code blocks.
- **Automatic deployment**: Push to master deploys. No manual steps.
- **HTTPS by default**: GitHub Pages provides TLS certificates automatically.

### What becomes harder

- **Dynamic content**: No server-side logic. Cannot query the catalog at page load time.
- **Search**: No built-in full-text search (Minima theme limitation). Users rely on browser find or external search engines.
- **Versioned docs**: No built-in version switcher. One version of docs lives at a time.
- **Custom components**: Limited to what Minima theme and Kramdown support. No React/Vue components.
- **Local preview**: Requires Ruby + Bundler installed locally. Not needed for content-only changes but helpful for layout testing.
- **Theme customization**: Minima is clean but opinionated. Custom layouts require overriding theme files.

### Risks

- **GitHub Pages outage**: Docs are unavailable if GitHub Pages is down. Mitigated by the fact that the Markdown source is always in the repo.
- **Jekyll deprecation**: GitHub has signaled support for Jekyll long-term, but a future migration to another static site generator is possible. The Markdown content would transfer.
- **Exclude list maintenance**: New top-level directories must be added to `_config.yml` exclude list. Forgetting this causes Jekyll build failures or unnecessarily large sites.

### Trade-offs Accepted

- Accepted: Ruby as a development dependency for local preview (not required for CI or non-docs work)
- Accepted: No versioned docs (single version acceptable for a rapidly evolving project)
- Accepted: No full-text search (acceptable given the documentation size)
- Accepted: Static-only content (all content can be expressed as Markdown + Mermaid)
