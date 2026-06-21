# Safety checklist — SAP Live Read-Only Landscape Discovery

Use before every live discovery session and before every command execution.

## Non-negotiables

- Do not execute any command that creates, updates, deletes, deploys, assigns, rotates, imports, or triggers a state change — even if the user explicitly requests it.
- Do not request, accept, log, echo, or include in any output: SAP system credentials, BTP service keys, OAuth client secrets, API tokens, ABAP logon passwords, or RFC user passwords.
- Do not proceed with live commands if the user has not confirmed that the credential in scope is read-only (viewer/auditor role or equivalent).
- Do not expand scope beyond what the user has explicitly authorized — if authorized for one subaccount, do not enumerate sibling subaccounts.
- Do not cache, store, or transmit live evidence outside the current session context.
- Redact all credential values from CF env output, service binding details, and destination configuration before logging.

## Forbidden action verification

Before executing any CLI command, verify the verb is on the allowed list:

| Allowed verbs | Forbidden verbs |
|--------------|----------------|
| list, get, describe, export, status, show, read, view | create, update, delete, set, push, deploy, assign, bind, entitle, rotate, regenerate, import, trigger, run, execute, apply, patch, approve |

If the command verb is not on the allowed list, refuse and explain which downstream skill handles that action.

## What people get wrong

- **Using CF SpaceDeveloper for "just reading"**: SpaceDeveloper has write access. Always use SpaceAuditor or OrgAuditor for discovery.
- **Including service key values in output**: `cf env` output includes VCAP_SERVICES with bound service credentials. Redact all credential blocks before including in output.
- **Treating `cf restage` or `cf restart` as read-only**: These are state-transition commands. Forbidden.
- **Assuming `btp list` can never cause side effects**: Some CLI versions may trigger audit log entries. This is acceptable; the key constraint is no state mutation.
- **Scope creep on ABAP RFC enumeration**: A display RFC user may have access to more tables than needed. Query only the specific landscape objects in scope.

## When to push back

- Push back when the user asks to combine a read step with a write step in a single command or sequence.
- Push back when the user asks to "quickly create" a test destination or service instance "to check something."
- Push back when credentials provided appear to have write scope (e.g., SpaceDeveloper, subaccount administrator).
- Push back when the user asks to enumerate systems outside the agreed scope.
- Push back when the user asks to export data to an external system or share it outside the session.

## Evidence labels

- `live evidence` — directly observed from a live SAP system; include command, timestamp, system identifier
- `documentation-based` — grounded in SAP official docs; no live access
- `user-provided evidence` — stated or supplied by the user
- `inference` — derived reasoning; must always be labeled as such
