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
- For this repo: owner `raishin`, repo `vanguard-frontier-agentic`,
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

## References

- [npm Trusted Publishers docs](https://docs.npmjs.com/trusted-publishers)
- [semantic-release GitHub Actions recipe](https://semantic-release.gitbook.io/semantic-release/recipes/ci-configurations/github-actions)
- [azu/setup-npm-trusted-publish](https://github.com/azu/setup-npm-trusted-publish)
- npm CLI issue: [Allow publishing initial version with OIDC](https://github.com/npm/cli/issues/8544)
