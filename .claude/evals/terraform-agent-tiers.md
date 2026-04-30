# Terraform agent tiers eval

[CAPABILITY EVAL: terraform-agent-tiers]
Task: Add a Terraform agent portfolio separated into advisory, repo-write execution, and guarded live operation.
Success Criteria:
- [ ] Terraform advisory skill/agent exists and is read-only in Codex.
- [ ] Terraform repo-write execution skill/agent exists and is workspace-write in Codex but forbids live apply by default.
- [ ] Terraform guarded live skill/agent exists and explicitly requires backend, workspace, lock, plan evidence, and approval before apply-class operations.
- [ ] Terraform provider README files describe the three tiers clearly.
Grader:
- `python3 tests/validate-catalog.py`
- `python3 tests/validate-terraform-agent-quality.py`
Expected Output:
- validators PASS and catalog entries exist.

[REGRESSION EVAL: repo-validation]
Baseline: catalog and manifest validation before Terraform tier additions.
Tests:
- `python3 tests/validate-skill-manifest.py`: PASS
- `python3 tests/validate-links.py --offline`: PASS
Result: PASS

## Verification

- `npm run manifest:write` -> PASS
- `python3 tests/validate-terraform-agent-quality.py` -> PASS
- `python3 tests/validate-catalog.py` -> PASS
- `python3 tests/validate-skill-manifest.py` -> PASS
- `python3 tests/validate-links.py --offline` -> PASS
- `npm run validate` -> PASS

## Status

READY
