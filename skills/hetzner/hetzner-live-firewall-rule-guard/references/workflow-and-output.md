# Workflow and output contract

Use this reference only when executing a confirmed live Firewall rule mutation, attachment change, or Firewall creation or deletion. All hard-stop pre-flight checks must be confirmed before proceeding.

## Mutation domains

Confirm which operation type is in scope before proceeding:

- **Inbound rule mutation** — adding, updating, or deleting inbound rules (protocol, port, source CIDR)
- **Outbound rule mutation** — adding, updating, or deleting outbound rules (protocol, port, destination CIDR)
- **Firewall attachment change** — attaching or detaching a Firewall from servers or Label groups
- **Firewall creation** — creating a new Firewall with initial rule set
- **Firewall deletion** — deleting a Firewall (verify no servers depend on it as their only protection)

## Hard-stop pre-flight workflow

All five checks are required before any mutation proceeds. Stop at the first blocking condition.

1. **Snapshot current Firewall state**
   - Execute: `GET /v1/firewalls/{id}`
   - Store the full rule set as rollback evidence
   - Confirm Firewall ID and name match the intended target

2. **Blast-radius review**
   - Execute: `GET /v1/firewalls/{id}` — inspect the `applied_to` field
   - List all attached servers (by ID and name) and Label groups
   - Confirm the operator understands every server that will be affected

3. **Confirm target and project context**
   - Confirm Firewall ID, Firewall name, and Hetzner project
   - Confirm region context (fsn1 / nbg1 / hel1) if servers are region-specific
   - Verify API token is project-scoped and has write access

4. **Confirm rollback plan**
   - Document the exact rule revert procedure (reapply snapshotted rules via `POST /v1/firewalls/{id}/actions/set_rules`)
   - Or confirm Firewall detach path if rollback requires detachment
   - Confirm who owns rollback execution and the rollback time window

5. **Receive explicit human approval**
   - Named approving identity confirms this specific Firewall ID and rule change
   - Show the exact API call that will be executed before sending it

## Safe execution workflow

1. **Frame scope** — confirm operation type, Firewall ID, and project context
2. **Collect pre-flight evidence** — snapshot rules, review blast-radius, confirm token scope
3. **Show the exact call** — display the full API request before executing
4. **Execute and verify** — run the mutation, then call `GET /v1/firewalls/{id}` to confirm new state
5. **Post-change validation** — verify attachment state, check affected servers for connectivity

## Output contract

Return this structure:

```markdown
# Hetzner Firewall Mutation: <Firewall ID / name>
## Pre-flight status
- Snapshot captured: yes / no (blocking)
- Blast-radius confirmed: <server count and names> / not confirmed (blocking)
- Target confirmed: Firewall ID <id>, project <project> / not confirmed (blocking)
- Rollback plan confirmed: <revert procedure> / not confirmed (blocking)
- Human approval: received from <identity> / not received (blocking)
## Operation
- Type: <inbound rule add / outbound rule delete / attachment change / etc.>
- Exact API call:
  ```
  <method> <endpoint>
  <request body>
  ```
## Blast-radius summary
- Servers affected: <list>
- Label groups affected: <list>
## Rollback procedure
- Command: <exact revert API call>
- Owner: <who executes rollback>
- Window: <rollback time window>
## Post-change verification
- Check: <GET /v1/firewalls/{id} — expected state>
- Connectivity test: <if applicable>
```
