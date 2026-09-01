# Azure skills AgentCore reference repair batch 001

Goal: Bring the first five sorted Azure skills up to the AgentCore reference-pack standard by adding exact missing risk and verification sections grounded in Microsoft Learn documentation.

Success criteria:
- Exactly five Azure skill directories processed in stable sorted order.
- No AWS assets changed.
- Each processed skill has `## High-risk assumptions to kill` and `## Safe command/code verification targets` in references.
- SKILL.md, metadata.json, and catalog/skills.json patch versions are aligned.
- Skill manifest and asset integrity are regenerated and validations pass.

Constraints:
- Use Microsoft Learn documentation through the user configured documentation MCP for Azure service evidence.
- Keep committed docs generic; do not mention internal MCP server names, connector IDs, local profiles, or environment-specific identifiers.
- Do not ask for credentials, tenant IDs, subscription IDs, customer data, keys, or secrets.
- Patch only Azure skills in this batch.

Packets:
- Packet A: Microsoft Foundry operations governance reference repair.
- Packet B: AKS platform operations reference repair.
- Packet C: App Service production operations reference repair.
- Packet D: Cosmos DB application design reference repair.
- Packet E: Cosmos DB performance investigation reference repair.

Integration policy:
- Keep SKILL.md lean; put operational specificity in references.
- Preserve existing reference files and insert only missing AgentCore-standard sections.
- Separate documentation-based claims from sampled configured-environment evidence language.

Verification:
- Structural heading audit for the five skill directories.
- Prohibited internal wording grep.
- AWS diff guard.
- `npm run validate:skill-schema`.
- `npm run manifest:check`.
- `npm run validate:asset-integrity`.
- Full `npm run validate`.
