## EVAL DEFINITION: cloud-agents-reference-quality-azure-agents-003

### Scope
Exactly five Azure agents in stable sorted order after validated batch 002:
1. agents/azure/azure-identity-governance-review-agent
2. agents/azure/azure-key-vault-secret-lifecycle-auditor-agent
3. agents/azure/azure-keyvault-certificate-issuer-review-agent
4. agents/azure/azure-landing-zone-architect-agent
5. agents/azure/azure-live-aks-rollout-guard-agent

### AgentCore reference standard
Agent-local references must match the stronger AgentCore-style pattern: component-specific operations guide with version note, what people get wrong, officially grounded service shape, key insight, non-negotiable rules, minimal safe flow, high-risk assumptions, safe verification targets, and push-back rules. Common references must cover official sources, safety checklist, evidence labels, workflow/output contract, and generic documentation MCP wording.

### Microsoft Learn evidence used
- Microsoft Entra ID Governance least privilege, PIM, access reviews, entitlement management, and role best practices.
- Azure Key Vault secure secrets, RBAC, soft delete, purge protection, recovery, autorotation, Event Grid, and certificate policy/export guidance.
- Azure landing zone design areas, platform/application landing zones, subscription and management group design, governance, security, and platform automation.
- AKS reliability guidance for PDBs, probes, multi-replica applications, rolling updates, maxSurge, maxUnavailable, and rollout/rollback guardrails.

### Capability Evals
- [x] Each target has one component-specific operations guide with AgentCore-standard headings and actionable, source-grounded content.
- [x] Each target has refreshed official-sources, safety-checklist, workflow-and-output, and mcp-and-evidence references.
- [x] Each target bumps patch version in AGENT.md, metadata.json, and catalog/agents.json.
- [x] Existing stale Azure tool phrasing is replaced with approved generic documentation evidence phrasing.
- [x] Agent harnesses remain compact and point to the bound skill/reference discipline without leaking environment-specific identifiers.
- [x] Live AKS adjacent permission/preflight/rollback docs avoid environment-specific IDs and require explicit approval before mutations.

### Regression Evals
- [x] Structural AgentCore heading grader passes.
- [x] Prohibited wording grep passes.
- [x] AWS scoped diff check returns no paths.
- [x] npm run validate:agent-schema passes.
- [x] npm run validate:plugin-manifest passes after plugin manifest regeneration.
- [x] npm run validate:kiro-powers passes after Kiro Powers regeneration.
- [x] npm run validate:multi-harness-marketplace passes.
- [x] npm run validate:codex-marketplace passes.
- [x] npm run validate:asset-integrity passes after integrity regeneration.
- [x] npm run validate passes.

### Verdict
Pass. Full `npm run validate` completed successfully with `VALIDATE_EXIT:0`; Azure agents batch 003 is validated and remains uncommitted.
