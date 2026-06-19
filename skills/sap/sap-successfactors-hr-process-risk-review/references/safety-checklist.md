# Safety checklist — SAP SuccessFactors HR Process Risk Review

Use before making any HR governance finding or remediation recommendation, especially for findings involving RBP access, PII fields, GDPR compliance, payroll integration, or JML lifecycle controls.

## Non-negotiables

- Do not access, connect to, or request access to any live SuccessFactors tenant, Employee Central OData API, Integration Center endpoint, or payroll system. This skill reviews governance artifacts only.
- Do not accept, request, or process actual employee records, national ID numbers, bank account numbers, salary figures, home addresses, health data, or any other personal data identifying real individuals. If the user supplies raw PII, redirect immediately — do not analyze the PII content.
- Do not accept SuccessFactors admin credentials, OData API keys, Integration Center basic auth credentials, or SFTP passwords for payroll file transfers.
- Do not recommend closing, deferring, or deprioritizing a finding of confirmed unauthorized access to compensation, health data, or national IDs without first escalating to HR leadership and legal or the data protection team.
- Do not recommend changes to RBP permission roles, permission groups, or target populations in a production SuccessFactors tenant based on advisory review alone. All RBP changes require testing in a sandbox tenant and HR leadership sign-off.
- Do not recommend removing a permission role from an active employee without confirming that the role removal will not block access required for the employee's current job responsibilities.
- Do not validate GDPR compliance or confirm data subject rights readiness from memory alone. Direct the user to verify against the current SAP SuccessFactors data privacy documentation and applicable data protection regulation for their jurisdiction.
- Do not assert that a terminated employee's access has been revoked without user-provided confirmation from an HR or IT system audit. Assume access is active until proven otherwise.

## What people get wrong

- **Confusing RBP permission groups with permission roles**: Permission groups define who holds a role (the grantee population); permission roles define what they can see or do (the permission). Access risk lies in the combination of role scope and target population, not in either alone. Reviewing permission roles without reviewing target population scope misses the most common over-permission pattern.
- **Treating UI suppression as a security control**: Fields hidden in the Employee Central UI via theme or view configuration are not secured if the underlying RBP permission role grants field-level read access. RBP field-level permission is the authoritative access control — UI suppression is cosmetic.
- **Missing JML triggered from manager self-service**: Managers can initiate job change events (promotions, transfers, cost center changes) in Employee Central without HR approval in some configurations. If manager-initiated job changes do not trigger RBP updates, the employee retains their previous access and the JML lifecycle is bypassed.
- **Ignoring Integration Center as a data flow risk**: Integration Center replication jobs run on a schedule and can expose more employee data fields to downstream systems than intended. Integration field mapping scope should be reviewed against the principle of data minimization.
- **Assuming erasure deletes integration copies**: Activating the Employee Central data erasure workflow deletes personal data from SuccessFactors but does not erase copies replicated to downstream payroll processors, third-party integration targets, or archived audit logs. A complete GDPR erasure process must cover all downstream copies.
- **Conflating position management approval with headcount budget approval**: An approved position workflow in Employee Central confirms that HR and a manager approved the position object. It does not confirm that the position is within the approved headcount budget in a financial planning system. Both controls must be assessed separately.
- **Overlooking rehire duplicate records**: When a previously terminated employee is rehired without using the correct rehire process, Employee Central may create a second active employee record. Duplicate records cause payroll and benefits errors and create incorrect JML audit trails. Rehire duplicate detection configuration must be assessed explicitly.

## When to push back

- Push back immediately when the user supplies actual employee records, national IDs, bank account numbers, or salary figures — redirect to provide anonymized or pseudonymized process descriptions instead.
- Push back when the user asks to confirm GDPR compliance or data subject rights readiness from memory alone without providing data privacy impact assessment documentation or Employee Central data privacy configuration exports.
- Push back when the request requires live SuccessFactors API access, OData query execution, or real-time HR report generation — state that live inspection is out of scope and ask the user to supply the relevant exports or summaries.
- Push back when the user asks to approve closing a finding of confirmed unauthorized access to compensation or health data without HR leadership and legal escalation.
- Push back when the user asks to recommend production RBP changes based solely on this advisory review — require sandbox testing and HR sign-off first.

## Evidence labels

- `documentation-based` — grounded in official SAP SuccessFactors Employee Central, RBP, integration, or data privacy documentation (help.sap.com)
- `user-provided evidence` — RBP configuration exports, permission role lists, org chart descriptions, integration mapping documentation, data privacy impact assessment summaries, or written process descriptions provided by the user
- `inference` — derived reasoning not directly confirmed by official docs or user evidence; must always be labeled as such
