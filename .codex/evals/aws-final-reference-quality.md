## EVAL DEFINITION: aws-final-reference-quality

### Assumptions
- These are the final five AWS skills without component-specific references in the current audit.
- Completion requires every AWS skill to have at least one domain/component reference beyond `official-sources.md`, `safety-checklist.md`, and `workflow-and-output.md`.
- `aws-maestro` is a router, so its component reference should cover routing failure modes rather than AWS service implementation guidance.

### Batch Scope
- `aws-cost-optimization-governor`
- `aws-compliance-evidence-mapper`
- `aws-ecs-fargate-platform-operator`
- `aws-private-ca-issuer-review`
- `aws-maestro`

### Capability Evals
1. Each processed skill has at least one component-specific reference beyond generic references.
2. Each new reference includes `## What people get wrong`, `## Minimum safe workflow`, `## Verification targets`, and `## When to push back`.
3. Each processed `SKILL.md` links its new component reference.
4. Versions are synchronized across `SKILL.md`, `metadata.json`, and `catalog/skills.json`.
5. Every AWS skill now has at least one component-specific reference.
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
generic={'official-sources.md','safety-checklist.md','workflow-and-output.md'}
skills = {
  'aws-cost-optimization-governor': ['cost-optimization-risk-governance.md'],
  'aws-compliance-evidence-mapper': ['compliance-evidence-chain.md'],
  'aws-ecs-fargate-platform-operator': ['ecs-fargate-service-safety.md'],
  'aws-private-ca-issuer-review': ['private-ca-issuer-trust-boundaries.md'],
  'aws-maestro': ['routing-quality-and-safety.md'],
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
for d in sorted(p for p in Path('skills/aws').iterdir() if p.is_dir() and (p/'SKILL.md').exists()):
    refs=d/'references'
    files=sorted(p.name for p in refs.glob('*.md')) if refs.exists() else []
    comp=[f for f in files if f not in generic]
    assert comp, f'missing component reference: {d.name}'
PY
```
