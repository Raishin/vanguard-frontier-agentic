## EVAL REPORT: cloud-skills-reference-quality-azure-leftovers

### Capability Evals
- azure-placeholder-removal: PASS - Azure/OCI skill forbidden-placeholder audit returned AZURE_OCI_SKILL_FORBIDDEN_GAPS 0 after replacing angle-bracket placeholders and stale/internal guidance patterns with neutral local shell variables or prose labels.
- microsoft-learn-refresh: PASS - official-sources references for the six originally identified Azure skills now include current Microsoft Learn deltas from the docs subagent for Entra ID, AKS rollout, App Service slots, Deployment Stacks, role assignments, and Key Vault rotation/purge.
- version-sync: PASS - changed Azure skill frontmatter, metadata.json, catalog/skills.json, catalog/skill-manifest.json, and catalog/asset-integrity.json were regenerated.
- subagent-triage: PASS - docs subagent supplied Microsoft Learn evidence; OCI agent triage subagent produced six exactly-5-item OCI agent repair batches.

### Regression Evals
- skill-schema: PASS - npm run validate:skill-schema.
- manifest-check: PASS - npm run manifest:check.
- asset-integrity: PASS - npm run validate:asset-integrity.
- full-validation: PASS - npm run validate; log at /tmp/vfa-validate-azure-skill-leftovers-current.log.

### Remaining Work
- OCI agents still need repair in six batches of five based on the read-only subagent triage.
- No commit was made.
