# Lock Files, Hashes, And Where Verification Stops

What the lock file actually guarantees, and the platform gap that quietly removes the guarantee.

- The dependency lock file records the provider versions selected during `init` so that later runs install the same ones; without it committed, every environment re-selects within the version constraint and the reviewed set and the executed set are different artifacts.
- The lock file tracks provider dependencies only. Module versions are not locked, which is why module sources need immutable references to achieve what the lock file does for providers.
- The `zh:` scheme hashes the registry's official archive and therefore cannot verify an unpacked directory or a recompressed archive; the `h1:` scheme is computed from package contents and can verify all three forms.
- Hashes are recorded for the platforms encountered during `init`, so a lock file created on a developer machine records that platform and provides no verification for the platform CI actually runs on.
- `terraform providers lock -platform=...` pre-populates hashes for every named platform, which is the documented way to close the developer-versus-CI verification gap rather than an optional hardening step.
- Installing a provider from a source other than the origin registry can leave the lock file unable to verify checksums for any platform other than the one where `init` ran, which turns a mirror into an unverified installation path unless the hashes are pre-populated.
- `init -upgrade` deliberately discards the existing selections and re-selects the newest versions matching the constraints, so it is a supply-chain event: it is the moment a permissive constraint turns into an unreviewed package.
