# Auth Compatibility Matrix

Protocol-to-auth-method compatibility matrix (REST/RESTlet/SuiteAnalytics/SOAP vs OAuth 2.0/TBA/NLAuth)

Scope: Authentication mechanism design and correctness in NetSuite integrations: OAuth 2.0 applicability scope (REST/RESTlets/SuiteAnalytics Connect only; NOT SOAP), TBA use-cases and sunset timeline, SSO/SAML integration, deprecated NLAuth/Passport patterns, and per-environment re-authorization requirements for sandbox and Release Preview.

- OAuth 2.0 review: Authorization Code flow and Client Credentials flow for REST web services (evidence 3a), RESTlets (evidence 3b), and SuiteAnalytics Connect (evidence 3c); flag OAuth 2.0 applied to SOAP (not supported, evidence 3d)
- TBA review: verify TBA is used only for scenarios where OAuth 2.0 is not yet available; apply 2027.1 new-TBA-block timeline (evidence 4d); confirm SOAP endpoint is 2020.2 or later for TBA (evidence 4c)
- Deprecated authentication patterns: NLAuth / Passport request-level credentials flagged as deprecated for RESTlets (evidence 4b) and SOAP endpoints 2020.2+ (evidence 4c)
- SSO/SAML review: validate integration setup, role mapping, and that required 2FA permissions for SSO setup are designated (evidence 5c)
- Sandbox and Release Preview re-authorization: confirm OAuth 2.0 authorized applications are not assumed to carry over from production (evidence 8a, 8b, 8c); confirm TBA tokens must be recreated in non-production environments (evidence 8d)
- SOAP deprecation risk: apply the four-milestone timeline (2026.1 recommendation, 2027.1 new-SOAP block, 2025.2 last planned endpoint, 2028.2 full sunset) to flag at-risk SOAP + TBA integrations (evidence 2a–2d)
