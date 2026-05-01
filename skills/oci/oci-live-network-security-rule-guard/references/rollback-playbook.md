# Rollback Playbook: OCI Live Network Security Rule Guard

OCI Security List and NSG rule changes take effect immediately with no native undo operation. The only rollback path is restoring the previous rule set from a captured baseline. **Capture current rules before every mutation — no exceptions.**

## Pre-mutation capture (mandatory)

```bash
# Security List — save to file before any change
oci network security-list get \
  --security-list-id <SECURITY_LIST_OCID> \
  --query 'data.{"ingress-security-rules":"ingress-security-rules","egress-security-rules":"egress-security-rules"}' \
  > securitylist-backup-$(date +%Y%m%d-%H%M%S).json

# NSG — save to file before any change
oci network nsg rules list \
  --nsg-id <NSG_OCID> --all \
  > nsg-backup-$(date +%Y%m%d-%H%M%S).json
```

## Restore Security List rules from backup

Security List update is a **full replace** — the update command overwrites the entire rule set. Pass the exact previous rules from the backup file.

```bash
# Restore ingress rules
INGRESS=$(cat securitylist-backup-<TIMESTAMP>.json | python3 -c "import json,sys; d=json.load(sys.stdin); print(json.dumps(d['ingress-security-rules']))")
oci network security-list update \
  --security-list-id <SECURITY_LIST_OCID> \
  --ingress-security-rules "$INGRESS" \
  --force

# Restore egress rules (same file, egress key)
EGRESS=$(cat securitylist-backup-<TIMESTAMP>.json | python3 -c "import json,sys; d=json.load(sys.stdin); print(json.dumps(d['egress-security-rules']))")
oci network security-list update \
  --security-list-id <SECURITY_LIST_OCID> \
  --egress-security-rules "$EGRESS" \
  --force
```

## Restore NSG rules from backup

NSG rule updates require rule IDs. To restore, remove new rules and re-add the old ones.

```bash
# List current rule IDs to identify added rules
oci network nsg rules list --nsg-id <NSG_OCID> --all --query 'data[].id'

# Remove a specific rule that was incorrectly added
oci network nsg rules remove \
  --nsg-id <NSG_OCID> \
  --security-rule-ids '["<RULE_ID_TO_REMOVE>"]'
```

## Verify restoration

```bash
# Confirm rules match the backup
oci network security-list get \
  --security-list-id <SECURITY_LIST_OCID> \
  --query 'data.{"ingress-security-rules":"ingress-security-rules","egress-security-rules":"egress-security-rules"}'
```

## Connectivity verification after rollback

```bash
# Check if affected instance can still reach expected endpoints
# (Run from inside the VCN or use OCI Network Path Analyzer)
oci network path-analyzer-test create \
  --compartment-id <COMPARTMENT_OCID> \
  --protocol-parameters '{"type":"TCP","destinationPort":<PORT>}' \
  --source-endpoint '{"type":"COMPUTE_INSTANCE","instanceId":"<INSTANCE_OCID>"}' \
  --destination-endpoint '{"type":"IP_ADDRESS","address":"<DEST_IP>"}'
```

## What cannot be rolled back

- Traffic that flowed through an incorrectly open rule during the window cannot be recalled.
- Data exfiltrated or connections established during the exposure window must be investigated separately via VCN Flow Logs.
- Enable Flow Logs on affected subnets before and after any security rule change for forensic coverage.
