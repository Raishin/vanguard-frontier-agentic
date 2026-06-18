# Preflight — D365 Live Security Role Guard

Before any live D365 Live Security Role Guard run, confirm all of the following:

## 1. Read-only assertion

- Confirm the agent is running in Phase A (`read-only-runtime`). No POST, PATCH, PUT, or DELETE Dataverse Web API calls will be issued.
- Confirm no mutation is requested in the current task. If mutation is implied, stop and redirect to Phase-B gated process.

## 2. Credential and application user confirmation

- Confirm `DATAVERSE_CLIENT_ID` and `DATAVERSE_ENV_URL` environment variables are set. Do not print or echo their values.
- Confirm the application user exists in the target Dataverse environment (SystemUser row with the correct `ApplicationId`).
- Confirm the application user is bound to the custom read-only security role, not System Administrator or System Customizer.
- Confirm the custom read-only security role grants only Read (prvRead) on the in-scope tables and no Create/Write/Delete/Append/AppendTo privileges.

## 3. SPN path assertion

- Confirm the application user was NOT registered via `pac admin create-service-principal`. If it was, stop — that path grants Power Platform Administrator-level access and is forbidden for this agent.

## 4. Scope confirmation

- Confirm the target environment URL is known and authorized for this discovery run.
- Confirm the requester has authority to review security role posture for the target environment.

## 5. Environment check

- Confirm outbound egress to `*.dynamics.com` and `login.microsoftonline.com` is permitted from the execution environment.
- Confirm no proxy or firewall will intercept and log Dataverse Web API responses containing security role privilege data.

## 6. Approval state

- Confirm that no proposed role design change from a prior run is pending execution without explicit human approval.
- If a prior discovery report exists, confirm its findings have been reviewed before initiating a new run.

## Block conditions

Stop and do not proceed if any of the following are true:

- The application user holds System Administrator or System Customizer role.
- The SPN was registered via the Power Platform management path.
- The credential value has been exposed in any log, chat, or environment dump.
- The target environment is production and the requester cannot confirm authorization.
- A mutation is being requested as part of this Phase-A run.
