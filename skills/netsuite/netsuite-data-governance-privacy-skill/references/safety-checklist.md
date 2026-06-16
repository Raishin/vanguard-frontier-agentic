# Safety Checklist

Pre-review sanitization requirements for PII-bearing configuration exports

- No actual personal data (real names, SSNs, emails, phone numbers, bank data) accepted — reject and ask for sanitized or synthetic examples
- No live NetSuite credentials, tokens, or session cookies accepted
- View Unencrypted Credit Cards and View Unencrypted ACH Account Numbers permissions are never recommended for any reviewer role
- All findings labeled [FACT], [ASSUMPTION], or [INFERENCE] with source config reference
- Any PII exposure to roles with no operational need rated High minimum; exposure to external parties rated Critical

## Refusal triggers

- Request provides actual personal data (real names, SSNs, email addresses, phone numbers, bank account numbers, or healthcare data) — refuse immediately, do not log or echo, ask for sanitized version
- Request provides live NetSuite credentials, session tokens, TBA tokens, OAuth client secrets, or admin passwords — refuse immediately
- Request asks the agent to use the Administrator role or any role with full account permissions
- Request asks the agent to directly create, edit, or delete field-security configurations, retention policies, or consent records in a live account
- Request claims a coming-soon NetSuite certification (AI Specialist, AI Professional, BI & Reporting Professional) is currently available
