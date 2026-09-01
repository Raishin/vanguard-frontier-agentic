Goal:
Refresh exactly 5 Azure skill assets in stable sorted order after batch 002: azure-identity-governance-review, azure-key-vault-secret-lifecycle-auditor, azure-keyvault-certificate-issuer-review, azure-landing-zone-architect, azure-live-aks-rollout-guard.

Success criteria:
- Each item is reviewed one at a time against official Microsoft Learn documentation through the user\x27s configured documentation MCP and available read-only Azure evidence where useful.
- Stale, risky, missing, vague, or under-specified guidance is patched only in relevant skill files/references.
- Changed asset patch versions are bumped consistently in SKILL.md metadata, metadata.json, and catalog/skills.json.
- Generated manifests are refreshed after skill changes.
- Narrow validation gates pass before any commit.

Current context:
- Branch: feat/cloud-skills-reference-quality.
- Provider: Azure only for this batch.
- Asset type: skills only for this batch.
- Batches 001 and 002 are present but uncommitted.

Constraints:
- Exactly 5 items per batch.
- No AWS edits.
- Do not mention internal MCP server names, internal role names, local profile names, connector IDs, or environment-specific identifiers in committed docs.
- Never request or record secrets, tenant IDs, subscription IDs, customer data, private keys, or credentials.

Workflow artifact path:
.workflow/cloud-skills-reference-quality-batch-azure-skills-003

Work packets:
- P1: Identity governance review evidence and patch.
- P2: Key Vault secret lifecycle auditor evidence and patch.
- P3: Key Vault certificate issuer review evidence and patch.
- P4: Landing zone architect evidence and patch.
- P5: Live AKS rollout guard evidence and patch.
- V1: Integration, manifests, validation, final report.
