# Workflow and Output — GCP Live Cost Budget Action Guard

## Step-by-Step Workflow

### Phase 1: Identity and Billing Account Confirmation

1. Confirm active gcloud identity:
   ```
   gcloud auth list
   gcloud config get-value account
   ```
2. List billing accounts accessible to the active principal:
   ```
   gcloud billing accounts list
   ```
3. Confirm the target billing account ID and its associated projects:
   ```
   gcloud billing projects list --billing-account=<BILLING_ACCOUNT_ID>
   ```

### Phase 2: Current Budget Inventory

4. List all budgets for the billing account (via Cloud Console or Billing Budgets API):
   ```
   # Via gcloud (if available):
   gcloud billing budgets list --billing-account=<BILLING_ACCOUNT_ID>
   ```
5. For each budget, capture: display name, amount, alert threshold percentages, notification channels, and programmatic notification Pub/Sub topics.

### Phase 3: CUD Commitment Inventory

6. List active committed-use discount commitments:
   ```
   gcloud compute commitments list --project=<PROJECT>
   ```
7. For each commitment, capture: resource type, region, plan (1-year or 3-year), status, start date, end date, and estimated monthly value.

### Phase 4: Quota Audit (for quota increase requests)

8. View current quotas for the target service and region:
   ```
   gcloud compute project-info describe --project=<PROJECT>
   # Or for specific quota:
   gcloud alpha services quota list --service=<SERVICE> --project=<PROJECT>
   ```
9. Calculate current utilization percentage and estimated spend at the new quota limit.

### Phase 5: Financial Authority Confirmation

10. Identify the financial authority approver (billing account owner, VP Engineering, or CFO-delegate).
11. Document their written approval, including: approver identity, date, action being approved, and acknowledged financial impact.
12. Do not proceed without confirmed financial authority.

### Phase 6: Execution

13. Execute the approved action:
    - Budget update: Via Cloud Console > Billing > Budgets & alerts > Edit, or Billing Budgets API.
    - CUD purchase:
      ```
      gcloud compute commitments create <COMMITMENT_NAME> \
        --project=<PROJECT> \
        --region=<REGION> \
        --resources=vcpu=<N>,memory=<N>GB \
        --plan=<TWELVE_MONTH|THIRTY_SIX_MONTH>
      ```
    - Quota increase: Via Cloud Console > IAM & Admin > Quotas > select quota > Edit Quotas.

### Phase 7: Post-Change Verification

14. Confirm the new budget threshold is reflected in the billing account.
15. For CUD purchases, confirm the new commitment appears:
    ```
    gcloud compute commitments describe <COMMITMENT_NAME> --project=<PROJECT> --region=<REGION>
    ```
16. Verify Cloud Monitoring budget alert channels are still active.
17. Check Cloud Audit Logs for the billing action.

## Expected Output Format

```
BILLING IDENTITY
  Billing Account: <billing-account-id>
  Display Name:   <account-name>
  Project:        <project-id>

ACTIVE PRINCIPAL
  Account:        <email>
  Billing Role:   [billing.admin | billing.viewer | MISSING]

BUDGET INVENTORY
  Budget count:   <N>
  Budget name:    <name>
  Amount:         $<amount> [monthly | annual]
  Alert levels:   <50% / 90% / 100% of actual spend>
  Pubsub topic:   [CONFIGURED | NOT CONFIGURED]

CUD INVENTORY
  Active CUDs:    <count>
  CUD #1:         <resource-type>, <region>, <plan>, expires <date>, ~$<monthly-value>/mo
  [additional CUDs...]

QUOTA AUDIT (if applicable)
  Service:        <service-name>
  Quota metric:   <metric>
  Current limit:  <N>
  Current usage:  <N> (<pct>%)
  Requested limit: <N>
  Max spend impact: ~$<estimate>/month at new limit

FINANCIAL AUTHORITY
  Approver:       <name / email>
  Authority:      [billing-owner | VP-Eng | CFO-delegate]
  Approved:       [YES / NO / PENDING]
  CUD non-cancellable acknowledged: [YES / NO / N/A]

ACTION
  [BLOCKED — reason]
  OR [EXECUTED — budget updated to $<amount> with alerts at <thresholds>]
  OR [EXECUTED — CUD commitment <name> purchased: <term>, ~$<value>/year]
  OR [SUBMITTED — quota increase request for <quota> to <N>]

ROLLBACK POSTURE
  Budget change:  [REVERSIBLE — restore to $<previous-amount>]
  CUD purchase:   [NOT REVERSIBLE — contact GCP Support P1 if in error]
  Quota increase: [REVERSIBLE — submit decrease request]

MONITORING
  Budget alerts:  [ACTIVE | CHECK REQUIRED]
  Audit log:      [FOUND — insertId: <id>]
```
