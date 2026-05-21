# Release Checklist Reference

Pre-deployment gates and validation requirements for Salesforce releases
across change set, Salesforce DX, and unlocked package deployment paths.

---

## Deployment Gate Categories

### Gate 1: Code Quality (Static Analysis)

All items must pass before proceeding to QA.

- [ ] PMD scan: zero CRITICAL violations (Apex security and performance rules).
- [ ] PMD scan: zero HIGH violations (or all HIGH findings documented with
  accepted risk justification).
- [ ] ESLint scan: zero errors in LWC JavaScript files.
- [ ] RetireJS/npm audit: no HIGH or CRITICAL CVEs in static resources.
- [ ] No hardcoded Salesforce IDs in Apex, Flow, or Formula fields.
- [ ] All new Apex classes declare `with sharing` or `without sharing`
  explicitly (no implicit system mode).

### Gate 2: Test Coverage

- [ ] Apex test coverage: >= 75% aggregate (Salesforce deploy requirement).
- [ ] Apex test coverage: >= 85% on classes modified in this release (internal standard).
- [ ] All new Apex classes have at least one test class.
- [ ] All test classes include `System.assert()` statements (not just line coverage).
- [ ] No test classes use `@isTest(SeeAllData=true)`.
- [ ] Test classes pass in full sandbox with no `System.debug` errors.

```bash
# Run all tests via CLI before deploy
sf apex run test \
  --test-level AllLocalTests \
  --result-format tap \
  --code-coverage \
  -o sandbox-org-alias
```

### Gate 3: Flow and Automation

- [ ] All new record-triggered flows have fault paths on DML elements.
- [ ] No new DML-in-loop patterns in any flow.
- [ ] Any new flow has been tested manually in sandbox for all trigger conditions.
- [ ] Flows that replace Workflow Rules have been validated to produce identical outcomes.
- [ ] Flow API version is current (within 2 versions of org API version).

### Gate 4: Schema and Data Migration

- [ ] Destructive changes (field/object deletions) reviewed and approved.
- [ ] Data export of affected fields completed before deployment.
- [ ] Field deletions confirmed empty (zero records with non-null value).
- [ ] Required new fields have default values or are nullable (to prevent
  breaking existing records and automations).

### Gate 5: Security

- [ ] No new `@AuraEnabled` methods without FLS enforcement.
- [ ] No new Remote Site Settings with `DisableProtocolSecurity = true`.
- [ ] No new OAuth flows using Username-Password grant type.
- [ ] No secrets or credentials in any metadata file.
- [ ] Any new Connected App has IP restrictions configured.

### Gate 6: Performance

- [ ] SOQL queries in modified Apex reviewed for missing indexes.
- [ ] No new SOQL without LIMIT clauses on objects with > 100,000 records.
- [ ] Batch jobs have been tested with full production-scale data volumes in sandbox.
- [ ] No new synchronous callouts in trigger context.

---

## Change Freeze Windows

### Standard Change Freeze Periods

| Period | Freeze Level |
|--------|-------------|
| Salesforce Seasonal Release window (3x per year) | No deployments 24h before to 48h after release |
| End of fiscal quarter (last 2 weeks) | No non-critical changes |
| Black Friday / Cyber Monday (retail) | 2-week freeze |
| Holiday season (Dec 20 - Jan 3) | Full freeze except emergency |
| Major product launches | As defined by change board |

<!-- verify-before-merge:2026-05-21 --> Salesforce release schedule: verify current dates at
https://help.salesforce.com/s/articleView?id=release-dates

### Salesforce Release Timeline Impact

Salesforce releases three major updates per year (Spring, Summer, Winter).
Each release may change:
- Default behavior of existing features.
- Governor limit adjustments.
- API version deprecations.
- Security policy defaults.

**Pre-release checklist for Salesforce release windows:**
- [ ] Review Salesforce Release Notes for breaking changes relevant to org's features.
- [ ] Test org in preview sandbox (sandboxes receive the release 4 weeks before production).
- [ ] Validate all active Flows and Apex triggers in preview sandbox.
- [ ] Check Salesforce Optimizer for new recommendations added in the release.

---

## Sandbox Refresh Cadence

| Sandbox Type | Recommended Refresh | Purpose |
|-------------|---------------------|---------|
| Developer sandbox | Ad hoc | Individual developer work |
| Developer Pro sandbox | Weekly | Team feature development |
| Partial Copy sandbox | Monthly | Integration testing |
| Full Copy sandbox | Before major release | Production-scale performance testing |

### Post-Refresh Validation

After each sandbox refresh:
1. Run all active test classes: `sf apex run test --test-level AllLocalTests`.
2. Verify all Named Credential endpoints point to sandbox counterparts (not prod).
3. Confirm email deliverability is set to "System Email Only".
4. Run masking batch if sandbox contains production data.
5. Verify connected app OAuth callback URLs match sandbox URLs.

---

## Deployment Commands Reference

### Salesforce DX: Deploy to Org

```bash
# Validate without deploying (check-only)
sf project deploy start \
  --source-dir force-app \
  --target-org sandbox-alias \
  --check-only \
  --test-level RunLocalTests

# Deploy with test run
sf project deploy start \
  --source-dir force-app \
  --target-org sandbox-alias \
  --test-level RunLocalTests

# Deploy with specific test classes
sf project deploy start \
  --source-dir force-app \
  --target-org sandbox-alias \
  --test-level RunSpecifiedTests \
  --tests AccountTriggerTest ContactControllerTest
```

### Destructive Changes Deployment

```bash
# Deploy components AND destructive changes in one step
sf project deploy start \
  --source-dir force-app \
  --post-destructive-changes destructiveChangesPost.xml \
  --target-org prod-alias \
  --test-level RunLocalTests
```

---

## Pre-Deployment Sign-Off Matrix

| Role | Responsibility | Sign-Off Required For |
|------|---------------|----------------------|
| Developer | Code quality and test coverage | All changes |
| QA Engineer | Functional testing in sandbox | All changes |
| Security Reviewer | Security gates | Auth, FLS, integration changes |
| Business Owner | Functional acceptance | New features |
| Release Manager | Change freeze compliance | All production deploys |
| DBA/Architect | Schema changes | Object and field changes |

---

## Deployment Emergency Rollback Procedure

If a deployment must be rolled back within the deployment window:

1. Do NOT re-deploy the old version immediately — assess what data has been
   modified by the new version since deployment.
2. If DML has occurred on changed fields, data migration back may be required.
3. For Apex/Flow changes with no schema changes: re-deploy previous version from
   source control.
4. For schema changes (new fields): set field to deprecated/hidden rather than
   deleting (deletion requires empty field).
5. For destructive changes (deleted fields): restore is not possible — this is
   why pre-deployment data export is mandatory.
6. Document the incident, root cause, and rollback action in the change log.
