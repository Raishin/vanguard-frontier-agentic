# Workflow and output contract — SAP CAP Architecture Review

Use this reference for all classification, severity assignment, and output formatting.

## Review domain taxonomy

| Domain | Scope |
|--------|-------|
| `CDS Modeling` | Entity definitions, associations, compositions, projections, aspects, annotations, view correctness |
| `Service Layer` | Service definition quality, action/function design, projection completeness, bypass patterns |
| `Authorization` | @requires and @restrict annotation coverage, role design, where-clause correctness, unguarded endpoints |
| `Multitenancy` | MTX service wiring, tenant lifecycle hooks, extensibility activation, subscriber passcode |
| `Draft Handling` | Draft-enabled entity completeness, draftActivate hooks, BeforeSave validation, side effects, cancel |
| `Testing` | cds.test setup, mock authentication, test isolation, CDS environment teardown, coverage |
| `Deployment Config` | package.json cds block, BTP profile wiring, service bindings, MTA descriptor completeness |

## Severity classification

| Severity | Meaning | Examples |
|----------|---------|---------|
| `critical` | Security breach or data integrity loss risk | Unguarded service with no @requires; exposed action with no authorization check |
| `high` | Functional or operational correctness failure | Incomplete draftActivate implementation; missing MTX tenant lifecycle hook; direct DB access bypassing service layer |
| `medium` | Governance, maintainability, or isolation gap | Service-level @requires only with no entity-level @restrict on sensitive entities; test not using in-memory SQLite |
| `low` | Best practice deviation or code quality concern | Missing @readonly on projection-only field; inconsistent CDS naming conventions |

## Workflow

1. **Receive artifacts** — CDS model files, service handler code, package.json, test files, MTX configuration, or user descriptions.
2. **Classify each finding** by review domain above.
3. **Assign severity** (critical / high / medium / low).
4. **Identify evidence level** (documentation-based / user-provided evidence / context7-supplementary / inference).
5. **Recommend specific remediation** — annotation to add, handler method to implement, configuration key to set, test pattern to adopt.
6. **Prioritize** — critical and high severity first; authorization findings before structural findings; multitenancy gaps before testing gaps.
7. **Return output** per the output contract below.

## Common finding patterns

### Authorization
- Service exposed with no `@requires` annotation (critical if externally reachable)
- Entity action with no `@restrict` or role check (critical for write operations)
- `@requires: 'any'` used on sensitive services (high — equivalent to unauthenticated access in some CAP versions)
- Missing `where` clause on `@restrict` for tenant-scoped data (high for multitenant apps)

### CDS Modeling
- `up_` associations exposed in projection service (medium — internal navigation leaked)
- `SELECT *` projection without field-level `@readonly` on computed fields (low)
- Missing `@assert.unique` on business key fields (medium)
- Composition of aspects without `@mandatory` on required fields (low)

### Multitenancy
- `cds.requires.multitenancy: true` set but `@sap/cds-mtxs` not in dependencies (high)
- Tenant upgrade hook (`/-/cds/saas-provisioning/upgrade`) not wired (high)
- Extensibility service activated without `cds.ExtensionDeveloper` role enforcement (critical)

### Draft Handling
- Draft-enabled entity with no `draftActivate` event handler (high)
- Missing `BeforeSave` validation handler (high for transactional correctness)
- Draft lock expiry not configured (medium)

### Testing
- Integration test using shared `cds.test` instance without `afterAll` teardown (medium)
- Mock authentication (`{ auth: { kind: 'mocked' } }`) not used in unit tests against auth-protected services (medium)
- No test coverage for CAP actions and functions (medium)

## Output contract

Return:

1. Artifacts reviewed and domains in scope
2. Finding(s) per domain with severity and evidence label
3. Specific remediation recommendation per finding (annotation text, handler method signature, configuration key, or test pattern)
4. Authorization summary — list of services and entities with their access control coverage
5. Prioritized remediation sequence (critical → high → medium → low)
6. Escalation trigger if live CAP runtime or BTP inspection is required before proceeding
