# Azure skills AgentCore reference repair batch 006

Goal: Bring the next five sorted Azure skills up to the AgentCore reference-pack standard by adding exact missing risk and verification sections grounded in Microsoft Learn documentation.

Success criteria:
- Exactly five Azure skill directories processed in stable sorted order after batch 005.
- No AWS assets changed.
- Each processed skill has `## High-risk assumptions to kill` and `## Safe command/code verification targets` in references.
- SKILL.md, metadata.json, and catalog/skills.json patch versions are aligned.
- Prohibited internal wording and raw placeholder patterns are absent in processed paths.
- Skill manifest and asset integrity are regenerated and validations pass.

Items:
1. `skills/azure/azure-platform-automation-devops`
2. `skills/azure/azure-private-endpoint-adoption-planner`
3. `skills/azure/azure-rbac-review`
4. `skills/azure/azure-resilience-bcdr-review`
5. `skills/azure/azure-resource-health-incident-triage`

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
