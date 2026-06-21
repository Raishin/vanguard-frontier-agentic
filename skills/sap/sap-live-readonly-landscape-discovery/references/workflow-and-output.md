# Workflow and output contract — SAP Live Read-Only Landscape Discovery

Use this reference for all discovery workflows, command selection, and output formatting.

## Allowed command patterns

### BTP CLI (read-only)

```bash
# List global account entitlements
btp list accounts/entitlement --global-account <ID>

# List subaccounts in a directory
btp list accounts/subaccount --directory <ID>

# Get subaccount details
btp get accounts/subaccount <SUBACCOUNT_ID>

# List service instances in a subaccount
btp list services/instance --subaccount <SUBACCOUNT_ID>

# List available service plans
btp list services/plan --subaccount <SUBACCOUNT_ID>

# List destinations in a subaccount
btp list connectivity/destination --subaccount <SUBACCOUNT_ID>

# Get a specific destination
btp get connectivity/destination <DESTINATION_NAME> --subaccount <SUBACCOUNT_ID>
```

### CF CLI (read-only)

```bash
# List orgs
cf orgs

# List spaces in org
cf spaces

# List apps (no modification)
cf apps

# List service instances
cf service-instances

# Get environment (redact credentials before logging)
cf env <APP_NAME>

# List routes
cf routes
```

### kubectl (read-only, Kyma)

```bash
# List namespaces
kubectl get namespaces

# List service instances (SAP BTP Operator)
kubectl get serviceinstances -n <NAMESPACE>

# Describe a service instance
kubectl describe serviceinstance <NAME> -n <NAMESPACE>

# List API rules (read-only)
kubectl get apirules -n <NAMESPACE>
```

## Discovery workflow

1. **Confirm authorization gate**
   - Confirm with the user that live read-only access is authorized for this session.
   - Confirm credential scope is viewer/auditor role only.
   - Confirm target system and scope (global account, subaccount, space, ABAP system).

2. **Select discovery scope**
   - Identify which landscape layer(s) are in scope: BTP account model / CF runtime / Kyma / ABAP system landscape.
   - Select the minimum command set needed.

3. **Execute read-only commands**
   - Run only allowed command patterns above.
   - Log every command with timestamp.
   - Redact credential values from CF env output before logging.

4. **Structure evidence**
   - Organize output by resource type.
   - Label all data as `live evidence` with command and timestamp.

5. **Return output** per the output contract below.

## Output contract

Return:

1. Discovery scope (what was enumerated and why)
2. Authorization gate confirmation (role confirmed read-only: yes/no)
3. Command log (every command, timestamp, summary of output)
4. Structured evidence (resources enumerated, organized by type)
5. Redaction log (what was redacted and why)
6. Next step (how this evidence feeds into the downstream advisory: clean core review, transport planning, etc.)
7. Escalation trigger (any forbidden action encountered or write attempt by the tool)
