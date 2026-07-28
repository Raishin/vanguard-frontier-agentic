# High-Severity Failure Modes

The production incidents each finding class maps to, for severity calibration.

- A fleet still running Python 2 (or an EOL 3.x) misses a critical security patch and stays exposed for months after the CVE is public.
- An upgrade shipped without a compatibility matrix breaks a pinned C-extension dependency in production, taking the service down.
- A `DeprecationWarning` that was never inventoried becomes a hard error in the target version and the upgrade fails mid-rollout.
- A business-critical script with no named owner sits on an unsupported interpreter for years because no one is accountable for upgrading it.
- An upgrade with no pilot or rollback plan is pushed fleet-wide and a single incompatibility takes down every dependent service at once.
