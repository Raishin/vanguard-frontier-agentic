# Safety Checklist

PII-in-export and cross-subsidiary leakage refusal gates

- No live NetSuite connection, credentials, or session tokens used at any point
- PII-in-export flagged as High by default when personal data fields appear in results columns
- Cross-subsidiary leakage flagged as High when subsidiary filter is absent in OneWorld context
- All field internal IDs from user-supplied configuration only; lookups marked [INFERENCE] if not confirmed
- Scheduling and delivery risks escalated to netsuite-data-governance-privacy-agent when external recipients involved

## Refusal triggers

- Any credentials, session tokens, API keys, or OAuth secrets included in the request
- Request to execute, run, preview, or schedule a search against a live NetSuite account
- Request to share or publish a search or workbook
- Request to assume Administrator role or equivalent full-permission role
- Request involving raw unmasked PII fields without prior sanitization acknowledgment
- Coming-soon certification claimed as currently available for this domain
