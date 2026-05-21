# Change Impact Categories Reference

Classification of Salesforce metadata changes by risk level, dependency
impact, and rollback complexity.

---

## Category 1: Destructive Changes

Destructive changes remove metadata from an org. They are irreversible if no
backup exists and can silently break dependent components.

### Destructive Change Types

| Metadata Type | API Name | Risk Level | Rollback Complexity |
|---------------|----------|------------|---------------------|
| Custom Object deletion | `CustomObject` | CRITICAL | High (data loss) |
| Custom Field deletion | `CustomField` | CRITICAL | High (data loss if not empty) |
| Apex Class removal | `ApexClass` | HIGH | Medium |
| Apex Trigger removal | `ApexTrigger` | HIGH | Medium |
| Flow deactivation | `Flow` | HIGH | Low |
| Flow version deletion | `Flow` | MEDIUM | Medium |
| Permission Set deletion | `PermissionSet` | HIGH | Medium |
| Named Credential deletion | `NamedCredential` | HIGH | Medium |
| Custom Label deletion | `CustomLabel` | MEDIUM | Low |
| Static Resource deletion | `StaticResource` | MEDIUM | Low |

### Destructive Change Deployment

Destructive changes are deployed via `destructiveChanges.xml` (pre-deployment)
or `destructiveChangesPost.xml` (post-deployment).

```xml
<!-- destructiveChangesPost.xml example -->
<?xml version="1.0" encoding="UTF-8"?>
<Package xmlns="http://soap.sforce.com/2006/04/metadata">
  <types>
    <members>OldHandler</members>
    <name>ApexClass</name>
  </types>
  <types>
    <members>Account.LegacyStatus__c</members>
    <name>CustomField</name>
  </types>
  <version>59.0</version>
</Package>
```

**Pre-deployment gate requirements for destructive changes:**
- Confirm the field/object is empty (zero records with non-null values).
- Identify all metadata referencing the removed component (use `sf scanner` or
  `sfdx-project.json` dependency analysis).
- Get explicit sign-off from org owner before deploying.

---

## Category 2: Dependent Metadata Chains

Changes to foundational metadata cascade impact to dependent components.
Use the Metadata API `listMetadata` and `readMetadata` calls to trace these chains.

### Common Dependency Chains

**Custom Object -> Downstream Dependents**
```
CustomObject
  -> CustomField (fields on the object)
  -> ValidationRule
  -> ApexTrigger (trigger on object)
  -> Flow (flows referencing object)
  -> Layout (page layouts)
  -> ListType (list views)
  -> PermissionSet (CRUD/FLS permissions)
  -> Report (reports using the object)
  -> Dashboard (dashboards from reports)
```

**Custom Field -> Downstream Dependents**
```
CustomField
  -> ValidationRule (references field)
  -> Formula fields (dependent on field value)
  -> Flow (Flow element referencing field)
  -> Apex code (field reference in SOQL/DML)
  -> LWC (getRecord fields, @salesforce/schema imports)
  -> Report columns
  -> PermissionSet (FLS entries)
```

**Apex Class -> Downstream Dependents**
```
ApexClass
  -> ApexTrigger (if trigger uses class)
  -> Other ApexClass (if class is a dependency)
  -> Flow (Apex action invocation)
  -> VF Page (if class is controller)
  -> PermissionSet (Apex class access)
```

### Tracing Dependencies via CLI

```bash
# List all references to a field in metadata
sf project retrieve start --metadata ApexClass,Flow,ValidationRule -o my-org
grep -r "LegacyStatus__c" force-app/ --include="*.{cls,xml,js}" -l

# Use Salesforce Dependency API (tooling API)
curl -s "$SF_INSTANCE_URL/services/data/v59.0/tooling/query/?q=SELECT+\
MetadataComponentName,MetadataComponentType+FROM+MetadataComponentDependency+\
WHERE+RefMetadataComponentName='LegacyStatus__c'" \
-H "Authorization: Bearer $SF_ACCESS_TOKEN"
```

---

## Category 3: Automation Chain Ripple Effects

Changes to one automation layer can trigger unintended execution in connected
layers. Understanding the execution order is critical before modifying any
automation.

### Salesforce Automation Execution Order (per-record)

```
1. System validation (required fields, data type checks)
2. Before-save flows (Record-Triggered Flows in Before Save mode)
3. Before triggers (Apex)
4. System validation again (after before triggers)
5. Duplicate Rules
6. After triggers (Apex)
7. Assignment Rules
8. Auto-Response Rules
9. Workflow Rules (LEGACY - being sunset)
10. After-save flows (Record-Triggered Flows in After Save mode)
    -> Escalation Rules
    -> Entitlement Rules
11. Processes (Process Builder - LEGACY)
12. Chatter notifications
```

Workflow Rules and Process Builder are on the Salesforce
end-of-life roadmap. Verify sunset dates before referencing in audit reports.

### Ripple Scenarios to Check

| Change | Potential Ripple | Check Before Deploying |
|--------|-----------------|----------------------|
| Add Before-Save Flow | Overwrites field before Apex trigger sees it | Review trigger field dependencies |
| Activate new Apex trigger | Recursive trigger if trigger updates same object | Check for recursion guards |
| Change field default value | May break validation rules checking for null | Scan all ValidationRule formulas |
| Add required field | Breaks flows/processes that create records without that field | Search flows for record-create actions |
| Change picklist value | Breaks flows/processes filtering on old value | Search flows for Equals comparisons on that field |
| Remove sharing rule | Records may become inaccessible to flows running in user context | Audit flows that use User-context on affected object |

### Recursion Guard Pattern (Apex)

```apex
// Prevent trigger recursion
public class AccountTriggerHandler {
    private static Boolean alreadyRunning = false;

    public static void onAfterUpdate(List<Account> newAccounts, Map<Id, Account> oldMap) {
        if (alreadyRunning) return;
        alreadyRunning = true;
        try {
            // trigger logic that may update Account records
        } finally {
            alreadyRunning = false;
        }
    }
}
```

---

## Change Risk Scoring Matrix

Use this matrix during change impact review:

| Dimension | Low (1) | Medium (2) | High (3) |
|-----------|---------|-----------|---------|
| Component type | Config (picklist value) | Apex/Flow | Schema (object/field) |
| Data at risk | 0 records | < 10,000 records | > 10,000 records |
| Dependent components | 0-2 | 3-10 | > 10 |
| Automation chain length | 0-1 steps | 2-3 steps | > 3 steps |
| Rollback complexity | Deploy inverse change | Restore from backup | Manual data repair |
| Business criticality | Low-traffic feature | Daily-use process | Revenue-critical path |

**Risk Score = Sum of dimension scores**
- 6-9: Low risk, standard deployment window acceptable.
- 10-14: Medium risk, require QA sandbox validation before production deploy.
- 15-18: High risk, require change freeze approval, staged rollout, and rollback plan.

---

## Rollback Checklist by Component Type

### Schema Changes (fields, objects)
- [ ] Export data from affected fields before deployment.
- [ ] Use `destructiveChangesPost.xml` not `destructiveChanges.xml` to delete
  after new components are deployed.
- [ ] Store data export in secure storage for 30-day recovery window.

### Apex Changes
- [ ] Previous version retrievable from Version Control.
- [ ] Test class coverage > 75% on incoming version.
- [ ] If rollback needed: retrieve previous version and redeploy.

### Flow Changes
- [ ] Previous active version noted before deactivation.
- [ ] Rollback: reactivate previous version.
- [ ] If version deleted: restore from source control and re-deploy.

### Permission Set Changes
- [ ] Clone permission set before modification for rollback reference.
- [ ] Document which users had access before change.
