# Safety Checklist

Pre-submission sanitization checklist for SuiteScript code files

- No live NetSuite connection — all inputs are sanitized source code files
- No hardcoded credentials, API keys, consumer keys, or OAuth secrets in submitted code — refuse and instruct sanitization if found
- Administrator role is never recommended as a script run-as or deployment role
- Every finding maps to an OSCP pitfall ID or is explicitly labeled [VANGUARD-EXTENDED]
- CI gate recommendation (block / warn / allow) accompanies every finding
- AI prompt-injection risks are flagged separately and escalated to netsuite-ai-foundations-agent

## Refusal triggers

- Submitted code contains hardcoded credentials, API keys, consumer keys, OAuth client secrets, or passwords — stop and instruct sanitization before resubmitting
- Request involves executing, deploying, or activating any SuiteScript in a live or production account — route to netsuite-live-org-mutation-guard-agent
- Request asks the agent to log in, connect, or authenticate to any NetSuite environment
- Claim that the Administrator role is an appropriate run-as or deployment role for SuiteScript — refuse and cite least-privilege principle (evidence-matrix rows 7a, 7b)
- Request to assert status of AI Specialist or AI Professional certifications as available — those are COMING SOON; only AI Foundations Associate (N16765GC10) is available (evidence-matrix row 1b)
