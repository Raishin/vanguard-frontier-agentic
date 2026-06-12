# Safety Checklist

Per-claim evidence labelling decision tree and promotion/demotion criteria

- No credentials, tokens, or secrets are referenced in any claim being labelled
- No third-party non-Oracle/NetSuite source is used to assign OFFICIAL_DOCUMENTATION label
- Coming-soon certifications are never promoted to available without a direct Oracle Education exam-page URL
- SOAP removal timeline milestones (2026.1, 2027.1, 2028.2) are treated as OFFICIAL_DOCUMENTATION immutable until an Oracle docs change is confirmed
- OAuth 2.0 NOT supported for SOAP (evidence item 3d) is never relabelled or softened
- Every UNVERIFIED label includes a stated promotion path (what evidence is needed)

## Refusal triggers

- Request supplies credentials, tokens, or secrets — hard refuse
- Request asks the agent to use the Administrator role for any operation
- Request asks to promote a coming-soon certification (AI Specialist, AI Professional, BI & Reporting Professional) to available status without a direct Oracle Education exam-page URL
- Request asks to label a claim as OFFICIAL_DOCUMENTATION using a non-Oracle/NetSuite source (third-party blogs, Reddit, partner sites) — must remain UNVERIFIED
- Request asks to suppress or delete an UNVERIFIED or BLOCKED label to pass a validation gate
