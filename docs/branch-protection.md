# Branch Protection as Code

The `master` branch is protected by a declarative GitHub **Repository
Ruleset** stored at `.github/rulesets/master.json` and applied via the
`Apply Branch Ruleset` workflow (`.github/workflows/apply-ruleset.yml`).

This is the source of truth. If the ruleset in the GitHub UI drifts from
the JSON, re-run the workflow to reconcile.

## What the ruleset enforces

Target: branches matching `refs/heads/master`.

Rules:

- **Deletion blocked** — `master` cannot be deleted.
- **Force-push blocked** (`non_fast_forward`) — history is append-only.
- **Branch creation blocked** (`creation`) — rejects `git push` of a new
  ref that would match `master` (defense-in-depth against accidental
  re-creation after deletion).
- **Linear history required** — merge commits are rejected; only
  squash and rebase merges are allowed.
- **Pull request required**:
  - `required_approving_review_count: 0` — solo-maintainer repository,
    so we cannot require approvals from a second human, but
  - `require_code_owner_review: true` — the
    [`CODEOWNERS`](../.github/CODEOWNERS) file routes review requests
    and a code-owner must approve before merge for paths it covers.
  - `dismiss_stale_reviews_on_push: true` — new commits invalidate
    prior approvals.
- **Required status checks** (strict — branch must be up to date with
  `master` before merge):
  - `validate`
  - `smoke`
  - `Analyze (javascript-typescript)`
  - `Analyze (python)`
  - `markdownlint`
  - `codespell`
- **Signature requirement: not enforced.** GPG/Sigstore commit signing
  is a separate roadmap item; we do not gate merges on it today.

`Scorecard analysis` is intentionally **excluded** from required checks
because it runs post-merge on `master`, not on pull requests.

## How to apply

The workflow runs only on manual dispatch. After merging a change to the
ruleset JSON, re-run the workflow:

```bash
gh workflow run apply-ruleset.yml
```

Optionally pass an alternate file path:

```bash
gh workflow run apply-ruleset.yml -f ruleset_file=.github/rulesets/master.json
```

The workflow is idempotent. It looks up an existing ruleset named
`master-branch-protection` and `PUT`s the new definition, or `POST`s a
new ruleset if none exists.

## How to update

1. Edit `.github/rulesets/master.json`.
2. Open a pull request. Required checks will gate the change.
3. After merge, run `gh workflow run apply-ruleset.yml`.
4. Verify in the repository UI under **Settings -> Rules -> Rulesets**.

## Why a ruleset, not the legacy branch protection API

- Rulesets are GitHub's modern, supported model. The legacy
  `branches/{branch}/protection` endpoint is in maintenance mode.
- Rulesets layer cleanly with organization-level rulesets, so an org
  policy can add additional constraints without conflicting.
- Rulesets support `fnmatch` ref selectors, so the same JSON can later
  protect release branches (`refs/heads/release/*`) without rewriting.
- Rulesets expose a single `enforcement` switch (`active`,
  `evaluate`, `disabled`) that supports dry-run rollout — useful when
  introducing new checks.

## Bypass procedure (emergency only)

`bypass_actors` lists the **Repository Admin** role
(`actor_type: RepositoryRole`, `actor_id: 5`) with
`bypass_mode: pull_request`. This means an admin can open a pull
request that bypasses the rules but **only by explicit declaration in
the PR** — direct pushes to `master` are still blocked.

If a true emergency (e.g., supply-chain incident requiring an immediate
revert) requires bypassing all rules:

1. Temporarily flip `enforcement` to `disabled` in
   `.github/rulesets/master.json`, commit on a hotfix branch, merge,
   and run `gh workflow run apply-ruleset.yml`.
2. Perform the emergency action.
3. Immediately revert `enforcement` to `active` and re-apply.
4. File a follow-up issue documenting the bypass and the postmortem.

Document every bypass in the repository's incident log.

## Caveats

- The workflow uses the default `GITHUB_TOKEN`. The token's
  `administration: write` permission is sufficient to manage rulesets
  on the repository it belongs to. If GitHub later requires a PAT or
  GitHub App token for ruleset writes on this account tier, swap
  `secrets.GITHUB_TOKEN` for a stored admin token (e.g.,
  `secrets.RULESET_ADMIN_TOKEN`).
- The `actor_id: 5` for `RepositoryRole` corresponds to the built-in
  **Admin** role. Custom repository roles have different IDs; verify
  with `gh api /repos/{owner}/{repo}/roles` before changing.
- The ruleset must be applied **once** after this change merges. After
  that, re-apply only when the JSON changes.
