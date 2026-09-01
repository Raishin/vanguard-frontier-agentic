# Azure skills AgentCore reference repair batch 005

Goal: Bring the next five sorted Azure skills up to the AgentCore reference-pack standard by adding exact missing risk and verification sections grounded in Microsoft Learn documentation.

Success criteria:
- Exactly five Azure skill directories processed in stable sorted order after batch 004.
- No AWS assets changed.
- Each processed skill has `## High-risk assumptions to kill` and `## Safe command/code verification targets` in references.
- SKILL.md, metadata.json, and catalog/skills.json patch versions are aligned.
- Raw subscription, tenant, resource-group, app, stack, vault, and role-assignment placeholders are not retained in committed guidance for the processed paths.
- Skill manifest and asset integrity are regenerated and validations pass.

Items:
1. `skills/azure/azure-live-pim-jit-activation-guard`
2. `skills/azure/azure-maestro`
3. `skills/azure/azure-migrate-landing-zone-cutover`
4. `skills/azure/azure-network-topology-review`
5. `skills/azure/azure-observability-investigator`

Integration policy:
- Keep SKILL.md lean; put operational specificity in references.
- Preserve existing reference packs and add only missing AgentCore-standard sections.
- Keep Azure evidence wording generic and separate documentation evidence from sampled configured-environment evidence.
- Do not request or preserve raw subscription identifiers, tenant identifiers, credentials, customer data, or secrets in committed guidance.

Verification:
- Structural heading audit for the five skill directories.
- Prohibited internal wording and raw-ID placeholder grep.
- AWS diff guard.
- `npm run validate:skill-schema`.
- `npm run manifest:check`.
- `npm run validate:asset-integrity`.
- Full `npm run validate`.
