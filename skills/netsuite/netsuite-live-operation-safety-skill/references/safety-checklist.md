# Safety Checklist

Expanded live-op authorization checklist with per-change-type decision trees

- No credentials, tokens, or secrets were supplied in the request input
- Administrator role is not required by the proposed change
- Authorized live-op protocol reference is present and cites a named human decision owner
- Rollback plan is documented and a rollback owner is named
- For SDF deploys: sandbox validation evidence is present
- For OAuth/TBA changes: confirms REST+OAuth2 path (not new SOAP TBA post-2027.1)
- For permission changes: target role is a custom copy of a standard role, not Administrator
- For AI Connector changes: confirms account is not a healthcare BAA-restricted account

## Refusal triggers

- Request supplies credentials, tokens, OAuth client secrets, TBA token values, or session cookies — hard refuse, do not echo or log
- Request asks for or implies use of the Administrator role for any automated or scripted operation
- No authorized live-op protocol or change-management ticket reference is present
- No named human decision owner is identified
- No rollback plan is provided for production-bound changes
- Request proposes building a new SOAP integration after the 2026.1 release (REST+OAuth2 is required for new builds per evidence item 2a)
- Request proposes new TBA for SOAP, REST, or RESTlets after 2027.1 (hard block per evidence item 4d)
- Proposed change would grant permissions that mandate 2FA (Access Token Management, OAuth 2.0 Authorized Applications Management, Core Administration Permissions, View Unencrypted Credit Cards, View Unencrypted ACH Account Numbers) without confirming 2FA enrollment
- Coming-soon certifications (AI Specialist, AI Professional, BI & Reporting Professional) cited as available in the change justification
