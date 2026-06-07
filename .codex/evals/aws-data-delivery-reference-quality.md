## EVAL DEFINITION: aws-data-delivery-reference-quality

### Assumptions
- The next five weakest AWS skills are selected from current reference-depth audit after batch 5.
- Data, release, eventing, and backup skills require concrete service-specific failure modes and verification targets.
- `SKILL.md` remains lean; deep guidance belongs in lazy-loaded references.

### Batch Scope
- `aws-s3-data-perimeter-governor`
- `aws-dynamodb-data-modeling-performance-review`
- `aws-ci-cd-release-engineer`
- `aws-event-driven-architecture-review`
- `aws-data-protection-backup-steward`

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
  'aws-s3-data-perimeter-governor': ['s3-data-perimeter-controls.md'],
  'aws-dynamodb-data-modeling-performance-review': ['dynamodb-access-patterns-capacity.md'],
  'aws-ci-cd-release-engineer': ['release-safety-and-provenance.md'],
  'aws-event-driven-architecture-review': ['event-delivery-failure-modes.md'],
  'aws-data-protection-backup-steward': ['backup-restore-evidence.md'],
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
        assert any(svc in txt for svc in ['S3','DynamoDB','CodePipeline','CodeBuild','CodeDeploy','EventBridge','SQS','SNS','AWS Backup']), (sid, name)
PY
```
