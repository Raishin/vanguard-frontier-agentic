# Upgrade Sequencing And Compatibility

Bounding an upgrade target by dependency compatibility, deprecation exposure, and safe rollout.

- The target version is bounded by the intersection of every dependency's supported Python range.
- `DeprecationWarning`s surface removals to come and should be resolved before the version jump.
- A staged pilot plus a rollback plan bounds the blast radius of an upgrade.

## Sources

- https://docs.python.org/3/whatsnew/index.html
- https://packaging.python.org/en/latest/
