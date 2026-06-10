# Safety Checklist

Pre-review checklist for AI Connector role, permissions, features, and HIPAA/BAA status

- No live NetSuite credentials, tokens, or session cookies accepted
- AI Connector role must never be Administrator or full-permission role — Critical if present
- Exact permission names must match verbatim: 'MCP Server Connection' and 'Log in using OAuth 2.0 Access Tokens'
- HIPAA/BAA healthcare accounts must not activate AI Connector — Critical if attempted
- Tool allowlist must be explicit — implicit full access is a High finding
- All findings labeled [FACT], [ASSUMPTION], or [INFERENCE] with source config reference

## Refusal triggers

- Request provides live NetSuite credentials, session tokens, TBA tokens, OAuth client secrets, or admin passwords — refuse immediately, do not log or echo
- Request asks the agent to use the Administrator role or any role with full permissions to access NetSuite features for AI Connector configuration (evidence row 6a)
- Request asks the agent to directly activate, modify, or disable the AI Connector Service in a live account
- Request uses 'Log in using Access Tokens' instead of 'Log in using OAuth 2.0 Access Tokens' and asserts they are equivalent — they are NOT equivalent (evidence row 6c); flag and correct
- Request claims AI Specialist or AI Professional certifications are currently available — they are COMING SOON only (evidence rows 1b, AI track)
- Request attempts to configure the AI Connector for a healthcare account with a signed BAA — blocked by Oracle policy (evidence row 6e)
