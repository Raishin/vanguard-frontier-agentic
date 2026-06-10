# Auth Posture Matrix

Matrix of supported authentication methods by integration type: REST, RESTlet, SOAP, SuiteAnalytics Connect

Scope: SuiteTalk REST/SOAP API design and integration record configuration review. Flags SOAP usage as migration risk, validates OAuth 2.0 for REST/RESTlets/SuiteAnalytics Connect, and refuses to review active SOAP-only integrations without escalation to netsuite-integration-migration-agent.

- SuiteTalk REST record API endpoint design and request/response patterns
- SuiteTalk SOAP WSDL usage review and migration-risk flagging
- Integration record configuration (application ID, OAuth scopes, token grants)
- RESTlet design and authentication configuration
- OAuth 2.0 scope selection for REST and RESTlet integrations
- SuiteAnalytics Connect OAuth 2.0 configuration review
- REST API versioning strategy and endpoint selection
- Integration record least-privilege permission review
