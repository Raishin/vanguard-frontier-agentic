# Provider Stability and Upgrades

Why Snowflake feature maturity and Terraform provider maturity must be assessed separately, and how to survive a provider upgrade. Load before adopting a resource or bumping a version.

## Two independent maturity axes

- Snowflake can ship a feature to general availability while the Terraform provider's resource for it remains preview — and the reverse can also be true for older resources that were never reworked.
- The provider requires preview features to be explicitly enabled in the provider configuration via a list of feature names. That opt-in is a maintenance commitment: preview resources are documented as not officially supported and as expected to introduce breaking changes.
- The provider's own roadmap distinguishes resources stabilized before v1.0.0, which receive official support, from preview features carried forward from earlier experimental versions and left in place to avoid blocking adoption. Read the resource's own documentation page — preview status is marked there.
- The practical rule: check the specific resource at the specific provider version. Neither 'Snowflake supports it' nor 'we're on v1.x' answers the question.
- Where a preview resource is genuinely needed, state it as an accepted risk with a named owner, a monitoring plan for provider releases, and a fallback — not as a configuration detail.

## Upgrading without an estate-wide incident

- Pin the provider version. An unpinned or loosely constrained provider means the next run may bring a different provider than the reviewed plan assumed, silently converting a reviewed change into an unreviewed one.
- Read the migration guide for every version step, not just the last one. Behaviour changes accumulate across intermediate versions.
- Rehearse in a non-production account with a representative configuration, and diff the resulting plan against the production plan. A provider upgrade that produces unexpected diffs in rehearsal has just paid for itself.
- Assess blast radius by what the provider manages, not by what the upgrade is intended to change. A provider upgrade touches every resource in state.
- Upgrade the provider as its own change, separately from any functional change. Combining them makes attribution impossible when something breaks.

## Time-sensitive claims

Each row is volatile: re-verify against the cited primary source before encoding it in a recommendation. A status that has moved silently converts a safe recommendation into an unsafe one.

| Claim | Status / constraint | Verified | What the source proves | What it does NOT prove |
|---|---|---|---|---|
| The Snowflake Terraform provider requires preview features to be explicitly enabled via `preview_features_enabled` in the provider configuration, and documents preview resources as not guaranteed stable, not officially supported, and expected to introduce breaking changes. | Current provider behaviour — verify against the provider version in use | 2026-08-17 via Context7 `/snowflakedb/terraform-provider-snowflake` (MIGRATION_GUIDE, ROADMAP, resource docs) | That provider resource stability is an explicit, separately declared property, independent of the Snowflake feature's own maturity | The status of any specific resource at the version this estate pins — check that resource's documentation page for that version |
| Resources stabilized before the provider's v1.0.0 release receive official Snowflake support after GA; preview features were carried forward from earlier experimental versions, were not reworked, and are expected to be reworked and marked stable in future releases. | Current provider roadmap position | 2026-08-17 via Context7 `/snowflakedb/terraform-provider-snowflake` (ROADMAP) | That the stable/preview split is a deliberate, documented provider policy with support consequences | When any specific preview resource will stabilize, or that it will do so without breaking changes |

## Sources

Primary sources for the claims above. Each line states what that page establishes — a URL with no claim attached is a bibliography, not a reference.

- https://github.com/snowflakedb/terraform-provider-snowflake — The provider's migration guide, roadmap, and per-resource preview markings — the authoritative source for resource stability at a given version
- https://registry.terraform.io/providers/snowflakedb/snowflake/latest/docs — Per-resource documentation, including the preview marking and the feature name required to enable it
