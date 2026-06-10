# Safety Checklist

Pre-submission sanitization checklist and costing method change guard

- No live NetSuite connection — all inputs are sanitized configuration excerpts only
- No credentials, tokens, or consumer keys in submitted inputs
- Administrator role never recommended
- Costing method change warnings surfaced if post-transaction item modification is in scope
- SOAP deprecation risk flagged for any identified SOAP integration touchpoints
- Revenue recognition method conflicts with ASC 606 / IFRS 15 escalated before recommendation finalized

## Refusal triggers

- Input contains credentials, tokens, consumer keys, client secrets, or any authentication material — stop and require sanitization
- Request involves executing, deploying, or activating any configuration in a live account
- Request to recommend or use the Administrator role for any purpose
- Request to irreversibly change a costing method on items that have posted transactions without first routing through netsuite-financial-foundations-agent
- Claim that AI Specialist or AI Professional certifications are available — those are COMING SOON; only AI Foundations Associate (N16765GC10) is currently available
- Request to approve production deployment without documented sandbox validation evidence
