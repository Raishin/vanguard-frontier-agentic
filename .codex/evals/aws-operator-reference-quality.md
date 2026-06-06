## EVAL DEFINITION: aws-operator-reference-quality

### Assumptions
- Repo-write AWS operator skills need stronger reference quality than static review skills because they can edit files and influence live rollout paths.
- The first operator-quality batch covers:
  - `aws-serverless-rollout-corrector`
  - `aws-deployment-hotfix-operator`
  - `aws-pipeline-fix-operator`
  - `aws-iac-patch-executor`
  - `aws-ecs-service-remediation-operator`
- Each must gain at least one component-specific reference with concrete failure modes, safe workflow, verification targets, and pushback criteria.

### Capability Evals
1. Each processed operator skill has at least one component-specific reference beyond `official-sources.md`, `safety-checklist.md`, and `workflow-and-output.md`.
2. Each new component reference includes:
   - `## What people get wrong`
   - `## Minimum safe workflow`
   - `## Verification targets`
   - `## When to push back`
3. Each processed `SKILL.md` links its new component reference.
4. Versions are synchronized across `SKILL.md`, `metadata.json`, and `catalog/skills.json`.
5. No internal MCP server/profile/role identifiers appear in the touched docs.

### Regression Evals
1. `npm run validate:aws` passes.
2. `npm run manifest:check` passes.
3. `npm run validate:skill-schema` passes.
4. `npm run validate:asset-integrity` passes.

### Grader
```bash
python3 - <<'PY'
from pathlib import Path
import json, re
skills = {
  'aws-serverless-rollout-corrector': ['lambda-rollout-correction.md'],
  'aws-deployment-hotfix-operator': ['deployment-hotfix-safety.md'],
  'aws-pipeline-fix-operator': ['pipeline-failure-analysis.md'],
  'aws-iac-patch-executor': ['iac-patch-safety.md'],
  'aws-ecs-service-remediation-operator': ['ecs-remediation-playbook.md'],
}
cat = {x['id']: x['version'] for x in json.loads(Path('catalog/skills.json').read_text())}
for sid, refs in skills.items():
    base = Path('skills/aws') / sid
    skill = (base / 'SKILL.md').read_text()
    meta = json.loads((base / 'metadata.json').read_text())
    sv = re.search(r'version: "([^"]+)"', skill).group(1)
    assert sv == meta['version'] == cat[sid], sid
    for name in refs:
        assert name in skill, (sid, name)
        txt = (base / 'references' / name).read_text()
        for marker in ['## What people get wrong', '## Minimum safe workflow', '## Verification targets', '## When to push back']:
            assert marker in txt, (sid, name, marker)
PY
```
