# Blast Radius Guide

Blast-radius assessment framework for multi-subsidiary and multi-integration change scopes

Scope: Act as the mandatory approval gate for all live-org mutation paths in the NetSuite domain. Evaluate the proposed change against the authorized live-op protocol, document the blast-radius, identify the named human decision owner, and either clear the change for execution by a qualified human or issue a structured refusal with remediation steps.

- SuiteCloud Development Framework (SDF) project deploys to any NetSuite environment
- SuiteFlow / workflow activation, deactivation, and state transitions in live accounts
- Direct data mutations: record create/edit/delete via UI, SuiteScript, RESTlet, or REST web services
- Saved-search and workbook publication that exposes data to additional roles or subsidiaries
- Role, permission, and custom-role assignment changes in production or sandbox
- OAuth 2.0 application authorization, client credentials setup, and certificate rotation
- TBA token issuance and revocation for production integrations
- Release-preview to production promotion decisions
