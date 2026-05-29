---
layout: default
title: "GitHub Pages Setup"
permalink: /docs/github-pages/
---

# 🌐 GitHub Pages Setup

The documentation site is built with Jekyll and deployed to GitHub Pages via a dedicated GitHub Actions workflow.

---

## 🔧 How It Works

1. Documentation lives in `docs/` as Markdown files with Jekyll front matter
2. A push to `master` that touches docs triggers `.github/workflows/jekyll-gh-pages.yml`
3. The workflow builds the Jekyll site and deploys it to GitHub Pages
4. The site is available at `https://raishin.github.io/vanguard-frontier-agentic/`

---

## 🚀 Enabling GitHub Pages

To enable Pages for this repository:

1. Go to **Settings > Pages**
2. Under **Source**, select **GitHub Actions**
3. The `jekyll-gh-pages.yml` workflow handles the rest

No branch-based deployment is needed. The workflow uses the official `actions/deploy-pages` action with OIDC token exchange.

---

## Workflow Explained

File: `.github/workflows/jekyll-gh-pages.yml`

### Triggers

```yaml
on:
  push:
    branches: [master]
    paths:
      - "docs/**"
      - "_config.yml"
      - "Gemfile"
      - "index.md"
      - "*.md"
  workflow_dispatch:
```

The workflow runs when documentation content changes on master, or on manual dispatch.

### Permissions (Least-Privilege)

```yaml
permissions:
  contents: read   # Top-level baseline

jobs:
  build-and-deploy:
    permissions:
      pages: write       # Push built artifact to Pages
      id-token: write    # OIDC for Pages deployment API
```

### Concurrency Control

```yaml
concurrency:
  group: "pages"
  cancel-in-progress: false
```

Only one deployment runs at a time. In-progress deployments are NOT cancelled to avoid inconsistent states.

### Build Steps

1. **Checkout** - `actions/checkout`
2. **Configure Pages** - `actions/configure-pages`
3. **Setup Ruby** - `ruby/setup-ruby` with `ruby-version: 3.3` and bundler caching
4. **Build Jekyll** - `bundle exec jekyll build --baseurl "/vanguard-frontier-agentic"`
5. **Upload artifact** - `actions/upload-pages-artifact` with `path: ./_site`
6. **Deploy** - `actions/deploy-pages`

### Environment

The job binds to the `github-pages` environment:

```yaml
environment:
  name: github-pages
  url: ${{ steps.deployment.outputs.page_url }}
```

---

## Jekyll Configuration

File: `_config.yml`

Key settings:

| Setting | Value | Purpose |
|---------|-------|---------|
| `baseurl` | `/vanguard-frontier-agentic` | URL path prefix for project Pages |
| `theme` | `minima` | Clean default theme |
| `markdown` | `kramdown` | Markdown processor |
| `plugins` | `jekyll-seo-tag` | SEO metadata |

The `exclude` list prevents Jekyll from processing the npm package tree (agents, skills, node_modules, etc.).

---

## Local Preview

### Prerequisites

- Ruby 3.3+ installed
- Bundler installed (`gem install bundler`)

### Steps

```bash
# Install gems
bundle install

# Serve locally with live reload
bundle exec jekyll serve --baseurl "" --livereload

# Site available at http://localhost:4000/
```

Use `--baseurl ""` locally to avoid the `/vanguard-frontier-agentic` prefix.

### Using Docker (alternative)

```bash
docker run --rm -v "$PWD:/srv/jekyll" -p 4000:4000 \
  jekyll/jekyll:4.3 jekyll serve --baseurl ""
```

---

## 📄 Adding New Documentation Pages

1. Create a Markdown file in `docs/` with front matter:

```markdown
---
layout: default
title: "Page Title"
permalink: /docs/page-slug/
---

# Page Title

Content here...
```

2. The page is automatically included in the build (no config change needed)
3. Add a link to `docs/index.md` for discoverability
4. Push to master; the workflow deploys automatically

---

## 🔍 Troubleshooting

### Build fails with "could not locate Gemfile"

**Cause:** The `Gemfile` is excluded in `_config.yml` or missing from the repo root.
**Fix:** Verify `Gemfile` exists at the repository root and is not in the `exclude` list (it should not be - it is needed for `bundle install`).

### Page shows 404

**Cause:** Permalink does not match the URL you are visiting, or Pages is not enabled.
**Fix:** Check `permalink:` in front matter. Verify Pages is set to "GitHub Actions" source in repo settings.

### Styles not loading

**Cause:** `baseurl` mismatch between local and deployed.
**Fix:** Use `--baseurl ""` for local development. The workflow passes `--baseurl "/vanguard-frontier-agentic"` for production.

### Workflow does not trigger

**Cause:** Path filters do not match the changed files.
**Fix:** The workflow triggers on `docs/**`, `_config.yml`, `Gemfile`, `index.md`, and `*.md`. Ensure your change touches one of these paths.

### Ruby version mismatch

**Cause:** Local Ruby version differs from workflow (3.3).
**Fix:** Use `ruby-install 3.3` or configure via `.ruby-version`.

---

## ✅ How to Verify This Works

1. Check the workflow ran: **Actions tab > Deploy Documentation to GitHub Pages**
2. Verify the site loads: `https://raishin.github.io/vanguard-frontier-agentic/`
3. Verify new pages appear in navigation
4. Confirm `workflow_dispatch` works by manually triggering from the Actions tab

```bash
# Verify the workflow file is valid YAML
python3 -c "import yaml; yaml.safe_load(open('.github/workflows/jekyll-gh-pages.yml'))"
```
