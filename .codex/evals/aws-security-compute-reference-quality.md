## EVAL DEFINITION: aws-security-compute-reference-quality

### Assumptions
- The next five weakest AWS skills are selected from current reference-depth audit after batch 6.
- Security, database, and compute operation skills need concrete failure modes and verification targets.
- `SKILL.md` remains lean; deep guidance belongs in lazy-loaded references.

### Batch Scope
- `aws-bedrock-agent-security-governor`
- `aws-kms-secrets-lifecycle-steward`
- `aws-iam-least-privilege-review`
- `aws-rds-aurora-performance-investigator`
- `aws-ec2-compute-operations-steward`

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
  'aws-bedrock-agent-security-governor': ['bedrock-agent-attack-surface.md'],
  'aws-kms-secrets-lifecycle-steward': ['kms-secrets-lifecycle-controls.md'],
  'aws-iam-least-privilege-review': ['iam-policy-trust-boundaries.md'],
  'aws-rds-aurora-performance-investigator': ['rds-aurora-performance-evidence.md'],
  'aws-ec2-compute-operations-steward': ['ec2-fleet-operations-safety.md'],
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
        assert any(svc in txt for svc in ['Bedrock','KMS','Secrets Manager','IAM','Access Analyzer','RDS','Aurora','Performance Insights','EC2','Auto Scaling','Systems Manager']), (sid, name)
PY
```
