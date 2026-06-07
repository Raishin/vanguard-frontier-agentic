## EVAL REPORT: cloud-agents-reference-quality-oci-agents-002

### Scope
- Provider: OCI
- Asset type: agents
- Batch size: exactly 5
- Items:
  - agents/oci/oci-database-platform-dba-agent
  - agents/oci/oci-dbtools-sql-analyst-agent
  - agents/oci/oci-devops-container-platform-engineer-agent
  - agents/oci/oci-exadata-platform-architect-agent
  - agents/oci/oci-fusion-apps-environment-operator-agent

### Evidence used
- Documentation-based: official OCI docs for Database Cloud Service/Data Guard, Database Tools and SQL Worksheet, DevOps OKE deployment pipelines, Container Registry, Exadata patch/update operations, and Fusion Applications Environment Management.
- Read-only OCI API/MCP evidence: no callable read-only OCI MCP was available in this session; agent guidance now phrases this generically as OCI API evidence through the user’s configured read-only OCI MCP when available.
- Inference: stale guidance was identified from repo text that asked for connector/server labels or assumed an OCI default CLI profile.

### Capability Evals
- stale-connector-label-guidance-removed: PASS - targeted grep found no exact stale phrases in batch 2: `ask only for the configured MCP server name`, `hard-coded MCP server name`, `MCP server name`, `configured server name`.
- default-profile-assumption-removed: PASS - stale `Default to OCI default profile` guidance removed; replacement requires explicit user-provided or confirmed profile.
- evidence-labeling-tightened: PASS - agent guidance now labels `sampled OCI API evidence`, `user-provided sanitized evidence`, `documentation-based`, or `inference`.
- version-bumps: PASS - all five AGENT.md frontmatter, metadata.json, and catalog/agents.json versions are `0.2.1`.

### Regression Evals
- validate-agent-schema: PASS - `npm run validate:agent-schema` exit 0.
- validate-asset-integrity: PASS - `npm run validate:asset-integrity` exit 0.
- full-validate: PASS - `npm run validate` exit 0.

### Commands
- `npm run plugin-manifest:write` -> PASS
- `npm run cursor-plugin:write` -> PASS
- `npm run kiro-powers:write` -> PASS
- `python3 tests/validate-asset-integrity.py --write` -> PASS
- `npm run validate:agent-schema` -> PASS
- `npm run validate:asset-integrity` -> PASS
- `npm run validate` -> PASS
