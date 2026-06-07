## EVAL DEFINITION: aws-platform-reference-quality

### Assumptions
- The next five weakest AWS skills are selected from the current component-reference audit.
- Platform, security, network, serverless, incident, and migration skills need concrete failure modes and verification targets, not generic “best practices.”
- Detailed guidance belongs in component references; `SKILL.md` remains lean and links the reference.

### Batch Scope
- `aws-network-architect`
- `aws-security-posture-hardening`
- `aws-serverless-production-readiness`
- `aws-observability-incident-responder`
- `aws-migration-cutover-architect`

### Capability Evals
1. Each processed skill has at least one component-specific reference beyond `official-sources.md`, `safety-checklist.md`, and `workflow-and-output.md`.
2. Each new component reference includes:
   - `## What people get wrong`
   - `## Minimum safe workflow`
   - `## Verification targets`
   - `## When to push back`
3. Each new component reference includes domain-specific failure modes and at least one concrete AWS service/control relevant to that skill.
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
  'aws-network-architect': ['network-routing-and-dns.md'],
  'aws-security-posture-hardening': ['security-posture-prioritization.md'],
  'aws-serverless-production-readiness': ['lambda-event-production-readiness.md'],
  'aws-observability-incident-responder': ['incident-evidence-correlation.md'],
  'aws-migration-cutover-architect': ['migration-cutover-readiness.md'],
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
        assert 'failure' in txt.lower() or 'risk' in txt.lower(), (sid, name)
        assert any(svc in txt for svc in ['VPC','Transit Gateway','Security Hub','GuardDuty','Lambda','CloudWatch','Migration Hub','Application Migration Service','Route 53']), (sid, name)
PY
```
