# Upgrade Risk, Ordering, And Rollback

Why upgrades stall, which breaks are invisible until plan time, and why rollback is a state problem.

- The expensive breaking changes in a provider major upgrade are usually not errors. A renamed, retyped, or newly computed attribute leaves the configuration parsing correctly and surfaces as a plan that destroys and recreates production resources, which means the upgrade must be assessed through a plan rather than through a successful `init`.
- A version move is not reversible by swapping the binary back: state written by a newer engine is generally not readable by an older one, so the real rollback path is a state restore, and an upgrade endorsed without a verified restore path has no rollback at all.
- Upgrade guidance is written per version pair, so a breaking change present in one minor version may be absent in the next; generalizing across versions produces findings that are confidently wrong rather than usefully uncertain.
- The v1 compatibility promise covers a defined surface and carries explicit exclusions; treating 'it is a minor version' as a safety argument substitutes the version number for the promise's actual scope.
- Moving core and multiple provider majors in a single change destroys attribution: when the resulting plan shows unexpected replacements, nothing identifies which of the moves caused them, and the only remaining diagnostic is to unpick the change and repeat it in pieces.
- Version lag is an expiring asset. Supported upgrade paths are defined from specific starting points, so waiting does not hold risk constant — it closes paths, and eventually converts a routine minor upgrade into a multi-version migration project.
- A deprecation notice is a scheduled breaking change with a known lead time, and its entire value lies in the interval before it expires; an estate that inventories deprecations only when they break has converted a warning system into an incident source.
- `init -upgrade` is the moment a permissive version constraint becomes a concrete selected version, so it is the event that must be gated and reviewed rather than a routine refresh.
