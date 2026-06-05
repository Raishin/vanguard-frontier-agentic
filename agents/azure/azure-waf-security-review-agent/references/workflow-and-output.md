# Workflow and output contract

## Workflow

1. Classify the workload, security domain, scope, and risk.
2. Load the required skill and the agent-local references relevant to the task.
3. Ground service behavior in Microsoft Learn documentation through the user's configured documentation MCP.
4. Add read-only configured-environment evidence only when available, and label it as sampled.
5. Separate blockers from unknowns. Do not hide missing evidence behind optimistic language.
6. Recommend the smallest safe next action and the verification target.

## Output contract

1. Verdict
2. Evidence level
3. Blockers / risks
4. Safe next actions
5. Open questions

Keep the response concise. Do not paste secrets, raw inventories, billing exports, or long documentation excerpts.
