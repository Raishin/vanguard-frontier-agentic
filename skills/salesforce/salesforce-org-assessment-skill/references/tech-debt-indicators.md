# Tech Debt Indicators Reference

Common technical debt patterns in Salesforce orgs with detection queries,
severity assessment, and remediation approaches.

---

## 1. Hardcoded IDs in Metadata

### Description
Salesforce record IDs (15 or 18 character) embedded in Apex code, Flows,
Validation Rules, Formula fields, or Custom Settings.

### Why It Creates Debt
- IDs differ between sandboxes and production.
- IDs become stale when records are deleted and recreated.
- Forces code change for what should be a configuration change.

### Detection
```bash
# Find hardcoded IDs in Apex
grep -rn "[0-9A-Za-z]\{15\}\|[0-9A-Za-z]\{18\}" \
  --include="*.cls" \
  force-app/main/default/classes/ | \
  grep -v "//\|test\|mock" | \
  grep -E "[0-9][A-Za-z]{2}[0-9A-Za-z]{12}"
```

**PMD Rule:** `AvoidHardcodingId`

### Remediation
Replace with dynamic resolution:
```apex
// Replace this:
Id supportQueueId = '00G0000000XYZabc';

// With this:
Id supportQueueId = [SELECT Id FROM Group WHERE Name = 'Support Queue' AND Type = 'Queue' LIMIT 1].Id;

// Or use Custom Metadata for org-portable config:
OrgConfig__mdt config = [SELECT SupportQueueId__c FROM OrgConfig__mdt WHERE DeveloperName = 'Default' LIMIT 1];
Id supportQueueId = config.SupportQueueId__c;
```

---

## 2. Deprecated Metadata (Workflow Rules, Process Builder)

### Detection Queries

```sql
-- Active Workflow Rules
SELECT Id, Name, TableEnumOrId, Description
FROM WorkflowRule
WHERE Active = true
ORDER BY TableEnumOrId

-- Active Process Builder
SELECT Label, ApiName, LastModifiedDate, Status
FROM FlowDefinition
WHERE ProcessType = 'Workflow'
  AND Status = 'Active'
ORDER BY Label
```

### Debt Score by Count

| Active Workflow Rules | Tech Debt Level |
|----------------------|----------------|
| 0 | None |
| 1-10 | Low |
| 11-50 | Medium |
| > 50 | High |

---

## 3. Managed Package Lag

### Description
Installed AppExchange packages that have not been updated to recent versions
accumulate API compatibility risks and security vulnerabilities.

### Detection
```sql
SELECT SubscriberPackage.Name,
       SubscriberPackageVersion.MajorVersion,
       SubscriberPackageVersion.MinorVersion,
       SubscriberPackageVersion.PatchVersion,
       SubscriberPackageVersion.ReleaseState
FROM InstalledSubscriberPackage
ORDER BY SubscriberPackage.Name
```

### Lag Assessment

| Versions Behind Current Release | Risk Level |
|--------------------------------|-----------|
| 0-1 minor | Low |
| 2-3 minor | Medium |
| 1+ major | High |
| 2+ major | Critical |

### Remediation
1. Check the AppExchange listing for the package for current version.
2. Review the package's release notes for breaking changes.
3. Test upgrade in full sandbox.
4. Schedule production upgrade during change window.

---

## 4. Custom Field Bloat

### Detection

```apex
// Anonymous Apex: field count per custom object
List<String> bloatedObjects = new List<String>;
for (Schema.SObjectType objType : Schema.getGlobalDescribe.values) {
    Schema.DescribeSObjectResult describe = objType.getDescribe;
    if (!describe.isCustom) continue;
    Integer fieldCount = describe.fields.getMap.size;
    if (fieldCount > 80) {
        bloatedObjects.add(describe.getName + ': ' + fieldCount + ' fields');
    }
}
for (String entry : bloatedObjects) System.debug(entry);
```

### Standard Object Thresholds

| Object | Warning Threshold | Critical Threshold |
|--------|------------------|--------------------|
| Account | 80 fields | 150 fields |
| Contact | 60 fields | 120 fields |
| Opportunity | 70 fields | 140 fields |
| Lead | 50 fields | 100 fields |
| Case | 50 fields | 100 fields |

### Finding Orphaned Fields (Unused)

Use Salesforce Optimizer (Setup > Salesforce Optimizer) to generate a field
usage report. Fields with 0% usage in reports, list views, and page layouts
for > 6 months are candidates for archival and deletion.

---

## 5. Test Coverage Below Threshold

Salesforce requires 75% aggregate Apex code coverage to deploy to production.
Best practice is 85%+ with meaningful assertions.

### Detection
```sql
SELECT PercentCovered
FROM ApexOrgWideCoverage
```

For per-class coverage:
```sql
SELECT ApexClassOrTrigger.Name, NumLinesCovered, NumLinesUncovered,
       (NumLinesCovered / (NumLinesCovered + NumLinesUncovered + 0.0001)) * 100 AS CoveragePercent
FROM ApexCodeCoverageAggregate
ORDER BY CoveragePercent ASC
LIMIT 50
```

### Quality Indicators of Test Coverage

Low coverage is not the only indicator of poor testing quality:
- Tests with 0 assertions (`System.assert` calls) — these cover lines but
  verify nothing.
- Tests that never test failure scenarios.
- Tests that use `SeeAllData=true` (indicates tests are not isolated).

```sql
-- Find test classes with SeeAllData=true (isolation debt)
SELECT Id, Name, Body
FROM ApexClass
WHERE Name LIKE '%Test%'
  AND Body LIKE '%SeeAllData=true%'
```

---

## 6. Excessive Use of `System.debug`

### Description
Large volumes of debug statements slow Apex execution (debug logging has I/O
cost) and make logs unusable for diagnosis. PMD flags `System.debug` without
a logging level.

### Detection
```bash
grep -rn "System.debug(" --include="*.cls" force-app/ | \
  grep -v "LoggingLevel\." | wc -l
```

More than 200 ungoverned debug statements in production code is a MEDIUM finding.

### Correct Pattern
```apex
// WRONG: ungoverned
System.debug('Processing account: ' + acc.Id);

// CORRECT: use appropriate level
System.debug(LoggingLevel.DEBUG, 'Processing account: ' + acc.Id);
System.debug(LoggingLevel.WARN, 'Account has no owner: ' + acc.Id);
System.debug(LoggingLevel.ERROR, 'Failed to update account: ' + err.getMessage);
```

---

## 7. No Trigger Handler Pattern

### Description
Trigger logic written directly in `.trigger` files rather than in handler
classes. Makes unit testing, extension, and refactoring difficult.

### Detection
```bash
# Find trigger files with more than 20 non-blank lines of logic
for f in force-app/main/default/triggers/*.trigger; do
    lines=$(grep -v "^[[:space:]]*$\|^//\|^trigger" "$f" | wc -l)
    if [ "$lines" -gt 20 ]; then
        echo "$f: $lines lines of logic in trigger body"
    fi
done
```

### Remediation
Refactor to Trigger Handler pattern:
- Trigger file: <= 10 lines (just routing calls to handler).
- Handler class: all business logic, unit-testable in isolation.
- See `apex-anti-patterns.md` for trigger handler pattern example.

---

## Tech Debt Summary Dashboard

| Indicator | Query/Check | Healthy | Warning | Critical |
|-----------|------------|---------|---------|---------|
| Workflow Rules | COUNT active | 0 | 1-20 | > 20 |
| Process Builder | COUNT active | 0 | 1-10 | > 10 |
| Apex test coverage | ApexOrgWideCoverage | > 85% | 75-85% | < 75% |
| Hardcoded IDs in code | PMD scan count | 0 | 1-5 | > 5 |
| Account custom fields | COUNT fields | < 80 | 80-120 | > 120 |
| Package lag | Max major versions behind | 0 | 1 | > 1 |
| Apex API versions | Minimum version | > 55.0 | 45-55 | < 45 |
| Tests with SeeAllData | COUNT | 0 | 1-5 | > 5 |

API version thresholds should be updated relative to current
Salesforce release at assessment time.
