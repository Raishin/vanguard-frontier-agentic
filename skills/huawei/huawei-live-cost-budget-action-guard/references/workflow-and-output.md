# Workflow and Output — Huawei Live Cost Budget Action Guard

## Step-by-Step Workflow

### Phase 1: Identity and Scope Confirmation

1. Confirm CBC account ID and enterprise project scope.
2. Capture current monthly spend for the affected scope.
3. Identify the specific action: budget threshold change, RI purchase, or CUD activation.

### Phase 2: Risk and Irreversibility Assessment

4. For budget threshold reduction: calculate delta between new threshold and current spend — flag if within 10% of current spend.
5. For RI/CUD: verify utilization data covers at least the last 30 days; confirm the resource flavor matches the RI type exactly.
6. State explicitly whether the action is reversible.

### Phase 3: Blast Radius Assessment

7. For budget threshold reduction: enumerate all services and resources in the budget scope that would be suspended on threshold breach.
8. For RI/CUD: enumerate the commitment amount, term, and hourly rate.

### Phase 4: Approval Gate

9. Present all evidence to the operator: current spend, proposed change, irreversibility, blast radius.
10. Require explicit written approval per Required Confirmation section.
11. Do not proceed until approval is received.

### Phase 5: Execution

12. Execute the approved action via CBC console:
    - Budget threshold: CBC console > Budget Management > Edit Budget
    - RI purchase: CBC console > Reserved Instances > Purchase
    - CUD activation: CBC console > Committed Use Discounts > Create

### Phase 6: Post-Action Verification

13. Confirm the change is reflected in CBC:
    - Budget threshold: verify new threshold value and SMN alert binding.
    - RI/CUD: verify commitment record shows correct amount, term, and start date.
14. Document the action in the associated change record.

## Expected Output Format

```
ACCOUNT IDENTITY
  Account:            <account-id>
  Enterprise Project: <enterprise-project>

ACTION TYPE
  [BUDGET THRESHOLD CHANGE | RI PURCHASE | CUD ACTIVATION]
  Detail:             <describe the specific change>

CURRENT SPEND
  Monthly spend:      <amount>
  Proposed threshold: <amount>
  Suspension risk:    [LOW / MEDIUM / HIGH / IMMEDIATE]

IRREVERSIBILITY
  [REVERSIBLE — threshold can be increased] OR [NON-REFUNDABLE — RI/CUD]

BLAST RADIUS
  Services in scope:  <list>
  Suspension trigger: [NONE / <threshold delta>]

APPROVAL STATUS
  Operator:           <identity>
  Approved:           [YES / NO / PENDING]

ACTION
  [BLOCKED — reason] OR [EXECUTING] OR [COMPLETE]

VERIFICATION
  CBC record confirmed: [YES / NO]
  SMN alert bound:      [YES / NO / N/A]
```
