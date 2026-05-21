# Rollback Strategy Reference

Rollback patterns and considerations for Salesforce releases across package,
change set, and direct metadata deployment paths.

---

## Rollback Principles

1. Plan rollback BEFORE deploying, not after something breaks.
2. Rollback complexity scales with the types of metadata changed.
3. Data that was written by the new version before rollback may create
   inconsistencies that require manual repair.
4. Schema changes (fields, objects) are significantly harder to roll back
   than code changes.

---

## Deployment Method Impact on Rollback

| Deployment Method | Rollback Speed | Complexity |
|-------------------|---------------|------------|
| Unlocked Package | Fast (install previous version) | Low |
| Managed Package | Fast (install previous version) | Low-Medium |
| Salesforce DX (source deploy) | Medium (re-deploy previous from VCS) | Medium |
| Change Sets | Slow (rebuild previous change set) | High |
| Direct Metadata Deploy | Slow (manual retrieval of previous version) | High |

---

## Rollback by Component Type

### Apex Classes and Triggers

**Rollback approach:** Re-deploy the previous version from source control.

```bash
# Check out previous version from git
git checkout HEAD~1 -- force-app/main/default/classes/AccountController.cls

# Deploy the reverted class
sf project deploy start \
  --source-dir force-app/main/default/classes/AccountController.cls \
  --target-org prod-alias \
  --test-level RunSpecifiedTests \
  --tests AccountControllerTest
```

**Key risk:** If the previous Apex version references a field that was also
removed in this deployment, the rollback will fail. Always roll back in reverse
order: restore code before restoring schema.

### Flows

**Rollback approach:** Re-activate the previous active version.

```bash
# Query version history
sf data query \
  --query "SELECT Id, VersionNumber, Status, ActiveVersion.VersionNumber \
           FROM FlowDefinition \
           WHERE ApiName = 'Account_Update_Flow'" \
  --use-tooling-api \
  -o prod-alias
```

In Setup > Flows > [Flow Name]:
- If previous version was not deleted: set previous version as Active.
- If previous version was deleted: restore from source control and re-deploy.

**Key risk:** Flows are versioned but old versions may be deleted automatically
when a new version is activated (depending on org setting). Confirm old versions
are retained before deployment.

### Custom Fields

**Rollback approach (new fields added):** Delete the field.

- Safe if no data was written to it.
- If data was written: export data first, then delete field, then re-import if
  rollback is needed later.

**Rollback approach (field deleted):** Cannot restore from Salesforce — must
re-create from source control metadata. Data is permanently lost unless backed up.

```bash
# Create field backup before deployment
sf data query \
  --query "SELECT Id, LegacyField__c FROM Account WHERE LegacyField__c != null" \
  --result-format csv \
  -o prod-alias \
  > legacy-field-backup-$(date +%Y%m%d).csv
```

### Permission Sets

**Rollback approach:** Re-deploy previous version or manually restore removed permissions.

```bash
# Re-deploy previous permission set version
sf project deploy start \
  --source-dir force-app/main/default/permissionsets/Sales_Rep.permissionset-meta.xml \
  --target-org prod-alias
```

### Named Credentials

**Rollback approach:** Re-deploy previous version. Credentials are stored separately
from metadata — re-deploying the Named Credential metadata restores the URL and
settings but does NOT restore passwords/secrets (these must be re-entered manually).

---

## Unlocked Package Rollback

Unlocked packages support version-based rollback:

```bash
# List installed package versions
sf package version list \
  --packages MyPackage \
  -o prod-alias

# Install previous version to roll back
sf package install \
  --package 04t... (previous version ID) \
  --target-org prod-alias \
  --wait 20
```

**Caution:** If the new version added a field or object that has data, rolling
back to the previous package version may fail because the previous version does
not include that field/object metadata — but the field still exists in the org.
You may need to manually delete the new fields before the previous version can
be installed cleanly.

---

## Destructive Change Rollback Considerations

Destructive changes (deleted fields, objects, classes) are the highest-risk
deployment type for rollback.

### Pre-Deployment Requirements (Mandatory)
- Export all data from fields being deleted.
- Store export in secure, versioned storage (e.g., S3 with versioning).
- Record the deletion in the change log with the export file location.

### If Rollback Is Needed After Destructive Change

1. **Field deleted, no data written in the interim:**
   - Re-create field with same API name.
   - Re-import data from pre-deployment export.

2. **Field deleted, new data exists (e.g., 2 hours of production use):**
   - Re-create field.
   - Re-import pre-deployment data.
   - Manually reconcile any new records created after deletion (no historical value for them).

3. **Object deleted:**
   - Re-create object and all fields.
   - Re-import data.
   - Re-create relationships (lookups from other objects will need re-configuration).
   - This is extremely high effort — treat object deletion as irreversible in practice.

---

## Rollback Decision Tree

```
Is the deployment causing production issues?
  YES -> Assess impact severity

    Is this a code/flow change only (no schema change)?
      YES -> Roll back code immediately via re-deploy
      NO  ->
        Was data written by the new code since deployment?
          NO  -> Roll back code and schema changes
          YES ->
            Can data inconsistency be manually corrected?
              YES -> Roll back, then run data correction script
              NO  -> Activate kill switch / feature flag instead of rollback
                     Escalate to emergency change board
```

---

## Feature Flags for Safe Rollback

For high-risk changes, implement a feature flag pattern to enable rapid
disable without a deployment:

```apex
// Custom Metadata: FeatureFlag__mdt
// Fields: IsEnabled__c (Checkbox), DeveloperName (Text)

public class FeatureFlag {
    private static Map<String, FeatureFlag__mdt> flagCache;

    public static Boolean isEnabled(String flagName) {
        if (flagCache == null) {
            flagCache = new Map<String, FeatureFlag__mdt>();
            for (FeatureFlag__mdt f : [SELECT DeveloperName, IsEnabled__c FROM FeatureFlag__mdt]) {
                flagCache.put(f.DeveloperName, f);
            }
        }
        return flagCache.containsKey(flagName) && flagCache.get(flagName).IsEnabled__c;
    }
}

// Usage in Apex trigger handler
if (FeatureFlag.isEnabled('New_Account_Routing_Logic')) {
    // New routing logic
} else {
    // Legacy routing logic
}
```

To roll back: set `IsEnabled__c = false` in the Custom Metadata record via
Setup — no code deployment required.

---

## Rollback Drill

Rollback plans should be tested at least quarterly:

1. Deploy a test change to a staging environment.
2. Simulate a production issue requiring rollback.
3. Execute the rollback procedure.
4. Measure time from incident detection to rollback completion.
5. Document actual time vs target (target: < 30 minutes for code changes,
   < 2 hours for schema changes).
6. Update runbook based on findings.
