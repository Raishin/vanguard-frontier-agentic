# Sandbox Promotion Checklist

Sandbox-to-production promotion checklist including OAuth re-authorization and smoke-test steps

Scope: Sandbox and non-production environment separation, OAuth 2.0 app re-authorization requirements per environment, TBA token isolation, sandbox refresh cycles, and Release Preview usage governance. Enforces the principle that authorized applications and tokens are not copied between environments and must be explicitly re-authorized after each sandbox refresh.

- Sandbox environment separation and governance policy review
- Release Preview account usage governance and change-risk assessment
- OAuth 2.0 authorized application re-authorization procedures per environment and post-refresh
- OAuth 2.0 client credentials flow re-authorization governance across environments
- TBA token lifecycle and isolation governance across production, sandbox, and Release Preview
- Sandbox refresh cycle planning and impact on active integration test coverage
- Sandbox-to-production promotion readiness checklist design
- Environment-specific role and permission configuration review
