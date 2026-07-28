# Estate-Modernization Review Checklist

The per-concern checklist applied to every runtime-estate review.

- EOL/support: every interpreter's support status is confirmed against the official CPython release/EOL schedule, never asserted from memory.
- Compatibility: the upgrade target is bounded by a real dependency/framework compatibility matrix (supported Python ranges, C-extension wheels, dropped stdlib modules).
- Deprecation: every removed/deprecated stdlib API and `DeprecationWarning`-to-error path is inventoried before the jump.
- Portfolio: shared-runtime and business-criticality mapping exists across the estate before sequencing upgrades.
- Ownership: every business-critical service on the estate has a named owner and a documented support-posture record.
- Rollout: every upgrade ships with a staged pilot cohort and a defined rollback plan.
