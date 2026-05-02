# Preflight Commands: OCI Live Network Security Rule Guard

Run all of these before adding, modifying, or removing any Security List or NSG rule.

## 1. Confirm active OCI profile and tenancy

```bash
oci iam region list --output table   # confirms CLI auth works
oci iam tenancy get --tenancy-id $(oci iam user get --user-id $(oci iam user list --query 'data[0].id' --raw-output) --query 'data."compartment-id"' --raw-output) 2>/dev/null || echo "Use: oci iam user list --all"
```

Simpler identity check:
```bash
oci iam user list --all --query 'data[0].{name:name, description:description}' --output table
```

## 2. Capture current Security List rules (CRITICAL — save as rollback baseline)

```bash
# Get current ingress and egress rules — save this output BEFORE any mutation
oci network security-list get \
  --security-list-id <SECURITY_LIST_OCID> \
  --query 'data.{"display-name":"display-name", "ingress-security-rules":"ingress-security-rules", "egress-security-rules":"egress-security-rules"}'
```

## 3. Capture current NSG rules (CRITICAL — save as rollback baseline)

```bash
oci network nsg rules list \
  --nsg-id <NSG_OCID> \
  --all \
  --query 'data[].{id:id, direction:direction, protocol:protocol, source:source, destination:destination, "source-type":"source-type", "tcp-options":"tcp-options", "udp-options":"udp-options", stateless:stateless}'
```

## 4. List Security Lists in a VCN to identify the target

```bash
oci network security-list list \
  --compartment-id <COMPARTMENT_OCID> \
  --vcn-id <VCN_OCID> \
  --query 'data[].{"display-name":"display-name", id:id, "lifecycle-state":"lifecycle-state"}'
```

## 5. Identify subnets attached to the Security List (blast radius)

```bash
oci network subnet list \
  --compartment-id <COMPARTMENT_OCID> \
  --vcn-id <VCN_OCID> \
  --query 'data[].{"display-name":"display-name", "cidr-block":"cidr-block", "security-list-ids":"security-list-ids", "prohibit-public-ip-on-vnic":"prohibit-public-ip-on-vnic"}'
```

`prohibit-public-ip-on-vnic: true` = private subnet. Ingress from 0.0.0.0/0 on a private subnet still allows internal CIDR access — confirm VCN CIDR scope.

## 6. Check if DB System or Autonomous DB is in the affected subnet

```bash
# List DB systems in compartment
oci db system list \
  --compartment-id <COMPARTMENT_OCID> \
  --query 'data[].{"display-name":"display-name", "subnet-id":"subnet-id", "lifecycle-state":"lifecycle-state"}'

# List Autonomous DBs
oci db autonomous-database list \
  --compartment-id <COMPARTMENT_OCID> \
  --query 'data[].{"db-name":"db-name", "subnet-id":"subnet-id", "lifecycle-state":"lifecycle-state"}'
```

If the affected subnet hosts a DB workload, classify the change as **critical** and require explicit DBA approval.
