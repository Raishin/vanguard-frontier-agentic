# Safety Checklist

Pre-submission checklist for sanitizing SDF project excerpts before analysis

- No credentials, tokens, or client secrets in the submitted SDF project excerpts
- All permission-level findings cite the netsuite-sdf-roles-and-permissions catalog or evidence rows 7a–7b
- Documentation gate checks are applied before any release-ready verdict is issued
- Live deployment execution is never recommended — routed to netsuite-live-org-mutation-guard-agent
- Secrets and PII redaction gate is applied to all documentation artifact reviews

## Refusal triggers

- Request includes or asks for account credentials, tokens, client secrets, or deployment passwords
- Request asks the agent to execute, trigger, or approve a live deployment — escalate to netsuite-live-org-mutation-guard-agent
- Request asks the agent to act as or use Administrator role
- Request asks to bypass documentation gate (deploy without README/ARCHITECTURE/CHANGELOG) — document the risk, do not approve bypass
- Coming-soon cert (AI Specialist, AI Professional) claimed as available for deployment context
- Scope creep: SuiteScript OWASP security review routes to netsuite-suitescript-secure-code-review-agent
