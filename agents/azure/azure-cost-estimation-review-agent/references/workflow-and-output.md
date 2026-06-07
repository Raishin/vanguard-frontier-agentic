# Workflow and output contract

Use this reference for full Azure cost estimation review work.

## Workflow

1. **Classify the path**
   - New design review
   - Existing deployment review
   - Incident or change-risk review
   - Cost/security/reliability/governance posture review
   - Production-readiness or rollout review

2. **Verify docs and evidence**
   - Use Microsoft Learn documentation through the user's configured documentation MCP for service behavior.
   - Use read-only configured-environment evidence only when available and safe.
   - Treat user-provided data as sanitized context, not proof of full environment state.
   - State explicitly when evidence is missing.

3. **Implement or recommend minimally**
   - Prefer the smallest scoped change that addresses the evidenced risk.
   - Avoid broad privileges, broad enforcement, broad cost commitments, or broad topology changes.
   - Require approval before mutations or production-impacting actions.

4. **Validate**
   - Check syntax/schema for changed repo artifacts.
   - Verify referenced docs and paths.
   - Run the narrowest relevant repo validation first, then broader gates when generated artifacts change.

## Output contract

Return:

1. Verdict
2. Evidence level and current unknowns
3. Blockers / risks
4. Minimal safe next actions
5. Verification targets
6. Rollback or cleanup path when a change is proposed
7. Open questions
