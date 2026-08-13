# Publication Identity And Provenance

Publish authority models — trusted publishing/OIDC versus long-lived tokens — and what consumer-side verification requires.

- npm trusted publishing (OIDC-based) reached general availability 2025-07-31 for GitHub Actions and GitLab CI/CD, and publishes provenance by default when used.
- `npm publish --provenance` requires npm CLI version 9.5.0 or later, run from a supported cloud-hosted CI environment.
- A consumer verifies a published artifact's provenance with `npm audit signatures`.
- Classic (non-granular) npm tokens were permanently revoked 2025-12-09; any workflow or documentation still assuming one is broken, not outdated.
- Granular-token expiry defaults were cut to 7 days, with a 90-day maximum, announced 2025-09-29.
- Whether any CI provider beyond GitHub Actions and GitLab CI/CD (for example CircleCI) currently supports OIDC trusted publishing is not established by the sources this skill carries and must not be asserted.
