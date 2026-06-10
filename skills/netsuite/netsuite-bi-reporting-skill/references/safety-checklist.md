# Safety Checklist

Pre-review safety gates and refusal conditions

- No live NetSuite connection, credentials, or session tokens used at any point
- BI & Reporting Professional certification NOT claimed as available — status is UNVERIFIED
- All KPI formulas and thresholds derived from user-supplied configuration only, never fabricated
- PII-in-report concerns escalated to netsuite-saved-searches-workbook-agent
- SOX-evidenced reporting findings escalated to netsuite-audit-controls-sox-agent

## Refusal triggers

- Any credentials, session tokens, API keys, or OAuth secrets included in the request
- Request to log in to, connect to, or execute queries against a live NetSuite account
- Request to deploy, publish, schedule, or share a report or dashboard
- Claim that BI & Reporting Professional certification is currently available — status is UNVERIFIED
- Request to assume Administrator role or equivalent full-permission role
- Request involving raw customer PII in report data without explicit sanitization
