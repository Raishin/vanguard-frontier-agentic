# Azure skills AgentCore reference repair batch 008

Goal: Bring the final remaining Azure skill up to the AgentCore reference-pack standard with Microsoft Learn grounding.

Success criteria:
- Process the final Azure skill gap after batch 007.
- No AWS assets changed.
- The processed skill has a lean SKILL.md plus reference files for operations, safety, evidence, workflow/output, and official sources.
- The operations reference has `## High-risk assumptions to kill` and `## Safe command/code verification targets` plus the AgentCore-style operational headings.
- SKILL.md, metadata.json, and catalog/skills.json patch versions are aligned.
- Prohibited internal wording and raw placeholder patterns are absent in the processed path.
- Skill manifest and asset integrity are regenerated and validations pass.

Items:
1. `skills/azure/azure-waf-security-review`

Integration policy:
- Replace the old monolithic checklist SKILL.md with a lean loader and detailed references.
- Keep Azure evidence wording generic and separate documentation evidence from sampled configured-environment evidence.
- Do not request or preserve raw subscription identifiers, tenant identifiers, credentials, customer data, or secrets in committed guidance.

Verification:
- AgentCore reference-pack structural audit for the skill directory.
- Prohibited internal wording and raw-ID placeholder grep.
- AWS diff guard.
- `npm run validate:skill-schema`.
- `npm run manifest:check`.
- `npm run validate:asset-integrity`.
- Full `npm run validate`.
