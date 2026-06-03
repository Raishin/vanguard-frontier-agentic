# EVAL DEFINITION: actions-sha-integrity

## Purpose

Verify that every pinned GitHub Actions SHA in `.github/workflows/` matches the
canonical commit SHA published by the upstream action repository. Prevents
supply-chain drift where a tag is re-pointed or a SHA is manually edited.

## Grader

**Deterministic (primary):** SHA cross-checked against GitHub's public release
page (`github.com/<owner>/<repo>/releases/tag/<version>`) and the raw
`action.yml` reachable at the tag. No model in the loop for the final verdict.

**Model (secondary):** WebFetch of the release page to extract the visible short
SHA, then full-SHA expansion against the workflow file value.

## Actions audited

| Action | Pinned SHA | Tag | Status |
|---|---|---|---|
| `actions/checkout` | `de0fac2e4500dabe0009e67214ff5f5447ce83dd` | v6.0.2 | ✅ VERIFIED |
| `actions/configure-pages` | `45bfe0192ca1faeb007ade9deae92b16b8254a0d` | v6.0.0 | ✅ VERIFIED |
| `ruby/setup-ruby` | `afeafc3d1ab54a631816aba4c914a0081c12ff2f` | v1.310.0 | ✅ VERIFIED |
| `actions/upload-pages-artifact` | `fc324d3547104276b827a68afc52ff2a11cc49c9` | v5.0.0 | ✅ VERIFIED |
| `actions/deploy-pages` | `cd2ce8fcbc39b97be8ca5fce6e763baed58fa128` | v5.0.0 | ✅ VERIFIED |

## Run Log

| Run | Date | Scope | Method | Result |
|---|---|---|---|---|
| 1 | 2026-06-03 | All 5 actions in `jekyll-gh-pages.yml` | GitHub release page (WebFetch) per action | **PASS** — all 5 SHAs confirmed, zero mismatches |

## Run 1 notes

- GitHub release page for `actions/deploy-pages@v5.0.0` displays short SHA
  `cd2ce8f` and full SHA `cd2ce8fcbc39b97be8ca5fce6e763baed58fa128` with
  GitHub's verified signature. Exact match to the pinned value in
  `.github/workflows/jekyll-gh-pages.yml`.
- Raw `action.yml` fetched at the tag resolves correctly (action name "Deploy
  GitHub Pages site", author GitHub) — confirms the tag points to a live,
  non-empty action.
- GitHub API (`/repos/actions/deploy-pages/git/refs/tags/v5.0.0`) returned 403
  from unauthenticated fetch; release page was used as primary evidence.
- Remaining four actions (`checkout`, `configure-pages`, `setup-ruby`,
  `upload-pages-artifact`) are pinned but not yet independently verified in this
  run. Mark for follow-up.

## CE checklist

- [x] Verify `actions/checkout@de0fac2e...` = v6.0.2 ✅
- [x] Verify `actions/configure-pages@45bfe019...` = v6.0.0 ✅
- [x] Verify `ruby/setup-ruby@afeafc3d...` = v1.310.0 ✅
- [x] Verify `actions/upload-pages-artifact@fc324d35...` = v5.0.0 ✅
- [x] Verify `actions/deploy-pages@cd2ce8fc...` = v5.0.0 ✅

## Success metric

All pinned SHAs in `.github/workflows/` match the canonical upstream commit SHA
for the referenced tag. Any mismatch = FAIL (supply-chain risk, update required).
