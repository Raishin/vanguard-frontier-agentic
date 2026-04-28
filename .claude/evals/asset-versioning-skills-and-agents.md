## EVAL DEFINITION: asset-versioning-skills-and-agents

### Capability evals

1. Every skill metadata file includes an explicit semantic `version`.
2. Every agent metadata file includes an explicit semantic `version`.
3. Every cataloged skill entry includes the same `version` as its adjacent metadata file.
4. Every cataloged agent entry includes the same `version` as its adjacent metadata file.

### Regression evals

1. Existing catalog validation still passes after adding required asset versions.
2. Existing skill manifest validation still passes.
3. Existing offline link validation still passes.

### Deterministic checks

- Count all skill metadata files and ensure each has `version`.
- Count all agent metadata files and ensure each has `version`.
- Ensure every `version` matches semver `X.Y.Z`.
- Ensure every skill catalog entry version matches its `metadata.json`.
- Ensure every agent catalog entry version matches its `metadata.json`.
- Run `npm run validate`.
