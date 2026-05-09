# Workflow and Output — Alibaba Cloud Live RAM Policy Change Guard

## Step-by-Step Workflow

### Phase 1: Identity and Scope Confirmation

1. Confirm the active RAM principal and account ID:
   ```
   aliyun ram GetAccountAlias
   aliyun sts GetCallerIdentity
   ```
2. List current policies attached to the target user or role:
   ```
   aliyun ram ListPoliciesForRole --RoleName <ROLE_NAME>
   aliyun ram ListPoliciesForUser --UserName <USER_NAME>
   ```
3. Describe the specific policy to be changed:
   ```
   aliyun ram GetPolicy --PolicyType Custom --PolicyName <POLICY_NAME>
   aliyun ram GetPolicyVersion --PolicyType Custom --PolicyName <POLICY_NAME> --VersionId <VERSION_ID>
   ```

### Phase 2: Blast-Radius Assessment

4. Determine the scope of the change:
   - RAM user/role attachment: single principal affected.
   - AdministratorAccess assignment: entire account affected — highest risk.
   - Control Policy change: all member accounts in the target OU affected.
5. List all roles and users currently attached to the policy being modified or deleted:
   ```
   aliyun ram ListEntitiesForPolicy --PolicyType Custom --PolicyName <POLICY_NAME>
   ```
6. Assess active STS tokens: identify services or applications known to use STS tokens derived from the target role or policy.

### Phase 3: Approval Gate

7. Present all evidence to the operator: account ID, principal, current policy inventory, proposed change, blast-radius assessment, STS token impact.
8. Require explicit written approval including acknowledgment of blast-radius and STS token impact.
9. Do not proceed until approval is received.

### Phase 4: Execution

10. Execute the approved mutation:
    - Attach policy to role:
      ```
      aliyun ram AttachPolicyToRole --PolicyType System --PolicyName AdministratorAccess --RoleName <ROLE>
      ```
    - Detach policy from role:
      ```
      aliyun ram DetachPolicyFromRole --PolicyType Custom --PolicyName <NAME> --RoleName <ROLE>
      ```
    - Delete custom policy (only when no entity is attached):
      ```
      aliyun ram DeletePolicy --PolicyName <NAME>
      ```
    - Create new policy version:
      ```
      aliyun ram CreatePolicyVersion --PolicyName <NAME> --PolicyDocument <DOCUMENT> --SetAsDefault true
      ```
11. Capture post-change inventory snapshot.

### Phase 5: Post-Change Verification

12. Confirm policy list reflects intended change:
    ```
    aliyun ram ListPoliciesForRole --RoleName <ROLE_NAME>
    ```
13. Test access for the affected principal using a read-only operation scoped to the new policy.
14. Check ActionTrail for the change event: query `ram` service events for the mutation.

## Expected Output Format

The agent response for a RAM policy change operation must include:

```
ACCOUNT IDENTITY
  Account ID:     <account-id>
  Active Principal: <ram-user-arn / role-arn>

POLICY/ROLE INVENTORY
  Target:         <RAM user / role name>
  Current Policies: [list]

PROPOSED CHANGE
  Action:         <ATTACH / DETACH / CREATE / DELETE / VERSION_UPDATE>
  Policy Name:    <policy-name>
  Policy Type:    <System / Custom>
  Blast Radius:   <single principal / entire account / OU scope>

STS TOKEN IMPACT
  Active Sessions: [NONE KNOWN | <description of services at risk>]
  Impact:          [NONE | SERVICES MAY FAIL ON NEXT CALL]

APPROVAL STATUS
  Operator:       <identity>
  Approved:       [YES / NO / PENDING]
  Blast radius acknowledged: [YES / NO]

ACTION
  [BLOCKED — reason] OR [EXECUTING] OR [COMPLETE]

ROLLBACK POSTURE
  [REVERSIBLE — detach policy to restore]
  OR
  [NOT REVERSIBLE — policy deleted; re-creation from backup required]

VERIFICATION
  Post-change policy list: [attached / detached as intended]
  ActionTrail event:       [logged / not yet confirmed]
```
