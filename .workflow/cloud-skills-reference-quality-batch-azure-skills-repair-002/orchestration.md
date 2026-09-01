# Azure skills AgentCore reference repair batch 002

Goal: Bring the next five sorted Azure skills up to the AgentCore reference-pack standard by adding exact missing risk and verification sections grounded in Microsoft Learn documentation.

Success criteria:
- Exactly five Azure skill directories processed in stable sorted order after batch 001.
- No AWS assets changed.
- Each processed skill has `## High-risk assumptions to kill` and `## Safe command/code verification targets` in references.
- SKILL.md, metadata.json, and catalog/skills.json patch versions are aligned.
- Skill manifest and asset integrity are regenerated and validations pass.

Items:
1. `skills/azure/azure-cosmosdb-platform-operator`
2. `skills/azure/azure-cost-estimation-review`
3. `skills/azure/azure-cost-optimization-governor`
4. `skills/azure/azure-entra-id-specialist`
5. `skills/azure/azure-governance-policy-guardrails`

Integration policy:
- Keep SKILL.md lean; put operational specificity in references.
- Preserve existing reference packs and add only missing AgentCore-standard sections.
- Keep Azure evidence wording generic and separate documentation evidence from sampled configured-environment evidence.

Verification:
- Structural heading audit for the five skill directories.
- Prohibited internal wording grep.
- AWS diff guard.
- `npm run validate:skill-schema`.
- `npm run manifest:check`.
- `npm run validate:asset-integrity`.
- Full `npm run validate`.
