# Workflow and output contract

Use this reference only when executing a confirmed live server creation, deletion, type change, or power operation. All hard-stop pre-flight checks must be confirmed before proceeding.

## Operation domains

Confirm which operation type is in scope before proceeding:

- **Server creation** — provisioning a new server (confirm public IP intent, server type, region, and SSH key)
- **Server deletion** — permanently destroying a server (irreversible without a prior snapshot)
- **Type change (rescale)** — upgrading or downgrading server type (requires server stopped; causes downtime)
- **Power operation** — reboot, reset, shutdown, or power-off (lower risk but confirm target server first)
- **Snapshot creation** — creating a server image snapshot before a destructive operation

## Hard-stop pre-flight workflow

All seven checks are required before any mutation proceeds. Stop at the first blocking condition.

1. **Confirm server identity**
   - Execute: `GET /v1/servers/{id}`
   - Verify server ID, name, type, region (datacenter), and current power state
   - Confirm project context and API token scope

2. **Confirm region and project**
   - Confirm datacenter location: fsn1 (Falkenstein DE), nbg1 (Nuremberg DE), or hel1 (Helsinki FI)
   - Confirm the operation is in the intended Hetzner project

3. **Confirm operation and target**
   - State the operation explicitly: create / destroy / resize to `<type>` / power-off / reboot
   - For creation: confirm `public_net.ipv4.create` and `public_net.ipv6.create` intent
   - For type change: confirm the new server type is available in the target region

4. **Snapshot evidence (required for deletion and type change)**
   - For deletion: verify an existing snapshot ID or create one: `POST /v1/servers/{id}/actions/create_image` with `type: snapshot`
   - For type change: recommend a pre-resize snapshot as recovery evidence
   - Confirm snapshot ID before proceeding

5. **Downtime window (required for type change)**
   - Confirm server is stopped or will be stopped before the type change
   - Confirm the downtime window is approved by the workload owner
   - For running servers: confirm stop sequence before `change_type`

6. **Confirm rollback plan**
   - For deletion: rollback = restore from confirmed snapshot ID
   - For type change: rollback = change_type back to original type (requires stop again)
   - For creation: rollback = delete the newly created server
   - Document who owns rollback execution and the rollback time window

7. **Receive explicit human approval**
   - Named approving identity confirms this specific server ID and operation
   - Show the exact API call that will be executed before sending it

## Safe execution workflow

1. **Frame scope** — confirm operation type, server ID, and project context
2. **Collect pre-flight evidence** — GET server state, confirm snapshot, confirm token scope
3. **Show the exact call** — display the full API request before executing
4. **Execute and verify** — run the operation, then GET server state to confirm new status
5. **Post-operation validation** — confirm server health, connectivity, and workload status

## Output contract

Return this structure:

```markdown
# Hetzner Server Lifecycle: <server ID / name> — <operation>
## Pre-flight status
- Server identity confirmed: ID <id>, name <name>, type <type>, region <datacenter> / not confirmed (blocking)
- Snapshot evidence: snapshot ID <id> confirmed / not present (blocking for deletion)
- Downtime window approved: yes / not applicable / not confirmed (blocking for type change)
- Rollback plan confirmed: <restore from snapshot <id> / delete server / revert type> / not confirmed (blocking)
- Human approval: received from <identity> / not received (blocking)
## Operation
- Type: <create / delete / resize to <type> / power-off / reboot>
- Exact API call:
  ```
  <method> <endpoint>
  <request body>
  ```
## Rollback procedure
- Method: <restore snapshot / delete server / revert type>
- Snapshot ID (if applicable): <id>
- Owner: <who executes rollback>
- Window: <rollback time window>
## Post-operation verification
- Check: <GET /v1/servers/{id} — expected state>
- Workload health check: <if applicable>
```
