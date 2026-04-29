# AWS guarded live operators eval

[CAPABILITY EVAL: aws-guarded-live-operators]
Task: Add a separate guarded live-AWS operator tier that is stricter than the existing repo-write execution agents.
Success Criteria:
- [ ] Five new AWS live-operation skills exist with target confirmation, approval, rollback, and verification gates.
- [ ] Five matching cross-platform AWS agents exist and link to the right skills.
- [ ] Codex adapters for the live tier are `workspace-write` and explicitly require target confirmation plus approval before live mutation.
- [ ] AWS README files explain the new guarded-live tier without blurring it into the repo-write execution tier.
Grader:
- `python3 tests/validate-catalog.py`
- `python3 tests/validate-aws-skill-quality.py`
- `python3 tests/validate-aws-progressive-disclosure.py`
Expected Output:
- all validators PASS and the new guarded-live roles appear in catalog metadata.

[REGRESSION EVAL: existing-aws-agent-catalog]
Baseline: existing AWS advisory + repo-write execution tier before guarded-live additions.
Tests:
- `python3 tests/validate-catalog.py`: PASS
- `python3 tests/validate-skill-manifest.py`: PASS
- `python3 tests/validate-links.py --offline`: PASS
Result: PASS

## Verification

- `npm run manifest:write` -> PASS
- `python3 tests/validate-aws-skill-quality.py` -> PASS
- `python3 tests/validate-aws-progressive-disclosure.py` -> PASS
- `python3 tests/validate-catalog.py` -> PASS
- `python3 tests/validate-skill-manifest.py` -> PASS
- `python3 tests/validate-links.py --offline` -> PASS
- `npm run validate` -> PASS

## Status

READY
