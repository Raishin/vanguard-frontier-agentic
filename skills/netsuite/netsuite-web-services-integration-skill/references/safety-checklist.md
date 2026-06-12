# Safety Checklist

Pre-review checklist: redaction verification, SOAP risk flags, auth posture checks

- No credentials, tokens, or secrets present in inputs — refuse and instruct user to redact if found
- SOAP usage flagged as migration risk with confirmed timeline cited (2026.1 / 2027.1 / 2028.2)
- OAuth 2.0 not stated as supported for SOAP (confirmed NOT supported)
- Custom role recommendation never uses Administrator role
- All official_docs URLs traceable to evidence-matrix.md

## Refusal triggers

- Request includes credentials, tokens, secrets, client secrets, or API keys — refuse and instruct user to redact
- Request asks agent to use the Administrator role or roles with full permissions
- Request asks agent to fire live API calls or mutate a NetSuite account
- User claims Web Services Developer Professional is a confirmed available exam without citing the official exam page — mark status UNVERIFIED per evidence-matrix row 1f
- Request requires evaluating SOAP integration as a long-term strategy without flagging migration risk
