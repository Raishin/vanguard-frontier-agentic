# Safety checklist

Use this reference before any recommendation that touches production DLP policy mutation, environment deletion, Dataverse role bulk-assignment, or connector reclassification affecting a production environment.

## Non-negotiables

- Never ask for or accept tenant IDs, environment IDs, connection strings, service principal secrets, user passwords, or customer PII.
- Never recommend disabling or broadly relaxing DLP policy to unblock delivery without documented business justification, blast-radius assessment, and explicit written approval.
- Production DLP changes are live-guard gated: always require explicit human confirmation, a blast-radius scope (which apps/flows would break), and a tested rollback path before applying any policy change.
- Do not invent environment names, connector classifications, security role privileges, or live configuration state.
- Require explicit user approval before recommending mutations that change connector availability, security role assignment, business unit membership, or environment permissions at production scope.
- Use current official Microsoft Learn documentation for service behavior when the answer depends on Power Platform or Dataverse service details.
- Keep all remediations least-privilege, reversible, and scoped to the requested environment or workload boundary.

## Stress checks

- What connectors, if reclassified, would allow data to flow to uncontrolled external endpoints?
- What Dataverse privilege change could expose records across business unit boundaries unintentionally?
- What environment or DLP change could break existing production apps or flows?
- What compliance or audit evidence is missing to support the recommendation?
- What rollback or validation path is unproven for the recommended change?
- Is the request to relax a control actually a symptom of missing environment strategy or CoE process?

## Evidence labels

Use `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's live tenant DLP state, environment inventory, or Dataverse role configuration.

## Live-guard gate reminder

DLP policy mutations on production environments must not be auto-applied. The required sequence is:
1. Identify blast radius (which apps/flows in scope environment use affected connectors).
2. Communicate change to affected makers.
3. Obtain explicit written approval from the Power Platform admin or tenant admin.
4. Apply change in a test or sandbox environment first.
5. Validate no regressions before production application.
6. Document rollback procedure (policy export before change, restore path if issues arise).
