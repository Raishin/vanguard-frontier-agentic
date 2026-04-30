# Preflight Commands: OCI Live IAM Policy Compartment Guard

Run these before any IAM policy modification. Paste sanitized output as evidence.

## 1. Confirm identity and tenancy

```bash
oci iam user list --query "data[?contains(\"defined-tags\".keys(@), 'Operations')].{name:name,id:id}" 2>/dev/null | head -20
# Or check active session
oci iam region-subscription list
```

## 2. List policies in target compartment

```bash
oci iam policy list \
  --compartment-id <COMPARTMENT_OCID> \
  --query "data[].{id:id,name:name,lifecycleState:\"lifecycle-state\",statements:statements}"
```

## 3. Inspect a specific policy

```bash
oci iam policy get --policy-id <POLICY_OCID> \
  --query "data.{name:name,statements:statements,versionDate:\"version-date\",freeformTags:\"freeform-tags\"}"
```

## 4. List dynamic groups and their matching rules

```bash
oci iam dynamic-group list \
  --query "data[].{name:name,id:id,matchingRule:\"matching-rule\",lifecycleState:\"lifecycle-state\"}"
```

## 5. Audit recent policy changes (Activity log)

```bash
oci audit event list \
  --compartment-id <TENANCY_OCID> \
  --start-time $(date -u -d '24 hours ago' +%Y-%m-%dT%H:%M:%SZ) \
  --query "data[?\"event-type\"=='com.oraclecloud.identitycontrolplane.updatepolicy'].{time:\"event-time\",user:data.\"request.headers\".\"opc-principal\"[0],policyId:data.\"request.path\"}"
```

## 6. Check for overly broad existing policies (anti-pattern scan)

```bash
oci iam policy list --compartment-id <TENANCY_OCID> --all \
  --query "data[?contains(to_string(statements), 'manage all-resources') || contains(to_string(statements), 'any-user')].{name:name,statements:statements}"
```
