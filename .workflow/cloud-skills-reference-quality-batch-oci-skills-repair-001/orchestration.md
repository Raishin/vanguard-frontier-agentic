# OCI skills AgentCore reference repair batch 001

Goal: Bring the first five sorted OCI skills up to the AgentCore reference-pack standard using official OCI documentation and OCI API evidence through the user's configured read-only OCI MCP.

Success criteria:
- Exactly five OCI skill directories processed in stable sorted order.
- No AWS assets changed.
- Each processed skill has a lean SKILL.md plus operation, safety, evidence-path, workflow/output, and official-source references.
- Each operations reference has `## High-risk assumptions to kill` and `## Safe command/code verification targets` plus the AgentCore-style operational headings.
- SKILL.md, metadata.json, and catalog/skills.json patch versions are aligned.
- Prohibited internal wording, local profile naming, configured-server naming, and raw placeholder patterns are absent in processed paths.
- Skill manifest and asset integrity are regenerated and validations pass.

Items:
1. `skills/oci/oci-autonomous-database-architect`
2. `skills/oci/oci-certificates-issuer-review`
3. `skills/oci/oci-cloud-guard-responder`
4. `skills/oci/oci-compute-instance-agent-operator`
5. `skills/oci/oci-compute-platform-operator`

Integration policy:
- Replace stale internal-tool-specific wording with generic OCI API evidence phrasing.
- Keep SKILL.md lean; put operational specificity in references.
- Preserve component-specific Autonomous Database deployment and compatibility references in refreshed form.
- Do not request or preserve credentials, tenancy identifiers, compartment identifiers, resource identifiers, customer data, wallets, private keys, or secrets in committed guidance.

Verification:
- AgentCore reference-pack structural audit for the five skill directories.
- Prohibited internal wording and raw-ID placeholder grep.
- AWS diff guard.
- `npm run validate:skill-schema`.
- `npm run manifest:check`.
- `npm run validate:asset-integrity`.
- Full `npm run validate`.
