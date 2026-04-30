# Terraform analyzer fixture eval

[CAPABILITY EVAL: terraform-analyzer-fixtures]
Task: Prove the Terraform AzureRM, OCI, and AWS diff analyzers against real fixture plan JSON instead of only static metadata checks.
Success Criteria:
- [ ] AzureRM analyzer passes an order-only fixture and an actual-change fixture.
- [ ] OCI analyzer passes an order-only fixture and an actual-change fixture.
- [ ] AWS analyzer passes an order-only fixture and an actual-change fixture.
- [ ] Each fixture run is graded by deterministic subprocess checks against JSON output and exit code behavior.
Grader:
- `python3 tests/validate-terraform-analyzer-fixtures.py`
- `python3 tests/validate-terraform-agent-quality.py`
Expected Output:
- provider analyzers classify fixture plans correctly and return expected exit codes.

[REGRESSION EVAL: terraform-validation-stack]
Baseline: Terraform tier and specialized-skill validators already pass before fixture additions.
Tests:
- `python3 tests/validate-catalog.py`: PASS
- `python3 tests/validate-skill-manifest.py`: PASS
- `python3 tests/validate-links.py --offline`: PASS
- `npm run validate`: PASS
Result: PASS

## Verification

- `npm run manifest:write` -> PASS
- `python3 tests/validate-terraform-analyzer-fixtures.py` -> PASS
- `python3 tests/validate-terraform-agent-quality.py` -> PASS
- `python3 tests/validate-catalog.py` -> PASS
- `python3 tests/validate-skill-manifest.py` -> PASS
- `python3 tests/validate-links.py --offline` -> PASS
- `npm run validate` -> PASS

## Status

READY
