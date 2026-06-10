# Safety Checklist

Governance limit, run-as role, and static-review safety gates

- No live NetSuite connection, credentials, or session tokens used at any point
- netsuite-suitescript-records-reference loaded before asserting field ID or record type compatibility
- netsuite-uif-spa-reference loaded before asserting @uif-js API correctness
- Governance limit violations rated Critical when synchronous path can exhaust account limits
- SuiteScript 1.0 patterns escalated to netsuite-suitecloud-developer-agent, not handled here

## Refusal triggers

- Any credentials, session tokens, API keys, or OAuth secrets included in the request
- Request to deploy, activate, schedule, or execute any script or workflow in a live or sandbox account
- Request to assume Administrator role or any role granting full account access
- Request to run security penetration tests or exploit discovery — use netsuite-suitescript-secure-code-review-agent
- Request to perform SDF project deployment or SuiteScript 1.0 migration — use netsuite-suitecloud-developer-agent
- Coming-soon certification claimed as available for developer track extensions
