## EVAL DEFINITION: aws-coordinator-reference-quality

### Assumptions
- The next weakest AWS skill batch is selected by current reference-depth audit: five skills with zero component-specific references and the lowest reference word counts.
- Coordinator/advisor skills still need domain-specific references because generic non-destructive language is not enough to prevent bad triage, cost, change, or automation advice.
- These skills should stay lean in `SKILL.md`; detailed behavior belongs in lazy-loaded references.

### Batch Scope
- `aws-change-impact-advisor`
- `aws-non-destructive-task-automation-advisor`
- `aws-daily-operations-briefing-coordinator`
- `aws-ticket-triage-escalation-coordinator`
- `aws-cost-anomaly-watch-coordinator`

### Capability Evals
1. Each processed skill has at least one component-specific reference beyond `official-sources.md`, `safety-checklist.md`, and `workflow-and-output.md`.
2. Each new component reference includes:
   - `## What people get wrong`
   - `## Minimum safe workflow`
   - `## Verification targets`
   - `## When to push back`
3. Each new component reference includes domain-specific failure modes, not generic cloud best-practice filler.
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
  'aws-change-impact-advisor': ['change-impact-assessment.md'],
  'aws-non-destructive-task-automation-advisor': ['non-destructive-automation-patterns.md'],
  'aws-daily-operations-briefing-coordinator': ['operations-briefing-signal-quality.md'],
  'aws-ticket-triage-escalation-coordinator': ['ticket-triage-escalation.md'],
  'aws-cost-anomaly-watch-coordinator': ['cost-anomaly-triage.md'],
}
cat = {x['id']: x['version'] for x in json.loads(Path('catalog/skills.json').read_text())}
for sid, refs in skills.items():
    base = Path('skills/aws') / sid
    skill = (base / 'SKILL.md').read_text()
    meta = json.loads((base / 'metadata.json').read_text())
    sv = re.search(r'version: "([^"]+)"', skill).group(1)
    assert sv == meta['version'] == cat[sid], sid
    assert sv == '0.1.2', (sid, sv)
    for name in refs:
        assert name in skill, (sid, name)
        txt = (base / 'references' / name).read_text()
        for marker in ['## What people get wrong', '## Minimum safe workflow', '## Verification targets', '## When to push back']:
            assert marker in txt, (sid, name, marker)
        assert 'failure' in txt.lower() or 'risk' in txt.lower(), (sid, name, 'domain-specific failure/risk missing')
PY
```
