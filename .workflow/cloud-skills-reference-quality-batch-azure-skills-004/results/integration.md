# Integration Result

Accepted:
- Exactly five Azure live-guard skill targets were refreshed in stable sorted order.
- Each target now has an AgentCore-style operations guide with required headings.
- Each target has refreshed evidence, safety, workflow, and source references.
- Each target version was bumped to 0.1.3 in SKILL.md, metadata.json, and catalog/skills.json.
- Generated skill manifest and asset integrity were regenerated.

Rejected:
- Any internal/environment-specific wording in committed references.
- Any AWS asset change in this pass.
- Any claim that documentation proves live tenant, subscription, app, cost, vault, or role state.

Conflicts:
- None after validation.

Decisions:
- Keep primary SKILL.md files lean and put live-operation detail in references.
- Treat Microsoft Learn as documentation evidence only, not proof of configured-environment posture.
- No live Azure tenant evidence was sampled in this batch.
