## EVAL DEFINITION: aws-skills-mcp-refresh

### Assumptions
- AWS skills are the sorted directories under `skills/aws`.
- Each refreshed skill must contain specific service facts and sampled live/read-only evidence in `references/official-sources.md`, not only generic MCP guidance.
- Internal MCP server names, role names, account IDs, credentials, and environment-specific identifiers must not appear in committed skill docs.

### Capability Evals
1. Every processed AWS skill has a `## Current MCP/documentation refresh (2026-06-02)` section in `references/official-sources.md`.
2. That section includes both documentation-based service facts and sampled live/read-only evidence.
3. Version is synchronized across `SKILL.md`, adjacent `metadata.json`, and `catalog/skills.json`.
4. Changed docs do not contain internal MCP server names or internal role/profile identifiers.
5. Generated manifests are refreshed after skill edits.

### Regression Evals
1. `npm run validate:aws` passes.
2. `npm run manifest:check` passes.
3. `npm run validate:skill-schema` passes.
4. `npm run validate:asset-integrity` passes.

### Graders
```bash
python3 - <<'PY'
from pathlib import Path
for p in sorted(Path('skills/aws').glob('aws-*/references/official-sources.md')):
    txt=p.read_text()
    if '## Current MCP/documentation refresh (2026-06-02)' in txt:
        assert 'Documentation-based' in txt or 'Service facts' in txt, p
        assert 'Live availability evidence' in txt or 'Sampled live evidence' in txt or 'live/read-only' in txt, p
PY
python3 - <<'PY2'
from pathlib import Path
needles = ['Safe' + 'Uat' + 'Mcp', 'Safe' + 'Uat' + 'Mcp' + 'ReadOnly' + 'Role']
paths = list(Path('skills/aws').rglob('*')) + [Path('catalog/skills.json'), Path('catalog/skill-manifest.json')]
hits = []
for p in paths:
    if p.is_file():
        txt = p.read_text(errors='ignore')
        for n in needles:
            if n in txt:
                hits.append(str(p))
assert not hits, hits
PY2
npm run validate:aws
npm run manifest:check
npm run validate:skill-schema
npm run validate:asset-integrity
```

### Success Metrics
- Capability evals: pass@1 for each processed batch.
- Regression evals: pass^1 per batch, final pass after all AWS skills are refreshed.
