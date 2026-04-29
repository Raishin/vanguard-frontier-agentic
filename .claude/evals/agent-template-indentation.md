# Agent template indentation eval

[CAPABILITY EVAL: agent-template-indentation]
Task: Keep canonical agent markdown and Markdown harness adapters flush-left after frontmatter.
Success Criteria:
- [ ] `agents/**/AGENT.md` files do not start body content with four-space indentation.
- [ ] `agents/**/harnesses/*.agent.md` files do not start body content with four-space indentation.
- [ ] `agents/**/harnesses/codex.toml` files still parse and keep required keys.
Grader:
- `python3 tests/validate-catalog.py`
Expected Output:
- `OK: validated ... catalog entries and scanned for obvious secrets`

[REGRESSION EVAL: existing-agent-validation]
Baseline: repo validator before/after AWS agent template cleanup.
Tests:
- `python3 tests/validate-catalog.py`: PASS
- `python3 tests/validate-skill-manifest.py`: PASS
- `python3 tests/validate-links.py --offline`: PASS
Result: pending
