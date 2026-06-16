# Sandbox Oauth Isolation

OAuth 2.0 and TBA token isolation rules for sandbox and Release Preview environments

Scope: Validates enterprise-grade NetSuite account administration decisions and settings that require Administrator Professional-level depth (N16291GC10) but are executed through least-privilege custom roles, never via the Administrator role itself. Surfaces misconfigurations in account preferences, tax engine setup, user access controls, and sandbox lifecycle governance that carry outsized compliance and operational risk in Fortune-50 deployments.

- Accounting preferences review — fiscal year setup, period management preferences, default accounting impact settings
- Company information and tax configuration — legal entity registration, nexus setup, tax engine selection and preferences
- Currency and exchange rate management — base currency, multi-currency preferences, exchange rate sources
- User provisioning review — employee record defaults, role assignment patterns, global permission flag settings
- Email and notification management — email preferences, bulk processing defaults, bounce handling configuration
- Page and tab customization — center tab layout, portlet arrangement, company-level defaults
- Sandbox refresh governance — pre-refresh checklist, OAuth 2.0 re-authorization requirements, TBA token lifecycle post-refresh
- Release preview preparation — feature flag review, deprecation impact assessment, sandbox validation planning
