# Deprecated Metadata Reference

Reference for deprecated Salesforce standard objects, end-of-life API versions,
and legacy automation types that should be migrated or decommissioned.

<!-- verify-before-merge:2026-05-21 --> Deprecation timelines change. Always verify
current retirement dates with Salesforce official release notes.

---

## Deprecated Automation Types

### Workflow Rules

Workflow Rules are one of Salesforce's original automation tools. Salesforce
has announced their retirement.

**Current Status:** End-of-life scheduled. No new Workflow Rules can be created
in new orgs. Existing rules continue to function but Salesforce has announced
a retirement date.
<!-- verify-before-merge:2026-05-21 --> Verify retirement date.

**Migration Target:** Record-Triggered Flow (Before-Save or After-Save).

**Detection Query:**
```sql
SELECT Id, Name, Active, Description, TableEnumOrId
FROM WorkflowRule
WHERE Active = true
ORDER BY TableEnumOrId, Name
```

**Migration Guide:**
1. For each active Workflow Rule, identify its actions:
   - Field Update -> Before-Save Flow assignment element.
   - Email Alert -> After-Save Flow Send Email action.
   - Task -> After-Save Flow Create Records element.
   - Outbound Message -> After-Save Flow HTTP Callout or Platform Event.
2. Build equivalent logic in Flow.
3. Test in sandbox.
4. Deactivate Workflow Rule and activate Flow.

### Process Builder

Process Builder is a visual automation tool introduced as a Workflow Rule
replacement. It is now itself being retired in favor of Flow.

**Current Status:** End-of-life scheduled. No new processes can be created
in new orgs. <!-- verify-before-merge:2026-05-21 -->

**Migration Target:** Record-Triggered Flow.

**Detection Query:**
```sql
SELECT Id, ApiName, Label, Status, LastModifiedDate
FROM FlowDefinition
WHERE ProcessType = 'Workflow'
  AND Status = 'Active'
ORDER BY Label
```
Process Builder processes have `ProcessType = 'Workflow'` in the FlowDefinition metadata.

### Legacy Assignment Rules and Auto-Response Rules

Assignment Rules and Auto-Response Rules for Cases and Leads are not deprecated
but are not enhanced with new capabilities. They remain functional.

**Recommendation:** For complex assignment logic, migrate to Record-Triggered
Flows which offer more conditional power and better visibility.

---

## End-of-Life API Versions

Salesforce retires old API versions periodically. Apex code, integrations, and
metadata deployed with retired API versions may stop functioning.

**Retirement Pattern:** Salesforce typically retires API versions that are more
than ~3 years old. The exact retirement dates are published annually.

### Checking API Versions in Use

```bash
# Check API versions in Apex classes
sf data query \
  --query "SELECT Name, ApiVersion, Status FROM ApexClass WHERE Status = 'Active' ORDER BY ApiVersion ASC LIMIT 200" \
  --use-tooling-api \
  -o my-org

# Check API versions in Apex triggers
sf data query \
  --query "SELECT Name, ApiVersion, Status FROM ApexTrigger WHERE Status = 'Active' ORDER BY ApiVersion ASC LIMIT 200" \
  --use-tooling-api \
  -o my-org

# Check API versions in Flows
sf data query \
  --query "SELECT Label, ApiVersion FROM FlowDefinition WHERE Status = 'Active' ORDER BY ApiVersion ASC LIMIT 200" \
  --use-tooling-api \
  -o my-org
```

**Finding:** Any Apex class or trigger on API version < 50.0 should be reviewed
for update. Versions < 40.0 are likely to be in or near retirement.
<!-- verify-before-merge:2026-05-21 -->

### Updating API Version in Metadata

```xml
<!-- In force-app/main/default/classes/MyClass.cls-meta.xml -->
<?xml version="1.0" encoding="UTF-8"?>
<ApexClass xmlns="http://soap.sforce.com/2006/04/metadata">
    <apiVersion>59.0</apiVersion>  <!-- Update from old version -->
    <status>Active</status>
</ApexClass>
```

After updating the API version, test thoroughly — behavior changes between
API versions may affect code that relies on specific platform behaviors.

---

## Deprecated Standard Objects and Features

### Activities: Task and Event (Classic)

Task and Event objects are not deprecated, but their classic user interface
(Activity History related list in Classic UI) is. All activity management
should use the Lightning Experience activity timeline.

### Contracts (Status Field Values)

The `Contract` object's `Status` field picklist values are not customizable
in all editions. Review any automation that depends on hardcoded contract
status values.

### Documents Object

The `Document` object (legacy file storage associated with folders) is
functionally deprecated in favor of ContentDocument (Salesforce Files / CRM
Content). New development should use ContentDocument, ContentVersion, and
ContentDocumentLink.

```sql
-- Find code referencing Document object (legacy)
-- Check Apex classes for 'FROM Document' queries
SELECT Id, Name, ApiVersion
FROM ApexClass
WHERE Body LIKE '%FROM Document%'
```
<!-- verify-before-merge:2026-05-21 -->

### Chatter (Legacy Objects)

Several legacy Chatter objects and APIs have been superseded by newer
collaboration features. Review use of:
- `FeedItem` (preferred over legacy `UserProfileFeed`).
- Legacy `Chatter API` endpoints (superseded by Connect REST API).

---

## Salesforce Classic vs Lightning Experience

Salesforce Classic is not officially deprecated for all orgs but:
- New features are Lightning-only.
- Lightning Experience is required for Salesforce mobile app.
- Most ISV managed packages are Lightning-only.

**Detection:**
```sql
SELECT Id, Name, UserPreferencesHideChatterOnboardingSplash, UserPreferencesLightningExperiencePreferred
FROM User
WHERE IsActive = true
  AND UserPreferencesLightningExperiencePreferred = false
  AND Profile.UserLicense.Name != 'Guest'
LIMIT 200
```

Users still preferring Classic should be reviewed for migration readiness.

---

## Managed Package Lag

Managed packages installed from AppExchange may not be updated to current
Salesforce releases promptly. Outdated packages can:
- Reference deprecated APIs.
- Conflict with new Salesforce features.
- Introduce security vulnerabilities.

**Detection:**
```sql
SELECT SubscriberPackage.Name, SubscriberPackageVersion.MajorVersion,
       SubscriberPackageVersion.MinorVersion,
       SubscriberPackageVersion.BuildNumber,
       SubscriberPackageVersion.ReleaseState
FROM InstalledSubscriberPackage
ORDER BY SubscriberPackage.Name
```

Check each installed package's AppExchange listing or vendor documentation
for the current supported version.

**Package lag risk:**
- Package > 2 major versions behind: HIGH risk of deprecated API usage.
- Package with no updates in 12 months: MEDIUM risk; verify vendor support status.

---

## Deprecated Metadata Review Checklist

- [ ] Active Workflow Rules inventoried and migration plan documented.
- [ ] Active Process Builder processes inventoried and migration plan documented.
- [ ] No Apex code on API versions below the org's minimum supported version.
- [ ] No Flows on API versions more than 5 versions old.
- [ ] Document object usage reviewed — migrated to ContentDocument where possible.
- [ ] Managed packages are within 1-2 major versions of current release.
- [ ] Users still on Salesforce Classic identified and migration plan created.
- [ ] SOAP API integrations checked for deprecated API version usage.
