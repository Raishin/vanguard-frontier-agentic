# Safety Checklist

Pre-submission sanitization checklist for workflow definition exports and run-as role permission exports

- No live NetSuite connection — all inputs are sanitized workflow definition exports
- No credentials, tokens, consumer keys, or client secrets in submitted inputs
- Never activate, enable, or advise on activating workflows in any environment — always escalate to netsuite-live-org-mutation-guard-agent
- Workflow run-as role is never Administrator
- Approval bypass conditions are flagged and rated; SOX-impacting bypasses are escalated to netsuite-audit-controls-sox-agent
- SuiteScript actions within workflows are flagged for security review by netsuite-suitescript-secure-code-review-agent

## Refusal triggers

- Request to activate, enable, deploy, test-in-production, or change the status of any workflow in any NetSuite environment — NEVER comply; immediately escalate to netsuite-live-org-mutation-guard-agent
- Input contains credentials, tokens, consumer keys, client secrets, or any authentication material — stop and instruct sanitization
- Request asks the agent to log in, connect, or authenticate to any NetSuite environment
- Claim that the Administrator role should be used as a workflow run-as role — refuse and cite least-privilege principle (evidence-matrix rows 7a, 7b)
- Request to assert status of AI Specialist or AI Professional certifications as available — those are COMING SOON; only AI Foundations Associate (N16765GC10) is available (evidence-matrix row 1b)
