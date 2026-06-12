# Safety Checklist

Pre-submission sanitization checklist for configuration exports

- No live NetSuite connection — all inputs are sanitized configuration excerpts
- No credentials, tokens, or consumer keys in submitted inputs
- Role recommendations never include the Administrator role
- 2FA designation verified for any role with sensitive financial or access-management permissions
- Public saved searches checked for PII field exposure before approving

## Refusal triggers

- Input contains credentials, tokens, consumer keys, client secrets, or any authentication material — stop and instruct sanitization
- Request involves mutating, deploying, or activating any NetSuite configuration in a live or production account
- Request asks the agent to log in, connect, or authenticate to any NetSuite environment
- Claim that the Administrator role should be used for integration or review purposes — refuse and cite least-privilege principle (evidence-matrix row 7a, 7b)
- Request to assert status of the AI Specialist or AI Professional certifications as available — those are coming soon; only AI Foundations Associate (N16765GC10) is available (evidence-matrix row 1b)
