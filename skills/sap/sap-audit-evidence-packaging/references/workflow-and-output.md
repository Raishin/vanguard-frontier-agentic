# Workflow and output contract — SAP Audit Evidence Packaging

Use this reference for evidence taxonomy, control-to-evidence mapping structure, chain-of-custody format, and output formatting.

## Evidence taxonomy

SAP audit evidence is classified by control domain and artifact type:

### Control domains

| Domain | Description |
|--------|-------------|
| `SoD` | Segregation of Duties — evidence that incompatible functions are not held by the same user |
| `Access Management` | User provisioning, deprovisioning, access review, role assignment, and privileged access controls |
| `Change Management` | Transport request lifecycle, approval chains, emergency change controls, and change freeze enforcement |
| `Financial Controls` | Posting authorization, period-end close controls, journal entry approval, and reconciliation evidence |
| `System Configuration` | SAP system parameter settings, security profile settings, audit log configuration, and baseline configuration |

### Artifact types

| Artifact type | Description | Typical source |
|---------------|-------------|---------------|
| `GRC SoD report` | Machine-generated SoD conflict report from GRC Access Control or equivalent | SAP GRC Access Control NWBC report / Pathlock / Fastpath |
| `Role assignment export` | Export of user-to-role or user-to-role-collection assignments at a point in time | BTP CLI, SUIM transaction, GRC Access Control, IAS export |
| `Transport log` | Log of transport request creation, release, import approvals, and execution timestamps | STMS transaction export, CTS+ log, Cloud ALM transport log |
| `Access review report` | Periodic access review completion record showing reviewer, outcome, and date | GRC Access Control recertification report, manual review record |
| `Audit log extract` | System audit log entries demonstrating monitoring is active and events are captured | SM19/SM20 ABAP audit log export, BTP audit log service export |
| `Security parameter screenshot or export` | System parameter values (login/fails_to_user_lock, auth/rfc_authority_check, etc.) | RZ11 transaction export, parameter configuration export |
| `Approval record` | Documented approval of a specific action (emergency access, transport import, user creation) | Workflow completion record, ServiceNow ticket, email chain with timestamp |
| `Period-end evidence` | Evidence that period-end close procedures were executed and controls operated | Financial posting report, period-lock screenshot, reconciliation sign-off |

## Control-to-evidence mapping structure

For each control, document:

```
Control ID:        <control reference from audit framework, e.g., SOX ITGC CC-01>
Control objective: <what the control is designed to prevent or detect>
Domain:            <SoD | Access Management | Change Management | Financial Controls | System Configuration>
Framework refs:    <SOC 2 CC6.1 | ISO 27001 A.9.2 | SOX ITGC | GxP Annex 11 clause X>
Required artifacts:
  - <artifact type 1>: <what exactly to collect; time window; format>
  - <artifact type 2>: <what exactly to collect; time window; format>
Chain-of-custody:
  - Source system: <SAP SID or BTP subaccount or GRC tenant>
  - Extraction method: <transaction/CLI command/API/manual export>
  - Extraction timestamp: <ISO 8601>
  - Extracted by: <role/person (no personal names in the mapping template)>
  - Transformations: <redactions applied; format conversions>
Completeness status: <sufficient | partial | missing>
Gap description:   <if partial or missing — what is absent and why>
```

## Chain-of-custody format

Every live evidence artifact included in an audit package must carry:

```
[CHAIN_OF_CUSTODY]
Artifact name:        <descriptive name, e.g., "SoD_Report_FI_2026Q1.xlsx">
Control ID(s):        <comma-separated control IDs this artifact supports>
Domain:               <SoD | Access Management | Change Management | Financial Controls | System Configuration>
Source system:        <SAP SID, BTP subaccount ID, or GRC tenant identifier>
Extraction method:    <GRC NWBC report / SUIM export / btp CLI / SM20 export / CloudALM API>
Extraction timestamp: <ISO 8601>
Extracted by role:    <GRC viewer / BTP subaccount viewer / ABAP display user>
Audit period covered: <start date ISO 8601> to <end date ISO 8601>
Redactions applied:   <yes/no — list what was redacted>
Reproducible:         <yes | no | partial — explanation>
Storage location:     <placeholder — do not include actual system paths with credentials>
Hash (SHA-256):       <file hash for tamper evidence — populate after file is finalized>
```

## Redaction requirements before packaging

The following must be redacted from all evidence artifacts before inclusion in an audit package:

| Data category | Redaction action |
|--------------|-----------------|
| SAP system password, client secret, API key | Replace value with `[REDACTED:CREDENTIAL]` |
| Employee personal name | Replace with employee ID only, or `[REDACTED:PII_NAME]` if employee ID is also not permitted |
| National identification number or social security number | Replace with `[REDACTED:PII_NID]` |
| Salary or compensation data | Replace with `[REDACTED:PII_SALARY]` |
| Customer name or customer personal data | Replace with `[REDACTED:CUSTOMER_PII]` |
| SAML signing certificate private key | Replace with `[REDACTED:SAML_PRIVATE_KEY]` |
| Connection string with embedded credentials | Replace with `[REDACTED:CONN_STRING]` |

After redaction, document what was redacted in the chain-of-custody `Redactions applied` field.

## Severity classification for evidence gaps

| Severity | Meaning | Examples |
|----------|---------|---------|
| `critical` | Control has no supporting evidence; gap cannot be closed before audit deadline | No SoD report extracted for the audit period; no transport approval records for the change management control |
| `high` | Evidence is partial; does not cover full audit period or key evidence artifact is missing | SoD report covers only part of the audit period; access review records missing for one system in scope |
| `medium` | Evidence is present but chain-of-custody is incomplete or redaction has not been applied | Evidence artifact lacks extraction timestamp or extracted-by role; personal data not yet redacted |
| `low` | Minor metadata gap or reproducibility risk only | File hash not yet calculated; storage location not yet documented in chain-of-custody |

## Framework alignment matrix

| Control domain | SOC 2 Trust Service Criteria | ISO 27001 Annex A | SOX ITGC | GxP (21 CFR Part 11 / EU Annex 11) |
|---------------|------------------------------|-------------------|----------|-------------------------------------|
| SoD | CC6.3, CC6.6 | A.9.2.3, A.9.4.1 | Access to Programs and Data | Annex 11 clause 12 (audit trail), clause 7 (access control) |
| Access Management | CC6.1, CC6.2, CC6.3 | A.9.2.1, A.9.2.2, A.9.2.6 | Logical Access | Annex 11 clause 7 |
| Change Management | CC8.1 | A.14.2.2, A.14.2.4 | Program Change | Annex 11 clause 10 |
| Financial Controls | CC4.1 (monitoring) | A.12.4.1 (logging) | Financial Reporting ITGC | N/A (GxP is process-industry focused) |
| System Configuration | CC6.6, CC7.1 | A.12.1.2, A.14.1.1 | Operations | Annex 11 clause 4 (validation), clause 11 (data storage) |

## Workflow

1. **Receive scope** — audit scope description, control list, or evidence request from user.
2. **Classify each control** by domain (SoD / Access Management / Change Management / Financial Controls / System Configuration).
3. **Map to required artifacts** — define the specific artifact types needed per control.
4. **Assess completeness** — review user-provided artifacts for sufficiency, period coverage, and chain-of-custody completeness.
5. **Flag gaps** — classify gaps by severity (critical / high / medium / low).
6. **Apply redaction checks** — identify any sensitive data requiring redaction.
7. **Align to frameworks** — map each control to specific SOC 2, ISO 27001, SOX ITGC, or GxP requirements.
8. **Return output** per the output contract below.

## Output contract

Return:

1. Audit scope summary (control domains in scope, audit period, frameworks applicable)
2. Control-to-evidence mapping for each control in scope (required artifacts, chain-of-custody requirements, framework mapping)
3. Completeness assessment per control (sufficient / partial / missing) with gap severity and description
4. Redaction advisory (which artifacts require redaction and what to redact)
5. Reproducibility assessment per artifact type
6. Escalation trigger if live SAP system access is required to gather missing evidence (with pointer to the appropriate live discovery skill)
