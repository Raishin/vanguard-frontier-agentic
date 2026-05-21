# Delegation Routing Reference — salesforce-metadata-fetcher-skill

This reference defines the routing map from retrieved metadata type to the correct downstream
review skill. It specifies the rationale for each route, the required handoff fields per route,
and the conditions under which multiple downstream skills must be invoked.

The metadata fetcher skill always declares the downstream skill recommendation in
`downstream_skill_recommendation` before emitting output. If multiple skills are relevant,
all are listed with their own handoff field sets.

---

## Routing Decision Tree

```
Metadata type retrieved
│
├─ CustomObject, CustomField, ValidationRule, RecordType, Layout
│   └─→ salesforce-metadata-review-skill
│
├─ Flow (AutoLaunchedFlow, ScheduledFlow, ScreenFlow, Orchestration)
│   └─→ salesforce-flow-automation-review-skill
│
├─ PermissionSet, PermissionSetGroup, Profile
│   └─→ salesforce-permission-model-review-skill
│
├─ ApexClass, ApexTrigger
│   └─→ salesforce-apex-lwc-code-review-skill
│
├─ LightningComponentBundle, AuraDefinitionBundle
│   └─→ salesforce-apex-lwc-code-review-skill
│
├─ ConnectedApp
│   └─→ salesforce-integration-agent (primary)
│       + salesforce-certificate-lifecycle-agent (if cert thumbprint present)
│
├─ CustomMetadata, CustomSetting (type definitions only)
│   └─→ salesforce-metadata-review-skill
│
└─ Mixed / multi-type retrieval
    └─→ Declare each type's downstream skill separately
        Route to salesforce-org-assessment-skill if > 3 types with findings
```

---

## Route 1 — ObjectDescribe → salesforce-metadata-review-skill

**Metadata types covered:**

- `CustomObject`
- `CustomField`
- `ValidationRule`
- `RecordType`
- `PageLayout`
- `FlexiPage` (Lightning record pages)
- `CustomMetadata` (type definition, not record values)
- `CustomSetting` (type definition, not record values)

**Rationale:**

The `salesforce-metadata-review-skill` is the correct consumer for object schema, field
definitions, validation rule formulas, record type assignments, and layout configurations.
It reviews for over-customization, unused fields, hardcoded IDs, and deprecated metadata patterns.
This is the most common route — the majority of admin metadata review requests land here.

**Required handoff fields:**

```yaml
downstream_skill_recommendation:
  skill_id: salesforce-metadata-review-skill
  rationale: "Object/field schema and validation rules require static review for hardcoded IDs, unused fields, and over-customization."
  required_handoff_fields:
    object_summary:
      api_name: "<ObjectApiName>"
      label: "<Label>"
      custom: <boolean>
      field_count: <integer>
      custom_field_count: <integer>
      record_type_count: <integer>
    field_inventory:
      - api_name: "<FieldApiName>"
        label: "<Label>"
        type: "<FieldType>"
        custom: <boolean>
        required: <boolean>
        default_value: "<sanitized_value_or_null>"
        formula: "<sanitized_formula_or_null>"
        fls_accessible: <boolean>
        encrypted: <boolean>
    validation_rules_summary:
      - name: "<RuleName>"
        active: <boolean>
        error_condition_formula: "<sanitized_formula>"
        description: "<Description>"
    audit_envelope:
      # Full audit envelope from fetcher skill
```

**Escalation to `salesforce-org-assessment-skill`:** If the object has > 100 custom fields,
> 10 validation rules, or appears to duplicate standard object functionality, declare an
escalation recommendation alongside the primary handoff.

---

## Route 2 — FlowMetadata → salesforce-flow-automation-review-skill

**Metadata types covered:**

- `Flow` (all process types: AutoLaunchedFlow, ScreenFlow, ScheduledFlow, ContactRequestFlow)
- `FlowDefinition` (if retrieved separately)

**Rationale:**

The `salesforce-flow-automation-review-skill` reviews flow logic for fault path coverage,
subflow error handling, automation conflicts, governor-limit risk in data manipulation,
and screen flow accessibility. It accepts sanitized flow XML or equivalent JSON structure
and produces structured findings.

**Required handoff fields:**

```yaml
downstream_skill_recommendation:
  skill_id: salesforce-flow-automation-review-skill
  rationale: "Flow metadata requires logic review for fault paths, automation conflicts, governor-limit risk, and hardcoded IDs."
  required_handoff_fields:
    flow_xml_sanitized:
      # Sanitized flow definition as structured JSON (not raw XML)
      # org IDs, user IDs, and hardcoded record IDs redacted
      api_name: "<FlowApiName>"
      master_label: "<Label>"
      process_type: "<ProcessType>"
      status: "<Active|Draft|Obsolete>"
      api_version: "<version>"
      elements:
        # Array of flow elements (decisions, loops, record creates, subflows, etc.)
        # Sanitized per sanitization-rules.md
    fault_path_present:
      # Boolean: does the flow have at least one fault connector?
      any_fault_connector: <boolean>
      elements_missing_fault_path:
        - "<ElementName>"  # Elements that have external callouts or DML but no fault connector
    automation_mix_summary:
      # Summary of automation types active on the same objects this flow touches
      objects_affected:
        - "<ObjectApiName>"
      other_automation_types_present:
        # List of other automation types (Workflow, Process Builder, Trigger, other Flows) on same objects
        # This is advisory — the fetcher can note what it knows; the review skill will do the deep analysis
        - type: "<type>"
          count: <integer>
    audit_envelope:
      # Full audit envelope from fetcher skill
```

**Stop condition that supersedes route:** If the flow body contains a hardcoded session ID
(`UserInfo.getSessionId` called within a flow formula), stop before handoff and escalate.

---

## Route 3 — ProfilePermissionSet → salesforce-permission-model-review-skill

**Metadata types covered:**

- `PermissionSet`
- `PermissionSetGroup`
- `Profile`
- `MutingPermissionSet`

**Rationale:**

The `salesforce-permission-model-review-skill` analyzes permission topology for toxic
combinations, excessive system permissions, ViewAllData on PII objects, guest user exposure,
and redundant grants between profiles and permission sets. It requires structured summaries
of the grants — not raw XML — because the raw Profile XML can be very large and contain
references that need contextual review.

**Required handoff fields:**

```yaml
downstream_skill_recommendation:
  skill_id: salesforce-permission-model-review-skill
  rationale: "Permission set / profile metadata requires toxic-combination analysis, ViewAllData PII check, and redundant-grant detection."
  required_handoff_fields:
    permission_set_summary:
      api_name: "<PermSetApiName>"
      label: "<Label>"
      type: "PermissionSet | Profile | PermissionSetGroup"
      license: "<License>"  # e.g., Salesforce, CRM User, etc.
      assignee_count: <integer>  # if retrievable; null if not available
    system_perms_granted:
      # List of system permissions set to true
      # CRITICAL fields to include: ModifyAllData, ViewAllData, ViewEncryptedData,
      #   AuthorApex, ManageConnectedApps, ApiEnabled, ViewSetup
      - permission_name: "<PermissionName>"
        granted: true
    object_perms_summary:
      # For each object with non-default permissions
      - object_api_name: "<ObjectApiName>"
        read: <boolean>
        create: <boolean>
        edit: <boolean>
        delete: <boolean>
        view_all: <boolean>
        modify_all: <boolean>
    fls_summary:
      # Sample of FLS grants — full FLS list may be very large; summarize sensitive objects
      sensitive_objects_with_full_fls:
        - "<ObjectApiName>"  # Objects where all fields are readable+editable
      inaccessible_fields_noted:
        - "<ObjectApiName>.<FieldApiName>"
    audit_envelope:
      # Full audit envelope from fetcher skill
```

**Profile note:** Full Profile XML retrieval may require elevated permissions (`Customize Application`).
If the profile is retrieved via `sf project retrieve start`, declare `elevated_path_used: true`
in the audit envelope. If only a profile list was retrieved (not full XML), declare
`items_retrieved_mode: list_only` and note in `missing_evidence`.

---

## Route 4 — ApexClass / ApexTrigger → salesforce-apex-lwc-code-review-skill

**Metadata types covered:**

- `ApexClass`
- `ApexTrigger`

**Rationale:**

The `salesforce-apex-lwc-code-review-skill` reviews Apex for sharing keyword omission, SOQL
and DML inside loops, `WITH SECURITY_ENFORCED` / `stripInaccessible` usage, governor-limit
risk, test coverage patterns, and async job patterns. It accepts sanitized Apex source code.

**Required handoff fields:**

```yaml
downstream_skill_recommendation:
  skill_id: salesforce-apex-lwc-code-review-skill
  rationale: "Apex class/trigger requires code review for sharing keywords, SOQL-in-loop, governor-limit risk, and security enforced usage."
  required_handoff_fields:
    class_name: "<ClassName or TriggerName>"
    apex_type: "ApexClass | ApexTrigger"
    api_version: "<version>"
    with_sharing_status:
      # One of: with_sharing, without_sharing, inherited_sharing, omitted
      declared: "<with sharing | without sharing | inherited sharing | omitted>"
    soql_count:
      # Count of SOQL queries in the class body (static analysis)
      total: <integer>
      in_loops: <integer>  # SOQL queries found inside for/while/do-while loops
    dml_count:
      # Count of DML statements in the class body (static analysis)
      total: <integer>
      in_loops: <integer>
    complexity_indicators:
      # Flags for review skill to focus on
      hardcoded_id_count: <integer>
      has_future_method: <boolean>
      has_batch_interface: <boolean>
      has_queueable_interface: <boolean>
      has_external_callout: <boolean>
      uses_stripInaccessible: <boolean>
      uses_security_enforced: <boolean>
      has_without_sharing_on_pii_object: <boolean>
    sanitized_apex_body: |
      # Full sanitized Apex source code
      # Hardcoded IDs replaced with <record_id_placeholder>
      # Session ID literals flagged (do not include if S-APEX-01 escalation fired)
    audit_envelope:
      # Full audit envelope from fetcher skill
```

**Pre-condition check before handoff:** If S-APEX-01 (`UserInfo.getSessionId` exfiltration)
fired during sanitization, **do not hand off to the review skill**. Emit a stop message first.
Only after the escalation has been acknowledged by a human operator should the code be passed
for review.

---

## Route 5 — LWC / Aura → salesforce-apex-lwc-code-review-skill

**Metadata types covered:**

- `LightningComponentBundle` (LWC)
- `AuraDefinitionBundle` (Aura)

**Rationale:**

LWC and Aura components may contain XSS-vulnerable patterns (`lwc:dom="manual"`, `innerHTML`,
unsafe `eval`), hardcoded record IDs, and Apex adapter calls that need security review. The
`salesforce-apex-lwc-code-review-skill` covers both Apex and frontend component review.

**Required handoff fields:**

```yaml
downstream_skill_recommendation:
  skill_id: salesforce-apex-lwc-code-review-skill
  rationale: "LWC/Aura component requires review for XSS patterns, hardcoded IDs, and Apex adapter security."
  required_handoff_fields:
    component_name: "<ComponentName>"
    component_type: "LWC | Aura"
    js_imports:
      # List of @salesforce/* and c/* imports declared in the component JS
      - "<import_statement>"
    apex_calls:
      # List of Apex methods called via @wire or imperative import
      - apex_class: "<ClassName>"
        method: "<MethodName>"
        pattern: "wire | imperative"
    lwc_security_concerns:
      # Flags for review skill to focus on
      uses_lwc_dom_manual: <boolean>
      uses_innerHTML: <boolean>
      uses_eval: <boolean>
      hardcoded_id_count: <integer>
      uses_navigate_page_ref_type_standard_record_page: <boolean>
      renders_user_input_directly: <boolean>
    sanitized_js_body: |
      # Sanitized component JavaScript (main JS file)
    sanitized_html_template: |
      # Sanitized HTML template
    audit_envelope:
      # Full audit envelope from fetcher skill
```

---

## Route 6 — ConnectedApp → salesforce-integration-agent + salesforce-certificate-lifecycle-agent

**Metadata types covered:**

- `ConnectedApp`

**Rationale:**

Connected App metadata exposes OAuth scope grants, IP relaxation settings, and certificate
thumbprints — all of which require separate review tracks. The `salesforce-integration-agent`
handles the integration security posture (scope, callback URLs, OAuth policies) while
`salesforce-certificate-lifecycle-agent` handles the certificate chain and expiry.

**This route always invokes two downstream skills.**

**Required handoff fields — salesforce-integration-agent:**

```yaml
downstream_skill_recommendation:
  - skill_id: salesforce-integration-agent
    rationale: "Connected App OAuth scopes, IP relaxation, and callback URL policies require integration security review."
    required_handoff_fields:
      app_name: "<ConnectedAppLabel>"
      api_name: "<ConnectedAppApiName>"
      oauth_scopes:
        # List of OAuth scopes granted
        - "<scope>"
      oauth_policies:
        ip_relaxation: "<WhitelistIpRanges | RelaxIpRanges | EnforceIpRanges>"
        refresh_token_policy: "<ZeroOrMore | OneToken | ...>"
        single_logout_url: "<url_or_null>"
        callback_urls:
          - "<callback_url>"
      admin_approved_users_only: <boolean>
      permitted_users: "<All users may self-authorize | Admin approved users are pre-authorized>"
      audit_envelope:
        # Full audit envelope from fetcher skill

  - skill_id: salesforce-certificate-lifecycle-agent
    rationale: "Connected App certificate thumbprint requires lifecycle and expiry review."
    required_handoff_fields:
      certificate_thumbprint: "<sha256_thumbprint_or_null>"
      certificate_subject: "<subject_dn_or_null>"
      certificate_expiry: "<ISO8601_date_or_null>"
      uses_jwt_bearer: <boolean>
      audit_envelope:
        # Full audit envelope from fetcher skill
```

**Stop condition:** If the Connected App has `oauth_scopes` containing `full` or `web` and
`ip_relaxation: RelaxIpRanges`, declare this as a Critical finding in the handoff and recommend
immediate review by `salesforce-permission-model-review-skill` as well.

---

## Multi-Type Retrieval Routing

When a single fetch session retrieves multiple metadata types, declare all applicable routes
in the `downstream_skill_recommendation` array:

```yaml
downstream_skill_recommendation:
  - skill_id: salesforce-metadata-review-skill
    metadata_types_covered: ["CustomObject", "CustomField"]
    required_handoff_fields:
      # ... ObjectDescribe handoff fields
  - skill_id: salesforce-flow-automation-review-skill
    metadata_types_covered: ["Flow"]
    required_handoff_fields:
      # ... FlowMetadata handoff fields
```

**Escalation to `salesforce-org-assessment-skill`:** If a single session retrieves > 3 metadata
types and findings span multiple review skills, recommend routing to `salesforce-org-assessment-skill`
for consolidated posture review rather than fragmenting across individual downstream skills.

---

## Handoff Anti-Patterns

| Anti-pattern | Problem | Correct approach |
|---|---|---|
| Sending raw XML to downstream skill | XML may contain un-redacted IDs; downstream skill may not sanitize | Always convert to sanitized YAML/JSON first |
| Routing all types to `salesforce-metadata-review-skill` | That skill is for object/field schema only; it cannot review Apex or Flows | Use the specific routing table above |
| Omitting the audit envelope from handoff | Downstream skill cannot verify provenance or sanitization status | Always include the full audit envelope |
| Routing ConnectedApp to only one downstream skill | Certificate and integration concerns require separate expert review | Always invoke both skills for ConnectedApp |
| Declaring handoff before sanitization is complete | Risk of propagating un-redacted values | Sanitization must complete before handoff declaration |
