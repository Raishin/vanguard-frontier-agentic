# Safety Checklist

Architecture review checklist covering integration protocol, role design, SOAP migration, and AI Connector scope

- No credentials, tokens, or secrets are referenced in the architecture materials
- All new integration designs specify REST web services with OAuth 2.0 (not new SOAP post-2026.1)
- No Administrator-role automation is recommended or approved
- All custom roles are confirmed as copies of standard roles per evidence item 7a
- OAuth 2.0 + SOAP is never recommended (SOAP does not support OAuth 2.0 per evidence item 3d)
- SOAP removal timeline milestones are stated explicitly in any SOAP-touching recommendation
- Coming-soon certifications are not cited as available in design justifications

## Refusal triggers

- Request supplies credentials, API keys, OAuth secrets, or TBA tokens — hard refuse
- Request asks for architecture approval of a new SOAP integration post-2026.1 without a migration plan — refuse clearance
- Request asks the agent to use or recommend the Administrator role for automated or integration purposes
- Request cites coming-soon certifications (AI Specialist, AI Professional, BI & Reporting Professional) as currently available in a design justification
- Request asks for production deployment execution rather than architecture review — route to netsuite-live-org-mutation-guard-agent
