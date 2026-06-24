# Workflow and output contract — SAP ABAP Cloud RAP Review

Use this reference for all classification, severity assignment, and output formatting.

## Review domain taxonomy

| Domain | Scope |
|--------|-------|
| `CDS Views` | View type selection (interface/projection/consumption), annotation completeness, metadata extensions, access control (DCL), UUID key design, draft table configuration |
| `BDEF` | Implementation type (managed/unmanaged), draft enablement, authorization master/dependent, alias, feature control, action/function declarations, association exposure |
| `Behavior Implementation` | AUTHORITY-CHECK coverage, action method completeness, validation message patterns, determination logic, forbidden language constructs |
| `Released API Compliance` | C1/C2 contract verification, NOT_RELEASED consumption, DEPRECATED object usage, tier-2 compliance |
| `ABAP Unit Tests` | Test double usage (if_abap_behv_test_environment / if_cds_test_environment), test class structure, ROLLBACK ENTITIES teardown, CREATE ENTITIES test correctness |
| `Object Composition` | Root vs. child entity hierarchy, composition relationship completeness, ETag strategy, UUID vs. semantic key trade-offs |

## Severity classification

| Severity | Meaning | Examples |
|----------|---------|---------|
| `critical` | Security breach or data integrity loss risk | AUTHORITY-CHECK simulation left active in production-bound code; authorization master method returning allowed without any check |
| `high` | Functional, correctness, or upgrade-blocking risk | NOT_RELEASED API consumption; missing `with draft;` on a BO requiring Fiori draft editing; missing draftActivate method; unmanaged RAP with no justification |
| `medium` | Governance, maintainability, or test isolation gap | ROLLBACK ENTITIES missing in test teardown; CDS metadata extension annotations on wrong view layer; unmanaged implementation where managed would suffice |
| `low` | Best practice deviation or code quality concern | Inconsistent CDS naming conventions; missing alias in BDEF; action not declared as static where appropriate |

## Common finding patterns

### BDEF
- `with draft;` absent on a root entity that requires Fiori Elements draft editing (high)
- `authorization master ( global )` declared but CHECK_AUTHORIZATION never implemented in the behavior implementation class (critical)
- `authorization dependent by _ParentAssociation` incorrectly used on the root entity (high — only valid for child entities)
- Unmanaged BDEF without documented justification on a greenfield BO (medium)

### Behavior Implementation
- `update_granted = abap_true` or `delete_granted = abap_true` hard-coded without actual `AUTHORITY-CHECK` — simulation left from development (critical)
- `AUTHORITY-CHECK OBJECT` used for update but not for delete (high — asymmetric authorization check)
- Missing `APPEND VALUE #(...) TO reported-travel` for unauthorized operations — authorization failure not surfaced to the caller (high)
- Validation method that returns without any `APPEND TO failed` or `APPEND TO reported` (high — silent failure)

### CDS Views
- Interface view directly used in service binding instead of projection view (medium — bypass of projection separation)
- Access control (DCL) missing on interface view (high for sensitive entities)
- Draft table not defined on the root CDS interface view while `with draft;` is in the BDEF (high — runtime error)
- Metadata extension annotations placed on interface view instead of projection view (medium)

### Released API Compliance
- Direct SELECT from SAP standard database table without C1 released CDS view equivalent (high)
- Consumption of a function module or BAPI with no C1/C2 release contract in ABAP Cloud context (high)
- Use of DEPRECATED released objects without noting migration path (medium)

### ABAP Unit Tests
- Test class uses `MODIFY ENTITIES` against live data without `cl_cds_test_environment` or `cl_abap_behv_test_environment` double (high)
- `ROLLBACK ENTITIES` absent in `teardown` method (medium)
- `cds_test_environment->destroy()` absent in `class_teardown` (medium)
- Test method asserts only initial result without asserting expected field values (low)

## Workflow

1. **Receive artifacts** — ABAP source code, CDS DDL files, BDEF source, metadata extensions, access control definitions, or user descriptions.
2. **Classify each finding** by review domain above.
3. **Assign severity** (critical / high / medium / low).
4. **Identify evidence level** (documentation-based / user-provided evidence / context7-supplementary / inference).
5. **Recommend specific remediation** — ABAP code pattern to apply, BDEF keyword to add, DCL source to create, API Business Hub check to perform.
6. **Prioritize** — critical and high severity first; released API compliance and authorization before structural findings; test isolation before style.
7. **Return output** per the output contract below.

## Output contract

Return:

1. Artifacts reviewed and domains in scope
2. Finding(s) per domain with severity and evidence label
3. Specific remediation recommendation per finding (with code-level detail from user-provided evidence where available)
4. Released API compliance summary — list any NOT_RELEASED or DEPRECATED objects with recommended C1/C2 alternatives (to be verified on API Business Hub)
5. Authorization coverage summary — AUTHORITY-CHECK status for each operation (create/update/delete) per reviewed behavior implementation
6. Prioritized remediation sequence (critical → high → medium → low)
7. Escalation trigger if live ABAP system access or ATC execution is required before proceeding
