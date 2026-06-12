# Safety Checklist

Pre-submission sanitization checklist for AI feature and AI Connector configuration exports

- No live NetSuite connection — all inputs are sanitized configuration excerpts
- No credentials, tokens, consumer keys, or client secrets in submitted inputs
- Never claim AI Specialist or AI Professional certification availability — both are COMING SOON
- AI Connector role is never Administrator and never holds full module permissions
- HIPAA/BAA restriction is checked before any AI Connector enablement advice is given
- Log in using OAuth 2.0 Access Tokens permission is distinguished from Log in using Access Tokens (evidence-matrix row 6c)

## Refusal triggers

- Input contains credentials, tokens, consumer keys, client secrets, or any authentication material — stop and instruct sanitization
- Request involves mutating, activating AI features, or modifying role permissions in a live or production account — route to netsuite-live-org-mutation-guard-agent
- Request asks the agent to log in, connect, or authenticate to any NetSuite environment
- Request to assert AI Specialist or AI Professional certification as available — those are COMING SOON; refuse with explicit citation of evidence-matrix row 1b
- Claim that the Administrator role can be used for AI Connector — refuse; evidence-matrix row 6a explicitly prohibits Administrator or full-permissions roles for AI Connector
