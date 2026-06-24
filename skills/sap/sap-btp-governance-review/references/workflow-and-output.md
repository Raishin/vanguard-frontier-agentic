# Workflow and output contract — SAP BTP Governance Review

Use this reference for all finding classification, risk assignment, remediation path selection, and output formatting.

## Governance domain taxonomy

| Domain | Scope | Typical findings |
|--------|-------|-----------------|
| `account-structure` | Global account, directory layout, subaccount design | Flat structure with no directories, production/non-production in same subaccount, no naming convention |
| `entitlements` | Service entitlements and quota assignments at directory/subaccount level | Entitlement sprawl, unused quota, global-account-level entitlement when subaccount scope would suffice |
| `environments` | Cloud Foundry org/space, Kyma cluster provisioning | Over-provisioned CF quotas, unused environment instances, missing space-level quota restrictions |
| `role-collections` | Platform and application role collection assignments | Direct user assignment instead of IdP group mapping, over-permissive built-in role collections, unused custom role collections |
| `trust` | Identity provider trust configuration per subaccount | Default SAP ID Service trust not disabled when custom IdP is in use, unconfigured attribute mappings, redundant trust entries |

## Risk severity classification

| Risk level | Criteria |
|-----------|---------|
| `critical` | Security or compliance boundary violation: unauthorized access path, identity trust misconfiguration enabling unintended login, missing MFA enforcement on platform users |
| `high` | Operational risk: entitlement over-provisioning causing cost overrun, role collection granting global account admin to non-admin users, no emergency access procedure |
| `medium` | Governance gap: no directory structure in large BTP estate, direct user role assignment (vs. IdP group mapping), unused environment instances consuming quota |
| `low` | Best practice deviation: inconsistent naming convention, missing account tags, undocumented subaccount purpose |

## Remediation path decision tree

For each finding:

1. **Is this a trust misconfiguration with an active unauthorized login path?**
   - Yes → `critical`. Disable the trust entry or remove the misconfigured attribute mapping immediately. Do not defer.
   - No → continue.

2. **Does the finding grant excessive platform or global account admin access?**
   - Yes → `high`. Remove direct user assignments; replace with IdP group-mapped role collections scoped to least privilege.
   - No → continue.

3. **Is this an entitlement or quota over-provisioning issue?**
   - Yes → `high` or `medium` depending on scale. Reduce entitlement quota to match actual consumption + reasonable headroom. Move entitlement to subaccount scope if currently set at global account scope unnecessarily.
   - No → continue.

4. **Is this a structural gap (no directories, flat subaccount structure, no naming convention)?**
   - Yes → `medium`. Recommend adding a directory tier with entitlement-managed and managed-subaccount-lifecycle policies. Define naming and tagging conventions.
   - No → continue.

5. **Is this a best practice deviation with no immediate risk?**
   - Yes → `low`. Provide guidance for future alignment; do not block.

## Workflow

1. **Receive artifacts** — BTP account structure exports, cockpit screenshots, role collection lists, entitlement summaries, or user descriptions.
2. **Classify each finding** by governance domain.
3. **Assign risk level** (critical / high / medium / low).
4. **Apply remediation decision tree** per finding.
5. **Prioritize** — critical findings first; then high; then medium structural gaps; then low best-practice items.
6. **Return output** per the output contract below.

## Output contract

Return:

1. Governance domain and specific finding
2. Evidence label (documentation-based / user-provided evidence / inference)
3. Risk level per finding (critical / high / medium / low)
4. Recommended remediation action (trust removal, entitlement reduction, role collection restructure, directory addition, etc.)
5. Governance posture after remediation
6. Prioritized remediation sequence
7. Escalation trigger if live BTP cockpit access is required to confirm the finding before remediation
