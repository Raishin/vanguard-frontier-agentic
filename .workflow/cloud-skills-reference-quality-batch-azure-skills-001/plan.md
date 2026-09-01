Goal:
Refresh exactly 5 Azure skill assets in stable sorted order: azure-ai-foundry-ops-governor, azure-aks-platform-operator, azure-app-service-production-readiness, azure-cosmosdb-application-developer, azure-cosmosdb-performance-investigator.

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

Constraints:
- Exactly 5 items per batch.
- No AWS edits.
- Do not mention internal MCP server names, role names, profile names, connector IDs, or environment-specific identifiers in committed docs.
- Never request or record secrets, tenant IDs, subscription IDs, customer data, private keys, or credentials.

Risks:
- Overstating documentation as tenant evidence.
- Accidentally changing generated assets without regeneration.
- Duplicating guidance between primary skill docs and references.

Approval required:
- Commit only when user asks.
- No external writes or deployments.

Workflow artifact path:
.workflow/cloud-skills-reference-quality-batch-azure-skills-001

Work packets:
- P1: Azure AI Foundry ops governor evidence and patch.
- P2: AKS platform operator evidence and patch.
- P3: App Service production readiness evidence and patch.
- P4: Cosmos DB application developer evidence and patch.
- P5: Cosmos DB performance investigator evidence and patch.
- V1: Integration, manifests, validation, final report.

Integration policy:
Patch lean SKILL.md only where needed; detailed behavior and evidence belongs in references.

Verification:
npm run validate:skill-schema; npm run manifest:check; npm run validate:asset-integrity; broaden if generated outputs require it.
