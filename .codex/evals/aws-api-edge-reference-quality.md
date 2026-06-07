## EVAL DEFINITION: aws-api-edge-reference-quality

### Assumptions
- `aws-agentcore/references` is the quality baseline because it uses component-specific references, anti-patterns, non-negotiables, verification targets, and source-grounded deltas.
- `aws-api-edge-delivery-review/references` must be upgraded from generic review prose into service-specific API Gateway, CloudFront, WAF/Shield, and observability guidance.
- Internal MCP server names, internal role names, account IDs, credentials, customer data, and private identifiers must not appear in skill docs.

### Capability Evals
1. API edge references include at least four component-specific reference files beyond the generic `official-sources`, `safety-checklist`, and `workflow-and-output` files.
2. Each component-specific reference includes:
   - `What people get wrong`
   - service-specific controls or failure modes
   - minimum safe workflow or verification targets
   - pushback criteria / anti-patterns
3. `SKILL.md` links the new progressive-disclosure references without bloating the main skill body.
4. `official-sources.md` includes official source URLs, live/read-only sampled evidence, and specific stale/missing deltas.
5. Versions stay synchronized across `SKILL.md`, `metadata.json`, and `catalog/skills.json`.

### Regression Evals
1. `npm run validate:aws` passes.
2. `npm run manifest:check` passes after regenerating skill manifest.
3. `npm run validate:skill-schema` passes.
4. `npm run validate:asset-integrity` passes after regenerating asset integrity.

### Graders
```bash
python3 - <<'PY'
from pathlib import Path
base = Path('skills/aws/aws-api-edge-delivery-review')
refs = base / 'references'
component_refs = [
    refs / 'api-gateway-controls.md',
    refs / 'cloudfront-origin-protection.md',
    refs / 'waf-shield-abuse-controls.md',
    refs / 'observability-incident-playbook.md',
]
for p in component_refs:
    txt = p.read_text()
    assert '## What people get wrong' in txt, p
    assert '## Minimum safe' in txt or '## Minimal safe' in txt, p
    assert '## When to push back' in txt, p
skill = (base / 'SKILL.md').read_text()
for p in component_refs:
    assert p.name in skill, p
source = (refs / 'official-sources.md').read_text()
for marker in ['Service facts from official docs:', 'Sampled live evidence:', 'Stale or missing guidance corrected:']:
    assert marker in source, marker
PY
npm run validate:aws
npm run manifest:check
npm run validate:skill-schema
npm run validate:asset-integrity
```
