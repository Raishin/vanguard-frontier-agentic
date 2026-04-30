# IAM Policy Compartment — Preflight Commands

## 1. List all policies in target compartment

```bash
oci iam policy list \
  --compartment-id <COMPARTMENT_OCID> \
  --all \
  --query 'data[].{id:id, name:name, statements:statements}' \
  --output json
```

## 2. Scan for any-user / any-group policies (red-flag detector)

```bash
oci iam policy list \
  --compartment-id <TENANCY_OCID> \
  --all \
  --query 'data[].statements[]' \
  --output json | grep -i 'any-user\|any-group'
```

Zero results expected. Any hit is a required review item before proceeding.

## 3. List dynamic groups and current matching rules

```bash
oci iam dynamic-group list \
  --compartment-id <TENANCY_OCID> \
  --all \
  --query 'data[].{name:name, rule:"matching-rule", id:id}'
```

## 4. Review the specific policy to be changed

```bash
oci iam policy get \
  --policy-id <POLICY_OCID> \
  --query 'data.{name:name, statements:statements, version:"version-date"}'
```

## 5. Export current statements as rollback backup (ALWAYS before write)

```bash
oci iam policy get \
  --policy-id <POLICY_OCID> \
  --query 'data.statements' > /tmp/policy-backup-$(date +%Y%m%dT%H%M%S).json
echo "Backup saved. Proceed only after confirming backup is complete."
```
