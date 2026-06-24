# Safety checklist — SAP Live Read-Only Identity and Trust Discovery

Use before every live discovery session and before every command execution.

## Non-negotiables

- Do not execute any command that creates, updates, deletes, assigns, rotates, triggers a provisioning job, or modifies trust configuration — even if the user explicitly requests it.
- Do not request, accept, log, echo, or include in any output: IAS admin credentials, IPS system user passwords, XSUAA client secrets, OAuth access tokens, BTP service keys, SAML signing certificate private keys, or any credential value of any kind.
- Do not proceed with live commands if the user has not confirmed that the credential in scope is read-only (viewer, auditor, or read-only API user role or equivalent).
- Do not expand scope beyond what the user has explicitly authorized — if authorized for one subaccount's role collections, do not enumerate sibling subaccounts or global account trust configurations without explicit authorization.
- Do not cache, store, or transmit live evidence outside the current session context.
- Redact all IAS access tokens, IPS passwords, XSUAA client secrets, and personal user data (email addresses, user names) beyond audit necessity before logging or returning output.
- Do not enumerate user-level personal data (individual user records from IAS or IPS target systems) unless the user has explicitly stated a legitimate audit scope requiring it and has confirmed data protection compliance.

## Forbidden action verification

Before executing any API call or CLI command, verify the HTTP method or CLI verb is on the allowed list:

| Allowed HTTP methods / CLI verbs | Forbidden HTTP methods / CLI verbs |
|----------------------------------|------------------------------------|
| GET, list, get, describe, export, status, show, read, view | POST, PUT, PATCH, DELETE, create, update, delete, set, assign, bind, rotate, regenerate, import, trigger, run, execute, apply, approve, modify-trust |

If the HTTP method or CLI verb is not on the allowed list, refuse and explain which downstream skill or authorized administrator handles that action.

## What people get wrong

- **Using an IAS tenant administrator for "just reading"**: A full IAS tenant administrator can create and modify applications and corporate identity providers. Use a dedicated read-only API user or a service user scoped to GET operations only.
- **Triggering IPS provisioning jobs by accident**: The IPS API uses `POST /Jobs` to trigger a job and `GET /Jobs` to list history. Verify the HTTP method before executing. A `POST` to the Jobs endpoint triggers a live provisioning run.
- **Including IAS access tokens in output**: IAS REST API responses may embed token endpoint metadata. Bearer tokens used in the request must never appear in logs. Replace with `[REDACTED:IAS_TOKEN]` before logging.
- **Enumerating personal user records without a stated audit purpose**: IPS source or target system enumeration may return individual user records with email addresses and personal attributes. Collect only system-level configuration, not individual user records, unless the audit scope explicitly requires it.
- **Assuming `btp list security/trust` cannot cause side effects**: In some BTP CLI versions, list operations may trigger internal audit log entries. This is acceptable; the key constraint is no state mutation.
- **Scope creep across subaccounts**: A subaccount viewer credential may grant visibility across sibling subaccounts in some BTP configurations. Query only the specific subaccount in scope.
- **Treating XSUAA token introspection as read-only for all purposes**: Token introspection exposes live token data. Enumerate role collections and scopes using the authorization API, not by inspecting live user tokens.

## When to push back

- Push back when the user asks to combine a read step with a write step (e.g., list role collections and then assign one to a user).
- Push back when the user asks to trigger an IPS provisioning job "just to test" the connector.
- Push back when credentials provided appear to have write scope (e.g., IAS tenant administrator with application creation rights, IPS administrator with job execution rights).
- Push back when the user asks to enumerate individual user records from IAS or IPS target systems without a stated audit scope and data protection confirmation.
- Push back when the user asks to export SAML metadata including private signing keys.
- Push back when the user asks to enumerate systems or subaccounts outside the agreed scope.

## Evidence labels

- `live evidence` — directly observed from a live SAP IAS, IPS, XSUAA, or BTP system; include command, timestamp, system identifier
- `documentation-based` — grounded in SAP official docs; no live access
- `user-provided evidence` — stated or supplied by the user (configuration exports, screenshots, written descriptions)
- `inference` — derived reasoning; must always be labeled as such
