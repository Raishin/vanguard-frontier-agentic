# Integration Result

Accepted:
- Exactly five Azure skill targets were refreshed in stable sorted order.
- Each target now has an AgentCore-style operations guide with required headings.
- Each target has refreshed evidence, safety, workflow, and source references.
- Each target version was bumped to 0.1.2 in SKILL.md, metadata.json, and catalog/skills.json.
- Generated skill manifest and asset integrity were regenerated.

Rejected:
- Internal/environment-specific wording in committed references. A grep caught prohibited wording used in a negative sentence; it was removed.
- Any AWS asset change in this pass.

Conflicts:
- None after validation.

Decisions:
- Keep primary SKILL.md files lean and put service-specific depth in references.
- Treat Microsoft Learn as documentation evidence only, not proof of tenant posture.
- No live Azure tenant evidence was sampled in this batch.
