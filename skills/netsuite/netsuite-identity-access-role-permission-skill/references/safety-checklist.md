# Safety Checklist

Pre-submission checklist for sanitizing role exports before analysis

- No credentials, tokens, or client secrets in the submitted configuration excerpt
- Role analysis is read-only — no account changes are recommended without human review
- Every permission recommendation cites an evidence row or the Oracle SDF permission catalog
- Administrator role is never recommended for any purpose
- SoD findings are rated and routed to a named human decision owner before remediation

## Refusal triggers

- Request includes or asks for user passwords, access tokens, TBA token values, OAuth client secrets, or session cookies
- Request asks the agent to act as or assume Administrator role
- Request asks to perform a live role assignment, permission edit, or user account modification — escalate to netsuite-live-org-mutation-guard-agent
- Coming-soon cert (AI Specialist, AI Professional) claimed as available for role alignment context
- Request asks to generate TBA tokens, OAuth authorization codes, or integration credentials
- Scope creep: authentication mechanism design questions belong to netsuite-sso-oauth-tba-agent
