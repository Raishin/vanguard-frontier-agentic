# Action Safety Matrix Reference

Classification of Agentforce <!-- verify-before-merge:2026-05-21 --> actions by autonomy risk tier,
required safeguards, and human-confirmation requirements.

---

## Risk Tier Definitions

| Tier | Label | Description |
|------|-------|-------------|
| 0 | Safe/Read-Only | Read operations, status lookups, informational responses |
| 1 | Low-Risk Write | Creates non-sensitive records, sends non-regulated notifications |
| 2 | Medium-Risk Write | Modifies existing records, updates financial or status fields |
| 3 | High-Risk Write | Deletes records, transfers ownership, submits external transactions |
| 4 | Critical/Irreversible | Data export, regulatory filings, payment processing |

---

## Action Category Matrix

### CRM Record Operations

| Action | Tier | Autonomous OK | Confirmation Required | Human Handoff Required |
|--------|------|--------------|----------------------|----------------------|
| Query (SOQL read) | 0 | Yes | No | No |
| Create Case | 1 | Yes | Recommended | No |
| Create Contact | 1 | Yes | Recommended | No |
| Update Case Status | 2 | Conditional | If closing | No |
| Update Opportunity Stage | 2 | No | Yes | Recommended |
| Update Account billing info | 3 | No | Yes | Yes |
| Delete record (any) | 3 | No | Yes | Yes |
| Transfer Case ownership | 2 | Yes | If cross-team | No |
| Mass update (>50 records) | 3 | No | Yes | Yes |
| Merge records | 3 | No | Yes | Yes |

### Communication Actions

| Action | Tier | Autonomous OK | Confirmation Required | Human Handoff Required |
|--------|------|--------------|----------------------|----------------------|
| Send Email (transactional) | 1 | Yes | No | No |
| Send Email (marketing) | 2 | No | Yes | No |
| Send SMS <!-- verify-before-merge:2026-05-21 --> | 1 | Yes | No | No |
| Log a call | 0 | Yes | No | No |
| Post to Chatter (internal) | 1 | Yes | No | No |
| Publish to Community | 2 | No | Yes | No |

### External Callout Actions

| Action | Tier | Autonomous OK | Confirmation Required | Human Handoff Required |
|--------|------|--------------|----------------------|----------------------|
| GET to external API | 0 | Yes | No | No |
| POST to external API (idempotent) | 1 | Yes | No | No |
| POST to external API (non-idempotent) | 2 | No | Yes | No |
| Payment/financial API call | 4 | No | Yes | Yes |
| External identity verification | 3 | No | Yes | Yes |

### Data and Privacy Actions

| Action | Tier | Autonomous OK | Confirmation Required | Human Handoff Required |
|--------|------|--------------|----------------------|----------------------|
| Return own-account data to authenticated user | 1 | Yes | No | No |
| Return third-party data in response | 3 | No | Yes | Yes |
| Export record data to file/link | 4 | No | Yes | Yes |
| Process data deletion (DSAR) | 4 | No | Yes | Yes |
| Update consent preferences | 3 | No | Yes | Recommended |

### Flow and Automation Trigger Actions

| Action | Tier | Autonomous OK | Confirmation Required | Human Handoff Required |
|--------|------|--------------|----------------------|----------------------|
| Launch Screen Flow | 0 | Yes | No | No |
| Launch Autolaunched Flow (read) | 1 | Yes | No | No |
| Launch Autolaunched Flow (write) | 2 | Conditional | If write path taken | No |
| Submit Approval Process | 2 | No | Yes | No |
| Recall Approval | 3 | No | Yes | Yes |
| Run Scheduled Apex | 3 | No | Yes | No |

---

## Safeguard Requirements by Tier

### Tier 0: No additional safeguards required.

### Tier 1: Logging required.
- Record agent conversation ID in the created/updated record's audit field or
  a custom `AgentActionLog__c` object.
- Confirm no PII is returned in the response payload beyond what the user is
  authorized to view.

### Tier 2: Confirmation dialog required.
```
Before executing: present a structured confirmation message to the user:
  "I am about to [action summary]. Please confirm with 'Yes' to proceed or
   'No' to cancel."
Log: AgentConversationId, ActionName, RecordId, Timestamp, UserConfirmation
```

### Tier 3: Human handoff or manager approval required.
- Trigger a Transfer to Agent action to a qualified human queue OR
- Invoke an Approval Process and wait for asynchronous approval before acting.
- Never execute Tier-3 actions inline without out-of-band approval.

### Tier 4: Full audit trail and dual authorization required.
- Agent must NOT execute autonomously.
- Action must be queued for human review with full audit record.
- Dual authorization (two named humans) required for financial and data export actions.
- Retain audit record for minimum 7 years (regulatory default; adjust per vertical).

---

## Autonomous vs Human-Confirmed: Decision Flowchart

```
Is the action read-only?
  YES -> Tier 0. No confirmation needed.
  NO ->
    Does the action affect > 1 record or > 1 object?
      YES -> Tier 2 minimum. Confirmation required.
      NO ->
        Does it affect financial, legal, or regulated data?
          YES -> Tier 3 or 4. Human handoff required.
          NO ->
            Does it trigger an external system?
              YES (non-idempotent) -> Tier 2 minimum.
              NO -> Tier 1. Logging required.
```

---

## Reference: Agentforce Action Types <!-- verify-before-merge:2026-05-21 -->

| Action Type | Metadata API Name | Notes |
|-------------|------------------|-------|
| Apex Action | `ApexAction` | Full Apex governor limits apply |
| Flow Action | `FlowAction` | Respects running user's sharing |
| Prompt Action | `PromptAction` | No DML; generates text response |
| External Service Action | `ExternalServiceAction` | Requires Named Credential |
| Standard Agent Action | `StandardAction` | Salesforce-provided, reviewed by Salesforce |

---

## Audit Log Fields to Capture for Every Action

```
AgentActionLog__c {
  ConversationId__c    : String(255)   // Agentforce session ID
  ActionName__c        : String(255)   // Exact action metadata name
  ActionTier__c        : Picklist      // 0/1/2/3/4
  ExecutedAt__c        : DateTime
  RunningUserId__c     : Lookup(User)
  TargetRecordId__c    : String(18)    // If a record was affected
  ConfirmedByUser__c   : Boolean
  HandoffTriggered__c  : Boolean
  OutcomeStatus__c     : Picklist      // Success/Failure/Cancelled
  ErrorMessage__c      : LongTextArea  // If failure
}
```
