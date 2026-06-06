# EVAL REPORT: Azure skills AgentCore reference repair batch 008

## Scope

Processed the final remaining Azure skill gap after batch 007:
1. `skills/azure/azure-waf-security-review`

## Evidence

Documentation-based evidence came from Microsoft Learn documentation through the user configured documentation MCP, including Azure Well-Architected Security principles, the Well-Architected Security checklist, Microsoft Cloud Security Benchmark guidance, Defender for Cloud regulatory compliance, secure score, and recommendation review guidance.

## Capability evals

- AgentCore reference pack shape: PASS — the skill has a lean SKILL.md plus operations, safety, evidence, workflow/output, and official-sources references.
- AgentCore headings present: PASS — the operations reference includes `## High-risk assumptions to kill` and `## Safe command/code verification targets` plus the expected operational headings.
- Version alignment: PASS — SKILL.md metadata, metadata.json, and catalog/skills.json versions are `0.1.1` for the changed skill.
- Evidence language discipline: PASS — no prohibited internal environment wording or raw-ID placeholder wording found in the processed path.
- Credential and identifier boundary: PASS — touched guidance does not ask users to paste credentials, tenant IDs, subscription IDs, customer data, private keys, or secrets.
- AWS non-interference: PASS — `git diff --name-only` returned zero `skills/aws` paths.

## Regression evals

- `npm run validate:skill-schema`: PASS.
- `npm run manifest:check`: PASS.
- `npm run validate:asset-integrity`: PASS before workflow/eval artifact creation; asset integrity regenerated again afterward.
- `npm run validate`: PASS, `VALIDATE_EXIT:0`; log: `/tmp/vfa-validate-azure-skills-repair-008.log`.

## Remaining work

This batch proves Azure skills are repaired to the AgentCore reference-pack heading/reference standard. The active objective is not complete because OCI assets still require the same audit and repair.
