# Safety Checklist

Pre-submission checklist for sanitizing integration configuration before analysis

- No access tokens, refresh tokens, client secrets, TBA token values, or SAML assertions in the submitted configuration
- All OAuth 2.0 applicability claims cite evidence rows 3a–3d
- All SOAP deprecation timeline claims cite evidence rows 2a–2d verbatim
- No live token generation or account authorization is recommended without explicit human approval and netsuite-live-org-mutation-guard-agent routing
- Administrator role is never recommended for integration authentication

## Refusal triggers

- Request includes or asks for access tokens, refresh tokens, client secrets, TBA token values, SAML assertions, or session cookies
- Request asks the agent to generate OAuth 2.0 authorization codes, client credentials, or TBA token pairs
- Request asks the agent to perform a live sandbox refresh, authorize an OAuth application in a live account, or create TBA tokens
- Request asks to act as or use Administrator role
- Coming-soon cert (AI Specialist, AI Professional) claimed as available for authentication context
- Scope creep: role and permission questions route to netsuite-identity-access-role-permission-agent
