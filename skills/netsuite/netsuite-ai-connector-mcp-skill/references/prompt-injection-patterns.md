# Prompt Injection Patterns

Reference patterns for prompt-injection testing and SafeWords mitigations in NetSuite AI Connector sessions

Scope: Audits the NetSuite AI Connector Service setup for correct role/permission configuration (NOT Administrator; exactly 'MCP Server Connection' and 'Log in using OAuth 2.0 Access Tokens'), explicit tool allowlists, HIPAA/BAA restriction compliance, and prompt-injection safeguards. Combines Vanguard harness governance (routing, logging, retry) with Oracle upstream AI connector guardrails.

- AI Connector role review: confirming the connecting role is NOT the Administrator role and does not have full permissions to access all NetSuite features (evidence row 6a)
- Required permission verification: exactly 'MCP Server Connection' (evidence row 6b) and 'Log in using OAuth 2.0 Access Tokens' (evidence row 6c) — neither more nor less
- Required feature verification: Server SuiteScript enabled, OAuth 2.0 enabled, REST Web Services enabled if using MCP Standard Tools SuiteApp (evidence row 6d)
- Tool allowlist review: assessment of whether explicit tool allowlists are defined and scoped to the minimum set of NetSuite operations needed by the AI session
- Prompt-injection testing coverage: review of whether prompt-injection test cases exist for the AI Connector integration and whether SafeWords principles are applied
- HIPAA/BAA restriction check: flagging any healthcare account with a signed BAA attempting to activate the AI Connector (evidence row 6e)
- Harness governance: Vanguard-specific tool-call logging, retry logic, and escalation routing for AI Connector sessions
