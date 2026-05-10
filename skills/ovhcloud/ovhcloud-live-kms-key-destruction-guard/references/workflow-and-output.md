# Workflow and output contract

Use this reference when processing an OVHcloud KMS key version destruction request. All five mandatory gates must be verified before producing a destruction plan. Do not skip or defer any gate.

## Gate evaluation sequence

Evaluate every gate in order. Stop immediately at the first gate that fails or is ambiguous.

### Gate 1 — Target key identity
- Exact key version ID as returned by the OVHcloud KMS API
- KMS service URN of the containing service (`urn:v1:<region>:resource:okms:...`)
- Confirmation that this is the specific version to destroy, not the entire key or a different version

### Gate 2 — Named approving identity
- Full name or unique identifier of the person approving destruction (not a role, alias, or team name)
- Confirmation that the approver has authority over this key and has been informed of the irreversible nature of the action
- No vague intent ("just delete it", "my team said so") qualifies as a gate pass

### Gate 3 — Usage audit
- Evidence that the key version has zero active references within the retention window
- This means: no service, application, or stored ciphertext is currently depending on this key version for decryption
- Acceptable evidence: audit log output, application-layer confirmation, or encryption inventory check

### Gate 4 — Waiting period
- Documented waiting period as required by OVHcloud KMS policy or the organization's key retirement standard
- The waiting period must be explicitly stated and accepted, not assumed

### Gate 5 — Rollback or data recovery plan
- Documented plan for any data that was encrypted under this key version
- If data recovery is impossible after destruction, that must be explicitly acknowledged by the approving identity

## Safe workflow

1. **Announce gate evaluation** — state that all five gates will be checked before a destruction plan is produced.
2. **Collect gate evidence** — request each gate's evidence explicitly if it was not provided in the initial request.
3. **Evaluate each gate** — label each gate as PASS, FAIL, or AMBIGUOUS with the evidence cited.
4. **Hard-stop on any failure** — if any gate fails or is ambiguous, refuse the destruction plan and state exactly which gate failed and what evidence is required to clear it.
5. **Produce the destruction plan only when all five gates pass** — present the plan for human review; do not execute automatically.

## Output contract

Return this structure:

```markdown
# OVHcloud KMS Key Destruction Gate Review: <key ID or label>
## Gate verdict
| Gate | Status | Evidence |
|---|---|---|
| 1. Key ID and KMS service URN | PASS / FAIL / AMBIGUOUS | <evidence cited> |
| 2. Named approving identity | PASS / FAIL / AMBIGUOUS | <evidence cited> |
| 3. Usage audit (zero active references) | PASS / FAIL / AMBIGUOUS | <evidence cited> |
| 4. Waiting period documented | PASS / FAIL / AMBIGUOUS | <evidence cited> |
| 5. Rollback or data recovery plan | PASS / FAIL / AMBIGUOUS | <evidence cited> |
## Overall result
- PROCEED / HARD STOP
## Hard-stop reason (if applicable)
- Gate(s) failed: <list>
- Required to proceed: <what evidence or action is needed>
## Destruction plan (only when all gates pass)
- Target key version ID:
- KMS service URN:
- Approving identity:
- Waiting period:
- Data recovery posture:
- Destruction command or Terraform action (for review, not auto-execution):
- Post-destruction verification steps:
## Residual risk
- <explicit acknowledgment that destruction is irreversible, or explicit none>
```
