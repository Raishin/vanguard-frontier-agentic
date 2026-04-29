## EVAL DEFINITION: aws-proactive-non-destructive-agents

### Assumptions
- AWS-specialized proactive roles should remain non-destructive and read-only.
- Existing AWS technical specialist roles remain unchanged and are not replaced by these new coordination roles.
- The repo uses `.claude/evals/` already, so eval artifacts stay there.

### Capability Evals
1. New AWS skills exist for proactive non-destructive business and automation coordination.
2. Matching AWS agents exist and link to the corresponding skills.
3. Every new Codex adapter remains `sandbox_mode = "read-only"`.
4. New role prompts explicitly reject destructive or mutation-heavy actions by default.

### Regression Evals
1. `python3 tests/validate-aws-skill-quality.py` passes.
2. `python3 tests/validate-catalog.py` passes.
3. `npm run manifest:write` updates the skill manifest cleanly.
4. `npm run validate` still passes.

### Success Metrics
- Capability evals: pass@1 = 1.00 for structural checks
- Regression evals: pass^1 = 1.00 for deterministic repo validators

## EVAL REPORT: aws-proactive-non-destructive-agents

### Capability Evals
- new-aws-skills-present: PASS - 5 new AWS skills added under `skills/aws/`
- matching-aws-agents-present: PASS - 5 new AWS agents added under `agents/aws/`
- codex-read-only: PASS - new Codex adapters use `sandbox_mode = "read-only"`
- non-destructive-contract-explicit: PASS - new skills and agents say non-destructive / read-only by default

### Regression Evals
- aws-skill-quality: PASS - `python3 tests/validate-aws-skill-quality.py`
- validate-catalog: PASS - `python3 tests/validate-catalog.py`
- manifest-write: PASS - `npm run manifest:write`
- full-validate: PASS - `npm run validate`

### Status
READY
