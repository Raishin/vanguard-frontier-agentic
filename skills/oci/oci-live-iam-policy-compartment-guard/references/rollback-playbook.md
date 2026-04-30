# Rollback Playbook: OCI Live IAM Policy Compartment Guard

## Revert to a previous policy version

OCI does not natively version policies, but you can restore from the prior statements.

```bash
# Get current policy statements
oci iam policy get --policy-id <POLICY_OCID> --query "data.statements"

# Update the policy with the previous statements
oci iam policy update \
  --policy-id <POLICY_OCID> \
  --statements '["Allow group <previous-group> to <verb> <resource> in <scope>", ...]' \
  --version-date $(date +%Y-%m-%d) \
  --force
```

## Delete an accidentally created policy

> ⚠️ **IRREVERSIBILITY WARNING — IAM blast radius**
>
> Deleting an IAM policy is **immediate** (eventual consistency: 10–30 seconds globally) and
> may revoke access to running production workloads before any operator can react.
> The `--force` flag below suppresses the OCI CLI's interactive confirmation prompt.
>
> **Required pre-delete confirmation steps** — do not skip:
>
> 1. Run `oci iam policy get --policy-id <POLICY_OCID>` and inspect the statements.
> 2. Confirm the displayed `name` and `compartment-id` match the policy you intend to delete.
> 3. Confirm in writing (chat, ticket, change record) that the policy is not in active use:
>    `oci search resource structured-search --query-text "query policy resources where compartmentId = '<compartment>'"`
> 4. If unsure, prefer `oci iam policy update` to empty the `statements` array first
>    (reversible) before issuing the `delete` command.
>
> Only after all four steps are complete should the `delete --force` command be executed.

```bash
# Step 1: Confirm the target policy
oci iam policy get --policy-id <POLICY_OCID> --query "data.{name:name,compartment:\"compartment-id\",statements:statements}"

# Step 2: Only after operator confirmation — delete
oci iam policy delete --policy-id <POLICY_OCID> --force
```

## Remove a group member added by mistake (privilege de-escalation)

```bash
# Find the user's group membership
oci iam group list-users --group-id <GROUP_OCID> \
  --query "data[?name=='<USERNAME>'].id"

# Remove from group
oci iam group remove-user --group-id <GROUP_OCID> --user-id <USER_OCID>
```

## Rollback limitations

- OCI IAM has eventual consistency — policy changes may take up to 10–30 seconds to propagate globally.
- There is no automated version history for policies — maintain external backups of policy statements.
- Removing a policy statement may immediately break running workloads that depend on that grant.
- Break-glass tenancy-root admin changes require emptying the `<iam-tenancy-admins>` group immediately after use.
