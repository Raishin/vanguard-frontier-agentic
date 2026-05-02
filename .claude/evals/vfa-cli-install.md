## EVAL DEFINITION: vfa-cli-install

### Capability Evals
E-01: --list prints all 141 agents (tab-separated id/provider/name, sorted)
E-02: --list-roles prints all 15 roles with agent + skill counts
E-03: --platform claude-code --agents <id> installs a single agent file
E-04: --platform codex --agents <id> installs correct .toml file
E-05: --platform kiro --agents <id> installs BOTH kiro-ide .md AND kiro-cli .json
E-06: --platform claude-code --role kubernetes-rbac-review installs all role agents
E-07: --provider filter narrows role install to matching provider prefix
E-08: --force overwrites existing file without error
E-09: Without --force, second install refuses to overwrite
E-10: --all installs all 141 agents for the platform

### Error Handling Evals
E-11: Unknown agent id exits non-zero with clear message
E-12: Unknown role exits non-zero with clear message
E-13: Unknown platform exits non-zero
E-14: Invalid --provider (injection chars) exits non-zero
E-15: Unknown platform alias exits non-zero
E-16: Missing --platform when using --agents exits non-zero
E-17: --role with --provider that matches 0 agents exits non-zero

### Platform Alias Evals
E-18: --platform claude (alias) resolves to claude-code
E-19: --platform kiroide (alias) resolves to kiro-ide

### Regression Evals
E-20: All 141 agents have metadata.json harness_variants covering all expected keys
E-21: New CNCF agents appear in --list output
E-22: New roles appear in --list-roles output
E-23: kubernetes-live-velero-restore-guard-agent installs with codex.toml using workspace-write

### Success Metrics
- Capability evals: pass@1 = 100% (deterministic behavior)
- Error handling evals: pass@1 = 100% (exits non-zero with message)
- Regression evals: pass^3 = 100% (stable)
