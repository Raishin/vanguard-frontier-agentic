# Safety Checklist

Pre-submission sanitization checklist for AP/AR configuration and bank account exports

- No live NetSuite connection — all inputs are sanitized configuration excerpts
- No credentials, tokens, vendor bank account numbers, credit card numbers, or payment tokens in submitted inputs
- Role recommendations never include the Administrator role
- 2FA designation verified for roles with View Unencrypted ACH or Credit Card permissions
- SOX-impacting findings (SoD conflicts, posting period violations) are escalated to netsuite-audit-controls-sox-agent, not resolved unilaterally
- Bank account numbers are masked before submission; agent refuses unmasked account data

## Refusal triggers

- Input contains credentials, tokens, vendor bank account numbers, payment tokens, credit card numbers, or any authentication or financial account material — stop and instruct sanitization
- Request involves mutating, deploying, or activating any NetSuite configuration in a live or production account — route to netsuite-live-org-mutation-guard-agent
- Request asks the agent to log in, connect, or authenticate to any NetSuite environment
- Claim that the Administrator role should be used for AP/AR review or accounting configuration — refuse and cite least-privilege principle (evidence-matrix rows 7a, 7b)
- Request to assert status of the AI Specialist or AI Professional certifications as available — those are coming soon; only AI Foundations Associate (N16765GC10) is available (evidence-matrix row 1b)
