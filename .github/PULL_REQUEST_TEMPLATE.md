## Summary

<!--
Describe what this PR changes and why. Link to the issue it addresses.
Closes #<issue-number>
-->

## Type of change

- [ ] Skill (new or updated `skills/` asset)
- [ ] Agent (new or updated `agents/` asset)
- [ ] Rule (new or updated `rules/` asset)
- [ ] Docs (changes under `docs/`, `CONTRIBUTING.md`, or other Markdown governance files)
- [ ] Infra (schemas, catalog, scripts, tests, CI/CD)

## Validation evidence

<!--
Paste the full output of `npm run validate` below. Every check must pass.
-->

```
<paste npm run validate output here>
```

## Risk and rollback

<!--
Describe the blast radius of this change.
If something goes wrong after merge, what is the rollback path?
For catalog or schema changes: note which downstream consumers are affected.
-->

## Checklist

- [ ] `npm run validate` passes with no errors
- [ ] `npm run manifest:write` was run and `catalog/skill-manifest.json` is committed (skills changed only)
- [ ] Catalog JSON files updated if an asset was added, moved, or removed
- [ ] No secrets, credentials, tokens, tenant IDs, or customer data included
- [ ] Relevant docs in `docs/` updated if behavior changed
- [ ] Apache-2.0 license terms acknowledged — contributed content must be compatible
- [ ] PR is scoped to one skill, one agent, or one coherent change
