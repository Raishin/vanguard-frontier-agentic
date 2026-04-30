# Cost Budget Runaway — Preflight Commands

## 1. List all budgets and current utilization

```bash
oci budgets budget list \
  --compartment-id <TENANCY_OCID> \
  --all \
  --query 'data[].{name:"display-name", amount:amount, spent:"actual-spend", forecast:"forecasted-spend", reset:"reset-period"}' \
  --output table
```

## 2. Check compute GPU/HPC service limits

```bash
oci limits value list \
  --compartment-id <TENANCY_OCID> \
  --service-name compute \
  --all \
  --query 'data[?contains(name, `gpu`) || contains(name, `hpc`)].{name:name, value:value, scope:"scope-type"}' \
  --output table
```

## 3. Search for running GPU/HPC instances across tenancy

```bash
oci resource search search-resources \
  --query-text 'query instance resources where
    (shape = '"'"'BM.GPU4.8'"'"' ||
     shape = '"'"'VM.GPU3.1'"'"' ||
     shape = '"'"'BM.HPC2.36'"'"' ||
     shape = '"'"'BM.GPU.H100.8'"'"') &&
    lifecycleState = '"'"'RUNNING'"'"'' \
  --query 'data.items[].{id:"identifier", name:"display-name", compartment:"compartment-id"}'
```

## 4. Audit cost-tracking tag namespaces

```bash
oci iam tag-namespace list \
  --compartment-id <TENANCY_OCID> \
  --all \
  --query 'data[].{name:name, state:"lifecycle-state", isRetired:"is-retired"}' \
  --output table
```

## 5. Check active budget alerts

```bash
oci budgets alert list \
  --compartment-id <TENANCY_OCID> \
  --all \
  --query 'data[].{budgetId:"budget-id", threshold:threshold, triggered:"time-first-triggered"}'
```
