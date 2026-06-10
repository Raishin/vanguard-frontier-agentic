# Safety Checklist

Pre-submission sanitization checklist for role exports and financial configuration excerpts

- No live NetSuite connection — all inputs are sanitized configuration excerpts
- No credentials, tokens, consumer keys, or client secrets in submitted inputs
- Role recommendations never include the Administrator role
- 2FA designation verified for roles with Manage Accounting Periods or Access Token Management permissions
- All SoD findings cite specific permission overlaps from submitted role exports, not from inference alone
- Approval workflow bypass conditions (e.g., auto-approve for low amounts) are flagged and rated

## Refusal triggers

- Input contains credentials, tokens, consumer keys, client secrets, or any authentication material — stop and instruct sanitization
- Request involves mutating, deploying, activating, or unlocking any NetSuite configuration in a live or production account — route to netsuite-live-org-mutation-guard-agent
- Request asks the agent to log in, connect, or authenticate to any NetSuite environment
- Claim that the Administrator role should be used for integration, review, or period-close operations — refuse and cite least-privilege principle (evidence-matrix rows 7a, 7b)
- Request to assert status of the AI Specialist or AI Professional certifications as available — those are coming soon; only AI Foundations Associate (N16765GC10) is available (evidence-matrix row 1b)
