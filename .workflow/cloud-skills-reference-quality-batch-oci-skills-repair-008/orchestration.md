# OCI Skills Repair Batch 008 Orchestration

Goal: finish the final OCI skill gap batch by aligning six remaining skills with the AgentCore-style reference standard.

Success criteria:
- Six final OCI skills have lean SKILL.md files and reference packs with operations, safety, evidence, workflow, and official-source files.
- Versions are bumped to 0.1.1 across frontmatter, metadata.json, and catalog/skills.json.
- Committed docs avoid local connector names, internal tool names, profile names, account-specific identifiers, and credential prompts.
- Skill manifest and asset integrity are regenerated.
- Narrow gates and full validation pass.

Work packets:
1. Storage and backup evidence + patch.
2. Support incident evidence + patch.
3. Well-Architected cost, reliability, and security evidence + patch.
4. Oracle/OCI MCP grounded-advisor source and safety patch.
5. Integration audit and validation.

Evidence policy:
- Official OCI documentation proves documented service behavior only.
- OCI API evidence through the user’s configured read-only OCI MCP proves sampled command shape or sanitized current-state observations only.
- Missing evidence remains an open question, not a positive finding.
