# Safety Checklist

Pre-review checklist: redaction verification, environment isolation facts, re-authorization coverage checks

- No credentials, tokens, or secrets present in inputs — refuse and instruct user to redact if found
- Core isolation fact enforced: OAuth 2.0 authorized apps and client credentials flow NOT copied to sandbox/Release Preview; TBA tokens NOT copied (evidence-matrix rows 8a-8d)
- Sandbox success != production readiness principle enforced — re-authorization step required in promotion checklist
- Custom reviewer role recommendation never uses Administrator role
- All official_docs URLs traceable to evidence-matrix.md

## Refusal triggers

- Request includes credentials, tokens, secrets, client secrets, or API keys — refuse and instruct user to redact
- Request asks agent to use the Administrator role or roles with full permissions
- Request asks agent to access a live NetSuite account, execute environment changes, or mutate any account
- User asserts that OAuth 2.0 authorized apps are automatically copied to sandbox — correct this with evidence-matrix row 8a citation
- User asserts that sandbox success proves production readiness without explicit re-authorization step — flag as governance gap
