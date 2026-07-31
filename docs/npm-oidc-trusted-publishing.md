# npm OIDC Trusted Publishing — Lessons Learned

This document records the decisions, dead ends, and working pattern reached
during the migration from a long-lived `NPM_TOKEN` secret to npm's OIDC
trusted publishing for the `@raishin/vanguard-frontier-agentic` package.

## What OIDC trusted publishing is

npm trusted publishing creates a trust relationship between npmjs.com and a
specific GitHub Actions workflow. When the workflow runs, the npm CLI exchanges
a short-lived GitHub OIDC identity token for a granular publish token scoped to
one package. No long-lived secret is stored in GitHub Secrets.

Prerequisites (configured once on npmjs.com per package):

- Owner, repository, workflow file name, and GitHub Actions environment name
  must all match the registered trusted publisher entry.
- For this repo: owner `vincentchuwaichow`, repo `vanguard-frontier-agentic`,
  workflow `release.yml`, environment `npm-deployment-master`.

## What did NOT work and why

### Manual OIDC token exchange script

Early attempts replicated npm's internal exchange manually via curl:

```
POST https://registry.npmjs.org/-/npm/v1/oidc/token/exchange/package/@raishin%2Fvanguard-frontier-agentic
```

This consistently returned HTTP 404. Root causes:

1. The endpoint is npm's internal API, not a stable public contract.
2. The package must already exist on the registry before the exchange works.
3. URL encoding was tricky: `@` must be preserved and `/` must be uppercase
   `%2F` — `encodeURIComponent` produces `%40` (wrong) and lowercase `%2f` also
   fails.

**Lesson:** do not replicate what npm CLI already does internally. The curl
exchange approach is fragile, version-dependent, and unnecessary.

### Placeholder NPM_TOKEN (`npm_oidc_placeholder`)

Setting `NPM_TOKEN` to a dummy string (e.g. `"<npm-oidc-placeholder>"`) to satisfy `@semantic-release/npm`'s
`verifyConditions` step caused `EINVALIDNPMTOKEN` because the registry validates
the token on the `/-/whoami` call that `verifyConditions` makes before publish.

**Lesson:** a fake token will always be rejected. Do not set NPM_TOKEN to a
non-functional value.

### `set -euo pipefail` in the exchange script

Using `set -e` caused the shell to exit silently before the error message was
printed when `curl` returned a non-zero status, making failures invisible.

**Lesson:** use `set -uo pipefail` (without `-e`) and add explicit `|| { echo
error; exit 1; }` after each command that must not fail silently.

### `@semantic-release/npm` verifyConditions with OIDC

`@semantic-release/npm`'s `verifyConditions` lifecycle step calls `npm whoami`
(registry auth check) before the prepare/publish steps. With OIDC, no token
exists at that point — the CLI only mints one during `npm publish`. This caused
`EINVALIDNPMTOKEN` regardless of whether `NPM_TOKEN` was set or not.

**Lesson:** set `npmPublish: false` in the `@semantic-release/npm` plugin config
to skip both `verifyConditions` and the plugin's own `publish` step. Then run
`npm publish` manually in a separate workflow step after semantic-release
completes the version bump, changelog, and git tag.

### npm CLI version too old

Node 22 ships a bundled npm that may be below the v11.5.1 threshold required for
OIDC trusted publishing to work natively. Running `npm publish` with an older CLI
silently falls back to token auth, which then fails with no token present.

**Lesson:** do not use `npm install -g npm@latest` to upgrade — on GitHub Actions
runners the bundled npm 10.x has a broken `@npmcli/arborist` dependency
(`MODULE_NOT_FOUND: promise-retry`) that causes the self-upgrade to fail. Instead,
use `npx --yes npm@^11 publish` which downloads npm 11 on demand for the publish
step without touching the global installation.

Reference: [azu/setup-npm-trusted-publish](https://github.com/azu/setup-npm-trusted-publish) (May 2026).

### Empty `_authToken` in `.npmrc` poisons the OIDC code path

This is the failure mode that silently broke v2.4.0, v2.4.1, and v2.4.2 — all
three GitHub Releases were tagged but never reached the npm registry, which
remained at v2.3.0.

`actions/setup-node@v6` with `registry-url: https://registry.npmjs.org` writes
the following line to `${RUNNER_TEMP}/.npmrc` **and exports
`NPM_CONFIG_USERCONFIG`** pointing at that file (see setup-node v6.4.0
`src/authutil.ts`). It does **not** write to `~/.npmrc`:

```
//registry.npmjs.org/:_authToken=${NODE_AUTH_TOKEN}
```

With OIDC trusted publishing, `NODE_AUTH_TOKEN` is intentionally unset. The
variable interpolation expands to an empty string, leaving:

```
//registry.npmjs.org/:_authToken=
```

npm 11.x reads this as "a token IS configured" and short-circuits the OIDC
token-exchange code path. It then sends the PUT with an empty
`Authorization: Bearer` header. npmjs.com responds with **HTTP 404** (not
401/403) for unauthenticated writes to existing scoped packages — a
documented quirk that makes the failure look like a missing package or
trusted-publisher misconfiguration.

The Sigstore provenance step still succeeds in this scenario because it
exchanges the OIDC token directly with Sigstore (different audience), not
through npmjs.com — so log output shows a signed provenance statement
immediately followed by `npm error code E404`.

**Lesson:** strip the poisoned `_authToken` line from the file npm actually
reads. That file is `${NPM_CONFIG_USERCONFIG}` when setup-node is used —
**not** `~/.npmrc`. Stripping the wrong path silently leaves the broken
line in npm's active userconfig and the 404 persists:

```yaml
- name: Strip empty _authToken from active .npmrc for OIDC publish
  run: |
    NPMRC_PATH="${NPM_CONFIG_USERCONFIG:-$HOME/.npmrc}"
    if [ -f "$NPMRC_PATH" ]; then
      sed -i '/_authToken/d' "$NPMRC_PATH"
    fi
    # Defensive: also strip ~/.npmrc when it differs from the active config.
    if [ -f "$HOME/.npmrc" ] && [ "$HOME/.npmrc" != "$NPMRC_PATH" ]; then
      sed -i '/_authToken/d' "$HOME/.npmrc"
    fi
```

The `azu/setup-npm-trusted-publish` action linked above performs this same
strip on the active config internally — that is its primary purpose, not
version management.

## Working pattern

Derived from npm docs, semantic-release docs, and azu/setup-npm-trusted-publish.

### `.releaserc.js` — disable npm plugin publish

```js
["@semantic-release/npm", { npmPublish: false }],
```

This skips `verifyConditions` (token check) and the plugin's own publish step.
semantic-release still handles the version bump in `package.json`.

### `.github/workflows/release.yml` — key requirements

```yaml
jobs:
  release:
    environment: npm-deployment-master   # must match npmjs.com trusted publisher
    permissions:
      contents: write
      issues: write
      pull-requests: write
      id-token: write        # required for OIDC
      attestations: write

    steps:
      - uses: actions/setup-node@...
        with:
          node-version: "22"
          registry-url: "https://registry.npmjs.org"   # configures .npmrc

      # ... validate, npm ci, semantic-release ...

      - name: Release
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          # No NPM_TOKEN — OIDC handles auth during npm publish below
        run: npx semantic-release --debug

      - name: Publish to npm via OIDC trusted publisher
        run: |
          POST_VERSION="$(node -p "require('./package.json').version")"
          if [ "$POST_VERSION" = "$PRE_VERSION" ]; then
            echo "No version bump; skipping npm publish."
            exit 0
          fi
          # npx npm@^11 avoids the broken global npm install on runners;
          # downloads npm 11 on demand. OIDC requires npm >= 11.5.1.
          npx --yes npm@^11 publish --access public --provenance
```

### Why this works

- `registry-url` in `setup-node` writes an `.npmrc` pointing at the registry
  without a token, which npm CLI replaces with the OIDC-minted token at publish
  time.
- `id-token: write` allows the runner to request a GitHub OIDC identity token.
- The `environment: npm-deployment-master` claim is embedded in the identity
  token and validated by npmjs.com against the registered trusted publisher.
- `npm publish --provenance` attaches a Sigstore attestation linking the tarball
  to the exact workflow run that produced it.
- No `NPM_TOKEN` secret is stored anywhere.

## Known Issue: v2.4.x npm publication gap

**Status:** Fixed as of v2.5.0 (May 2026).

Versions **2.4.0, 2.4.1, 2.4.2, 2.4.3, and 2.4.4** were tagged in GitHub and
GitHub Releases were created, but these versions **never reached the npm registry**.
The npm registry remained at v2.3.0 while these GitHub tags accumulated.

### Root cause

All five releases failed due to the empty `_authToken` poisoning issue documented
above. Each CI run hit the HTTP 404 (misidentified as a trusted-publisher config
error) and exited without publishing. The fix was implemented after v2.4.4 was
already tagged, so the backlog of unpublished GitHub Releases was not retroactively
pushed to npm.

### Resolution

- v2.5.0 and all subsequent releases are now published to npm successfully using
  the fixed workflow (auto-fix `_authToken` stripping + npm@^11.5.1 + `--provenance`).
- The 2.4.x versions remain as GitHub Releases only and are not available on npm.
  They should **not** be used for production — always upgrade to v2.5.0+.

### If backfilling 2.4.x to npm is needed

To manually publish any of the v2.4.x releases to npm (if desired for archival):

```bash
git checkout v2.4.4
npm publish --access public --provenance
# Repeat for v2.4.3, v2.4.2, v2.4.1, v2.4.0 if needed
```

This requires:
- npm CLI >= 11.5.1 installed locally
- Valid OIDC trusted publisher registration on npmjs.com (same as CI workflow)
- GitHub OIDC token available (works in Actions; on local machines, requires
  manual OIDC token exchange or fallback to long-lived token)

This is a low-priority backfill since users were blocked from using 2.4.x anyway
due to the registry gap. Focus should remain on keeping 2.5.0+ releases flowing
to npm without interruption.

## References

- [npm Trusted Publishers docs](https://docs.npmjs.com/trusted-publishers)
- [semantic-release GitHub Actions recipe](https://semantic-release.gitbook.io/semantic-release/recipes/ci-configurations/github-actions)
- [azu/setup-npm-trusted-publish](https://github.com/azu/setup-npm-trusted-publish)
- npm CLI issue: [Allow publishing initial version with OIDC](https://github.com/npm/cli/issues/8544)
