# Workflow and output contract

Use this reference for full Azure Well-Architected Security reviews.

## Workflow

1. **Classify scope**
   - Workload, environment, data sensitivity, critical flows, compliance obligations, owners, and production impact.
   - Identity, network, data, secrets, monitoring, policy, DevSecOps, and incident-response surfaces in scope.

2. **Ground in Microsoft Learn**
   - Use Microsoft Learn documentation through the user's configured documentation MCP for current Well-Architected Security, MCSB, Defender, Policy, Key Vault, Entra, and Azure Monitor guidance.
   - Treat commands and exact feature availability as version-sensitive until verified.

3. **Collect current-state evidence when available**
   - Read-only role assignments, identities, network exposure, Defender recommendations, secure score controls, policy compliance, exemptions, diagnostics, alerting, logs, secrets posture, and vulnerability controls.
   - Do not retrieve secret values or raw customer payloads.

4. **Stress test the posture**
   - Ask what attacker path remains if identity is compromised.
   - Ask what data can be exfiltrated if an app component is compromised.
   - Ask which alerts fire, who owns them, and what runbook executes.
   - Ask what breaks if a deny policy or private endpoint is rolled out too broadly.

5. **Prioritize remediation**
   - Fix broad access, public exposure, secret leakage, missing logs, and unowned alerts before nice-to-have maturity work.
   - Stage disruptive changes and require rollback or safe-deployment plans.

## Output contract

Return:

1. Scoped workload and evidence level
2. Verdict: pass, conditional, or blocked
3. Top security blockers
4. Findings by Security checklist area
5. Safe next actions in priority order
6. Required approvals for any mutation
7. Open questions and assumptions
8. Evidence labels and source notes
