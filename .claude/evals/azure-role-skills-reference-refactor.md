## EVAL DEFINITION: azure-role-skills-reference-refactor

### Capability evals

1. Every Azure role-based skill keeps a lean `SKILL.md` entrypoint and moves heavy detail into lazy-loaded `references/`.
2. Every Azure role-based skill exposes a consistent reference split:
   - `references/mcp-and-evidence.md`
   - `references/workflow-and-output.md`
   - `references/official-sources.md`
3. Every Azure role-based skill still preserves:
   - role or purpose framing,
   - trigger guidance,
   - explicit progressive-disclosure reference links.

### Regression evals

1. `catalog/skills.json` remains valid after the refactor.
2. `catalog/skill-manifest.json` is refreshed to match the changed skill contents.
3. Offline link validation still passes.

### Deterministic checks

- Count Azure skills and ensure each has the three reference files.
- Ensure each Azure `SKILL.md` links to the three local reference files.
- Ensure each Azure `SKILL.md` no longer carries direct Microsoft Learn URLs inline.
- Run `npm run manifest:write`.
- Run `npm run validate`.
