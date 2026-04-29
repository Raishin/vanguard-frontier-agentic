## EVAL DEFINITION: aws-trigger-keywords-and-manifest

### Assumptions

- Existing repo convention stores eval artifacts under `.claude/evals/`, so this eval follows that convention rather than creating `.codex/evals/`.
- Trigger quality can be partially checked deterministically by requiring domain-specific keywords in each AWS skill frontmatter description.
- Deterministic keyword checks do not prove real model trigger behavior; they catch obvious under-specified descriptions and drift.

### Capability Evals

1. Every cataloged AWS skill has a parseable `SKILL.md` frontmatter block.
2. Every AWS skill frontmatter description includes enough domain-specific trigger terms for expected routing.
3. High-collision AWS skills include disambiguation terms such as `Prefer`, `Use only`, or explicit alternative skill surfaces.
4. Every AWS skill has `metadata.author` and `metadata.version` under `metadata` in `SKILL.md` frontmatter.
5. No AWS skill uses top-level `author:` or top-level `version:` frontmatter keys.

### Regression Evals

1. Every AWS skill `metadata.json` version matches `catalog/skills.json` version.
2. Every AWS skill `SKILL.md` `metadata.version` matches adjacent `metadata.json` version.
3. `catalog/skill-manifest.json` includes every cataloged AWS skill and is current.
4. Existing catalog validation, manifest validation, and offline link validation still pass.

### Deterministic Graders

```bash
python3 tests/validate-aws-skill-quality.py
npm run validate
```

### Success Metrics

- Capability evals: pass@1 = 100% for deterministic checks.
- Regression evals: pass^1 = 100% for manifest/catalog/version checks.

### Human Review Required

A real trigger benchmark still needs the agent runtime to decide which skill it would invoke for realistic prompts. This eval only verifies static trigger readiness.

## EVAL REPORT: aws-trigger-keywords-and-manifest

### Capability Evals

- parseable-aws-frontmatter: PASS — `python3 tests/validate-aws-skill-quality.py` parsed all 25 AWS skill frontmatter blocks.
- trigger-keyword-coverage: PASS — required domain terms were present in every AWS skill description after patching missing terms for DynamoDB, EKS, and IaC.
- high-collision-disambiguation: PASS — high-collision skill descriptions now include explicit preference/disambiguation language.
- metadata-author-version-under-metadata: PASS — all AWS skills use `metadata.author` and `metadata.version`.
- no-top-level-author-version: PASS — grader found no forbidden top-level `author:` or `version:` frontmatter keys.

### Regression Evals

- metadata-catalog-version-sync: PASS — grader checked `SKILL.md` version, `metadata.json` version, and `catalog/skills.json` version alignment.
- manifest-includes-aws-skills: PASS — grader found every AWS skill in `catalog/skill-manifest.json`.
- manifest-current: PASS — `npm run manifest:write` then `npm run validate` confirmed manifest check passes.
- catalog-and-link-validation: PASS — `npm run validate` passed catalog, manifest, and offline link checks.

### Commands run

```bash
python3 tests/validate-aws-skill-quality.py
npm run manifest:write
python3 tests/validate-aws-skill-quality.py
npm run validate
```

### Results

```text
OK: validated 25 AWS skill trigger descriptions, versions, and manifest entries
OK: wrote catalog/skill-manifest.json with 78 skill entries
OK: validated 134 catalog entries and scanned for obvious secrets
OK: skill manifest matches 78 skill entries
OK: validated README links and 309 URLs (offline)
```

### Metrics

- pass@1 capability checks: 5/5 after targeted patching.
- pass^1 regression checks: 4/4.

### Status

READY, with one honest limitation: this is static trigger-readiness validation, not a live model trigger benchmark.
