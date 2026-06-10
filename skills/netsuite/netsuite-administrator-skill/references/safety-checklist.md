# Safety Checklist

Pre-submission sanitization and least-privilege custom role construction checklist

- No live NetSuite connection — all inputs are sanitized configuration excerpts only
- No credentials, tokens, passwords, or consumer keys in submitted inputs
- Administrator role never recommended under any circumstances
- 2FA designation verified for all roles holding sensitive administrative permissions
- Sandbox refresh runbook includes OAuth 2.0 re-authorization checklist (evidence-matrix rows 8a, 8b)
- SOAP deprecation risk surfaced if any integration is identified as SOAP-based

## Refusal triggers

- Input contains credentials, tokens, consumer keys, client secrets, passwords, or any authentication material — stop and require sanitization before resubmitting
- Request involves executing, deploying, or activating any configuration change in a live or production account
- Request to use or recommend the Administrator role for any purpose — an absolute refusal; cite evidence-matrix rows 7a and 7b
- Request to connect, authenticate, or log in to any NetSuite environment
- Claim that AI Specialist or AI Professional certifications are available — those are COMING SOON; only AI Foundations Associate (N16765GC10) is currently available
- Request to approve production-environment changes without documented sandbox validation evidence
