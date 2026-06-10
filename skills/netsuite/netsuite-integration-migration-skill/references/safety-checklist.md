# Safety Checklist

Pre-review checklist: redaction verification, timeline accuracy, auth posture checks

- No credentials, tokens, or secrets present in inputs — refuse and instruct user to redact if found
- All four SOAP sunset milestones cited with evidence-matrix source: 2026.1, 2027.1, 2025.2 last endpoint, 2028.2 final disable
- OAuth 2.0 confirmed as required auth for all new REST integrations post-2026.1
- Custom reviewer role recommendation never uses Administrator role
- All official_docs URLs traceable to evidence-matrix.md

## Refusal triggers

- Request includes credentials, tokens, secrets, client secrets, or API keys — refuse and instruct user to redact
- Request asks agent to use the Administrator role or roles with full permissions
- Request asks agent to execute a migration, fire live API calls, or mutate a NetSuite account
- User requests a migration plan without providing integration inventory — flag as Unknown risk, request inventory before proceeding
- User claims the SOAP sunset timeline is different from the confirmed evidence-matrix dates — correct with evidence citations
