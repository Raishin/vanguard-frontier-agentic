# Preflight Commands: OCI Live Cost Budget Runaway Guard

Run these before any budget modification. Paste sanitized output as evidence.

## 1. Confirm identity and tenancy target

```bash
oci iam region-subscription list --query "data[].{homeRegion:\"is-home-region\",regionName:\"region-name\"}"
oci budgets budget list --compartment-id <TENANCY_OCID> \
  --query "data[].{displayName:\"display-name\",amount:amount,actualSpend:\"actual-spend\",forecastedSpend:\"forecasted-spend\",lifecycleState:\"lifecycle-state\"}"
```

## 2. Inspect a specific budget

```bash
oci budgets budget get --budget-id <BUDGET_OCID> \
  --query "data.{displayName:\"display-name\",amount:amount,actualSpend:\"actual-spend\",percentUsed:\"percent-used\",alertRuleCount:\"alert-rule-count\",targets:targets}"
```

## 3. List alert rules on the budget

```bash
oci budgets alert-rule list --budget-id <BUDGET_OCID> \
  --query "data[].{displayName:\"display-name\",type:type,threshold:threshold,thresholdType:\"threshold-type\",recipients:recipients}"
```

## 4. Check current compute shape usage against quota

```bash
oci limits resource-availability get \
  --service-name compute \
  --limit-name standard-e4-core-count \
  --compartment-id <COMPARTMENT_OCID> \
  --availability-domain <AD>
```

## 5. Verify ONS topic is active (for alert routing)

```bash
oci ons topic get --topic-id <TOPIC_OCID> \
  --query "data.{displayName:\"display-name\",lifecycleState:\"lifecycle-state\"}"
```
