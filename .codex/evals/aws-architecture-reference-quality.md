## EVAL DEFINITION: aws-architecture-reference-quality

### Assumptions
- The next weakest AWS reference batch after coordinator/operator fixes is selected by current component-reference audit.
- `aws-waf-*` in this repository means AWS Well-Architected Framework pillar review, not AWS Web Application Firewall review.
- Architecture/review skills need deep reference files that force evidence, tradeoff, and pushback discipline instead of generic best-practice prose.

### Batch Scope
- `aws-generative-ai-developer`
- `aws-waf-reliability-review`
- `aws-waf-security-review`
- `aws-waf-cost-optimization-review`
- `aws-solution-architect`

### Capability Evals
1. Each processed skill has at least one component-specific reference beyond `official-sources.md`, `safety-checklist.md`, and `workflow-and-output.md`.
2. Each new component reference includes:
   - `## What people get wrong`
   - `## Minimum safe workflow`
   - `## Verification targets`
   - `## When to push back`
3. Well-Architected references explicitly avoid confusing WAF pillar review with AWS Web Application Firewall configuration.
4. Each processed `SKILL.md` links its new component reference.
5. Versions are synchronized across `SKILL.md`, `metadata.json`, and `catalog/skills.json`.
6. No internal MCP server/profile/role identifiers appear in touched docs.

### Regression Evals
1. `npm run validate:aws` passes.
2. `npm run manifest:check` passes after manifest regeneration.
3. `npm run validate:skill-schema` passes.
4. `npm run validate:asset-integrity` passes after integrity regeneration.

### Grader
```bash
python3 - <<'PY'
from pathlib import Path
import json, re
skills = {
  'aws-generative-ai-developer': ['bedrock-serverless-genai.md'],
  'aws-waf-reliability-review': ['well-architected-reliability-review.md'],
  'aws-waf-security-review': ['well-architected-security-review.md'],
  'aws-waf-cost-optimization-review': ['well-architected-cost-review.md'],
  'aws-solution-architect': ['architecture-decision-stress-test.md'],
}
cat = {x['id']: x['version'] for x in json.loads(Path('catalog/skills.json').read_text())}
for sid, refs in skills.items():
    base = Path('skills/aws') / sid
    skill = (base / 'SKILL.md').read_text()
    meta = json.loads((base / 'metadata.json').read_text())
    sv = re.search(r'version: "([^"]+)"', skill).group(1)
    assert sv == meta['version'] == cat[sid], sid
    assert sv == '0.1.4', (sid, sv)
    for name in refs:
        assert name in skill, (sid, name)
        txt = (base / 'references' / name).read_text()
        for marker in ['## What people get wrong', '## Minimum safe workflow', '## Verification targets', '## When to push back']:
            assert marker in txt, (sid, name, marker)
        if sid.startswith('aws-waf-'):
            assert 'Well-Architected Framework' in txt and 'not AWS Web Application Firewall' in txt, sid
PY
```
