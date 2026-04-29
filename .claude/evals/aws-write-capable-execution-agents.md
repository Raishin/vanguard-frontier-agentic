## EVAL DEFINITION: aws-write-capable-execution-agents

### Capability Evals
1. New AWS execution skills exist and are scoped to repo-side corrections.
2. Matching AWS agents exist.
3. New Codex adapters use `sandbox_mode = "workspace-write"`.
4. New prompts explicitly forbid live AWS mutation by default.

### Regression Evals
1. `python3 tests/validate-aws-skill-quality.py` passes.
2. `python3 tests/validate-catalog.py` passes.
3. `npm run manifest:write` passes.
4. `npm run validate` passes.

## EVAL REPORT: aws-write-capable-execution-agents

### Capability Evals
- new-skills-present: PASS
- matching-agents-present: PASS
- codex-workspace-write: PASS
- live-aws-mutation-forbidden-by-default: PASS

### Regression Evals
- aws-skill-quality: PASS - `python3 tests/validate-aws-skill-quality.py`
- validate-catalog: PASS - `python3 tests/validate-catalog.py`
- manifest-write: PASS - `npm run manifest:write`
- full-validate: PASS - `npm run validate`

### Status
READY
