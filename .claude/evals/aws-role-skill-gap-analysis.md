## EVAL DEFINITION: aws-role-skill-gap-analysis

### Capability evals

1. The repo includes a detailed AWS role-based skill gap analysis document.
2. The analysis separates AWS-official documentation from community marketplace signal.
3. The analysis identifies P0 missing AWS role/scenario skills with trigger, evidence, output, and safety rationale.
4. The analysis explicitly includes AWS DevOps Agent skill design implications, including scenario-level skills and learned/tool-use best practices.
5. The analysis includes anti-bloat gates so new skills are not created only because they appear in skills.sh.
6. The analysis includes a skill-comply plan with supportive, neutral, and competing prompts.

### Regression evals

1. Existing AWS skill files remain valid.
2. Catalog validation still passes.
3. Skill manifest validation still passes.
4. Offline link validation still passes.

### Deterministic checks

```bash
test -f docs/aws-role-based-skill-gap-analysis.md
grep -q "aws-iac-change-safety-review" docs/aws-role-based-skill-gap-analysis.md
grep -q "aws-rds-aurora-performance-investigator" docs/aws-role-based-skill-gap-analysis.md
grep -q "aws-bedrock-agent-security-governor" docs/aws-role-based-skill-gap-analysis.md
grep -q "aws-ecs-fargate-platform-operator" docs/aws-role-based-skill-gap-analysis.md
grep -q "skills.sh is not AWS official" docs/aws-role-based-skill-gap-analysis.md
npm run validate
```

### Human review questions

1. Are the P0 skills the right next implementation slice, or should one be demoted?
2. Does the board critique reject enough low-value skill bloat?
3. Are any enterprise-critical AWS roles missing from the backlog?

## IMPLEMENTATION EVAL ADDENDUM: all-missing-aws-skills

### Added skill checks

- [ ] `aws-iac-change-safety-review` exists with `SKILL.md`, `metadata.json`, catalog entry, official docs, and safety notes.
- [ ] `aws-rds-aurora-performance-investigator` exists with `SKILL.md`, `metadata.json`, catalog entry, official docs, and safety notes.
- [ ] `aws-bedrock-agent-security-governor` exists with `SKILL.md`, `metadata.json`, catalog entry, official docs, and safety notes.
- [ ] `aws-ecs-fargate-platform-operator` exists with `SKILL.md`, `metadata.json`, catalog entry, official docs, and safety notes.
- [ ] `aws-event-driven-architecture-review` exists with `SKILL.md`, `metadata.json`, catalog entry, official docs, and safety notes.
- [ ] `aws-s3-data-perimeter-governor` exists with `SKILL.md`, `metadata.json`, catalog entry, official docs, and safety notes.
- [ ] `aws-dynamodb-data-modeling-performance-review` exists with `SKILL.md`, `metadata.json`, catalog entry, official docs, and safety notes.
- [ ] `aws-ci-cd-release-engineer` exists with `SKILL.md`, `metadata.json`, catalog entry, official docs, and safety notes.
- [ ] `aws-devops-agent-skill-designer` exists with `SKILL.md`, `metadata.json`, catalog entry, official docs, and safety notes.
- [ ] `aws-ec2-compute-operations-steward` exists with `SKILL.md`, `metadata.json`, catalog entry, official docs, and safety notes.
- [ ] `aws-api-edge-delivery-review` exists with `SKILL.md`, `metadata.json`, catalog entry, official docs, and safety notes.
- [ ] `aws-kms-secrets-lifecycle-steward` exists with `SKILL.md`, `metadata.json`, catalog entry, official docs, and safety notes.
- [ ] `aws-compliance-evidence-mapper` exists with `SKILL.md`, `metadata.json`, catalog entry, official docs, and safety notes.
- [ ] `aws-hybrid-multicloud-connectivity-review` exists with `SKILL.md`, `metadata.json`, catalog entry, official docs, and safety notes.

### Deterministic checks after implementation

```bash
python3 - <<'PY'
import json, pathlib, sys
root=pathlib.Path('.')
ids = [
    "aws-iac-change-safety-review",
    "aws-rds-aurora-performance-investigator",
    "aws-bedrock-agent-security-governor",
    "aws-ecs-fargate-platform-operator",
    "aws-event-driven-architecture-review",
    "aws-s3-data-perimeter-governor",
    "aws-dynamodb-data-modeling-performance-review",
    "aws-ci-cd-release-engineer",
    "aws-devops-agent-skill-designer",
    "aws-ec2-compute-operations-steward",
    "aws-api-edge-delivery-review",
    "aws-kms-secrets-lifecycle-steward",
    "aws-compliance-evidence-mapper",
    "aws-hybrid-multicloud-connectivity-review",
]
cat={i["id"]: i for i in json.load(open(root/"catalog/skills.json"))}
for sid in ids:
    d=root/"skills/aws"/sid
    assert (d/"SKILL.md").exists, sid
    assert (d/"metadata.json").exists, sid
    assert sid in cat, sid
    assert cat[sid]["version"] == json.load(open(d/"metadata.json"))["version"], sid
print("all added AWS skill checks passed")
PY
npm run validate
```
