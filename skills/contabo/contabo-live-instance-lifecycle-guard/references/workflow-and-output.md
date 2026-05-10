# Workflow and output contract

Use this reference when executing a VPS or VDS lifecycle operation: create, reinstall, or cancel. Every step in this sequence is mandatory before issuing any mutation call.

## Pre-mutation sequence

1. **Confirm target identity and environment**
   - For create: confirm product ID, region, contract period, image, Cloud-Init userData (if any), and SSH key secret IDs.
   - For reinstall: confirm instance ID, new image, Cloud-Init userData (if any), and SSH key secret IDs. Warn that reinstall wipes the current OS and data.
   - For cancel: confirm instance ID, remaining contract period, and early-termination billing impact.
   - Verify OAuth2 token freshness. Tokens expire in ~5 minutes. Refresh immediately before the mutation call.

2. **Run read-only inventory first**
   - List current instances via `GET /v1/compute/instances` before any mutation.
   - Confirm the target instance ID and its current state. Do not proceed if the inventory call fails or returns unexpected state.
   - For create: confirm product availability in the target region via the API or official docs.

3. **Enforce hard-stop gates**
   - All four gates (target, contract period + billing acknowledgment, rollback plan, named approving identity) must be confirmed in writing before proceeding.
   - If any gate is missing, stop and request the specific missing item. Do not infer or assume.

4. **Review Cloud-Init userData if present**
   - Scan for embedded secrets, hardcoded credentials, curl-pipe-sh patterns, and commands that disable audit logging.
   - Refuse to include userData that fails this review. Request a remediated version.

5. **Execute with traceability**
   - Include a fresh UUIDv4 `x-request-id` in the mutation call header.
   - Log the request ID for support traceability. Do not log the OAuth2 token value.
   - Report the API response and confirm instance state after the operation.

6. **Post-mutation verification**
   - Confirm the instance reached the expected state (running, reinstalling, or cancelled).
   - Verify SSH access or console availability after create/reinstall.
   - Record the new contract period end date for cancellations and renewals.

## Output contract

Return this structure:

```markdown
# Contabo Instance Lifecycle: <operation> — <instance ID or product>
## Hard-stop gate status
- [ ] Target confirmed: <instance ID / product ID + region>
- [ ] Contract period acknowledged: <1 | 3 | 6 | 12> months — billing impact: <amount/period>
- [ ] Rollback plan documented: <recovery path>
- [ ] Named approving identity: <full name or authenticated account identifier>
- [ ] OAuth2 token freshness: confirmed fresh (refreshed at <time>)
## Pre-mutation inventory
- Current instance state: <state or "not yet queried">
- Product availability in region: <confirmed | documentation-based | not checked>
## Cloud-Init review (if applicable)
- userData review result: PASS / FAIL / NOT APPLICABLE
- Issues found: <issues or "none">
## Proposed action
- Operation: <create | reinstall | cancel>
- API call: <sanitized call with x-request-id placeholder>
## Post-mutation verification
- Instance state after operation:
- SSH / console access confirmed:
- Contract period end date:
## Open risks or refusal reason
- <risk or explicit none>
```
