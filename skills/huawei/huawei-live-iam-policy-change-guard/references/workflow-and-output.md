# Workflow and Output — Huawei Live IAM Policy Change Guard

## Step-by-Step Workflow

### Phase 1: Identity and Scope Confirmation

1. Confirm account ID and active IAM principal.
2. Confirm master vs. member account — SCP mutations require master account authority.
3. Confirm enterprise project scope: account-wide or scoped to specific enterprise project(s).

### Phase 2: Current Policy Inventory

4. List all policies attached to the target user/group/agency:
   - IAM console > Users/User Groups > select target > Permissions tab
5. List active SCPs at org level (for member account changes):
   - Organizations console > Policies > Service Control Policies
6. List agency trust relationships if cross-account access is involved:
   - IAM console > Agencies > select agency > Trust Relationships tab

### Phase 3: Blast-Radius Assessment

7. For system policy grants (FullAccess): enumerate all services affected — a service FullAccess policy grants complete control of that service.
8. For SCP changes: enumerate all member accounts and enterprise projects affected.
9. For agency grants: enumerate all trusted principals that will gain the delegated access.
10. For fine-grained policy changes: list all actions added or removed and the resource scope.

### Phase 4: Privilege Escalation Check

11. Confirm the proposed change does not grant the ability to:
    - Create or attach FullAccess policies to other principals
    - Create agencies with SecurityAdministrator or IAMFullAccess
    - Modify or delete existing SCPs
    - Bypass MFA requirements
12. If any privilege escalation path is identified, halt and report.

### Phase 5: Approval Gate

13. Present all evidence to the operator: current policies, proposed change, blast-radius, escalation check.
14. For SCP mutations: require org-admin confirmation and evidence of non-production testing.
15. Require explicit written approval.
16. Do not proceed until approval is received.

### Phase 6: Execution

17. Execute via IAM console or API:
    - Attach policy: IAM console > Users/User Groups > Permissions > Grant Permissions > select policy
    - Create custom policy: IAM console > Permissions > Custom Policies > Create Custom Policy
    - Modify agency: IAM console > Agencies > select agency > Modify
    - Apply SCP: Organizations console > Policies > SCP > Attach to org unit or account

### Phase 7: Post-Change Verification

18. Re-list policies for the target principal — confirm the intended change is applied.
19. Verify a dependent service or resource access works as expected.
20. Query CTS for the IAM event to confirm audit trail.
21. For SCP changes, verify member account behavior for a sample of previously allowed operations.

## Expected Output Format

The agent response for an IAM policy change must include:

```
IAM TARGET
  Account:            <account-id>
  Account Type:       [master / member]
  Enterprise Project: <enterprise-project or "account-wide">
  Target Principal:   <user/group/agency-name>
  Active Operator:    <identity>

CURRENT POLICY INVENTORY
  System Policies:    <list>
  Custom Policies:    <list>
  Agency Trusts:      <list or "none">
  Active SCPs:        <list or "none">

PROPOSED CHANGE
  Action:             <attach/detach/create/modify/delete>
  Policy/SCP:         <name>
  Blast Radius:       <description>
  Privilege Escalation Risk: [NONE / LOW / HIGH — reason]

AGENCY TRUST ASSESSMENT
  [NOT APPLICABLE] OR [trusted-principal: <name>, access-level: <level>]

APPROVAL STATUS
  Operator:           <identity>
  Approved:           [YES / NO / PENDING]
  SCP Test Confirmed: [YES / NO / N/A]

ACTION
  [BLOCKED — reason] OR [EXECUTED] OR [PENDING APPROVAL]

ROLLBACK
  Reverse action:     <describe>
  CTS event:          <event-id or "pending">

VERIFICATION
  Policy Applied:     [YES / NO]
  Access Test:        [PASS / FAIL / NOT YET RUN]
  CTS Logged:         [YES / NO / PENDING]
```
