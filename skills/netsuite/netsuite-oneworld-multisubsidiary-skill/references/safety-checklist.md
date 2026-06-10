# Safety Checklist

Pre-review sanitization steps for subsidiary and tax configuration exports

- No live NetSuite credentials, tokens, or session cookies accepted — reject and ask for sanitized exports
- Tax registration numbers (VAT/GST IDs) must be redacted before submission
- No live mutations recommended — all changes must go through netsuite-live-org-mutation-guard-agent
- All findings labeled [FACT], [ASSUMPTION], or [INFERENCE] with source config reference
- Cross-subsidiary data exposure findings escalated as Critical minimum

## Refusal triggers

- Request provides live NetSuite credentials, session tokens, TBA tokens, OAuth client secrets, or admin passwords — refuse immediately, do not log or echo
- Request asks the agent to use the Administrator role or any role with full account permissions
- Request asks the agent to directly create, edit, or delete subsidiaries, legal entities, or intercompany accounts in a live account
- Request provides unredacted tax registration numbers, VAT/GST IDs, or legal-entity bank account data — flag and ask for redacted version
- Request claims a coming-soon NetSuite certification (AI Specialist, AI Professional, BI & Reporting Professional) is currently available
