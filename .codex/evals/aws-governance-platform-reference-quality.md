## EVAL DEFINITION: aws-governance-platform-reference-quality

### Assumptions
- The next five weakest AWS skills are selected from current reference-depth audit after batch 4.
- Governance/platform skills need concrete control boundaries, failure modes, and verification targets instead of broad generic safety language.
- `SKILL.md` stays lean; detailed guidance goes into lazy-loaded references.

### Batch Scope
- `aws-landing-zone-governor`
- `aws-iac-change-safety-review`
- `aws-devops-agent-skill-designer`
- `aws-resilience-bcdr-review`
- `aws-eks-platform-operator`

### Capability Evals
1. Each processed skill has at least one component-specific reference beyond generic references.
2. Each new reference includes `## What people get wrong`, `## Minimum safe workflow`, `## Verification targets`, and `## When to push back`.
3. Each reference includes concrete AWS service/control terms relevant to that skill.
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
  'aws-landing-zone-governor': ['landing-zone-governance-controls.md'],
  'aws-iac-change-safety-review': ['iac-change-risk-review.md'],
  'aws-devops-agent-skill-designer': ['devops-agent-skill-quality.md'],
  'aws-resilience-bcdr-review': ['bcdr-recovery-evidence.md'],
  'aws-eks-platform-operator': ['eks-platform-operations.md'],
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
        assert any(svc in txt for svc in ['Control Tower','Organizations','CloudFormation','DevOps Agent','Resilience Hub','EKS','Kubernetes','Application Recovery Controller']), (sid, name)
PY
```
