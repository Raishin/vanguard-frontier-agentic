# Packaging Supply-Chain Review Checklist

The per-concern checklist applied to every packaging review.

- Index: no private+public `--extra-index-url` mix; a single trusted `--index-url` or explicit namespacing plus pinning.
- Hashing: hash-checking mode is complete — every requirement and transitive dependency is hashed and pinned; `--require-hashes` is enforced for deploys.
- Locking: a lockfile with exact versions is the installed source of truth; no range-only production dependency.
- Build: `[build-system].requires` is pinned and hashed and build isolation is not disabled.
- Metadata: `[project]` metadata conforms to the PyPA specification and declares `requires-python` and a license.
- CI: release tokens are unavailable to fork-originated PR code; publishing uses short-lived scoped credentials.
