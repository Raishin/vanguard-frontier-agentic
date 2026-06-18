# Preflight — M365 Live Identity Posture Guard

Before any live M365 Live Identity Posture Guard run, confirm all of the following:

## 1. Read-only assertion

- Confirm the agent is running in Phase A (`read-only-runtime`). No write, patch, post, or delete Graph API calls will be issued.
- Confirm no mutation is requested in the current task. If mutation is implied, stop and redirect to Phase-B gated process.

## 2. Scope and credential confirmation

- Confirm `GRAPH_CLIENT_ID` and `GRAPH_TENANT_ID` environment variables are set. Do not print or echo their values.
- Confirm the app registration holds only the five read-only application permissions listed in `PERMISSIONS.md` and that admin consent has been granted.
- Confirm no `*.Write` or `*.ReadWrite.*` permissions appear on the app registration.
- Confirm the credential type is certificate or managed identity (preferred) or a short-lived secret.

## 3. Scope confirmation

- Confirm the target tenant ID is known and authorized for this discovery run.
- Confirm the requester has authority to review identity posture for the target tenant.

## 4. Environment check

- Confirm outbound egress to `graph.microsoft.com` and `login.microsoftonline.com` is permitted from the execution environment.
- Confirm no proxy or firewall will intercept and log Graph API responses containing sensitive directory data.

## 5. Approval state

- Confirm that no proposed hardening change from a prior run is pending execution without explicit human approval.
- If a prior discovery report exists, confirm its findings have been reviewed before initiating a new run.

## Block conditions

Stop and do not proceed if any of the following are true:

- A write-capable permission exists on the app registration.
- The credential value has been exposed in any log, chat, or environment dump.
- The target tenant is production and the requester cannot confirm authorization.
- A mutation is being requested as part of this Phase-A run.
