# Workflow and output contract — SAP Integration Suite Review

Use this reference for all finding classification, risk assignment, remediation path selection, and output formatting.

## Integration issue taxonomy

| Component domain | Finding class | Description |
|-----------------|--------------|-------------|
| `cloud-integration` | `missing-exception-subprocess` | iFlow has no exception subprocess; errors silently discard messages |
| `cloud-integration` | `missing-idempotency` | Retry-capable iFlow (JMS, polling, webhook) has no duplicate check or idempotent receiver |
| `cloud-integration` | `plaintext-credential` | Credential (password, API key, client secret) hardcoded in iFlow property or mapping |
| `cloud-integration` | `missing-correlation-id` | iFlow does not propagate SAP-MessageId or business key across adapter boundaries |
| `cloud-integration` | `unbounded-retry` | JMS or adapter retry configured with no maximum retry count or dead-letter routing |
| `cloud-integration` | `missing-receiver-determination` | Dynamic routing iFlow has no default receiver path; unmatched messages are silently dropped |
| `api-management` | `missing-oauth-policy` | API proxy inbound step has no OAuth 2.0 or API key enforcement policy |
| `api-management` | `missing-rate-limit` | API proxy has no spike arrest or quota policy; susceptible to abuse or overload |
| `api-management` | `missing-threat-protection` | API proxy has no JSON/XML threat protection policy on inbound payloads |
| `api-management` | `basic-auth-inbound` | API proxy inbound step uses Basic Auth — not acceptable for production endpoints |
| `api-management` | `odata-spec-mismatch` | OData V4 service exposed via API Management does not match declared metadata or violates OData V4 conventions |
| `event-mesh` | `shared-service-key` | Multiple consumers share a single Event Mesh service key — violates isolation principle |
| `event-mesh` | `missing-dead-letter-queue` | Queue has no dead-letter destination; poison messages cause indefinite retry loops |
| `event-mesh` | `missing-consumer-group-isolation` | Consumer groups not isolated per application — one consumer can read another application's messages |
| `security` | `missing-certificate-rotation` | Adapter certificate-based auth uses certificates without documented rotation schedule |
| `monitoring` | `no-alert-rules` | Cloud Integration tenant has no configured alert rules for failed message processing |

## Risk severity classification

| Risk level | Criteria |
|-----------|---------|
| `critical` | Direct security exposure or data loss: no inbound OAuth/API key enforcement, plaintext credential in iFlow property |
| `high` | Operational reliability failure: missing exception subprocess, missing idempotency on retry-capable iFlow, unbounded retry with no dead-letter |
| `medium` | Governance or observability gap: missing correlation ID propagation, no monitoring alert rules, shared Event Mesh service key |
| `low` | Best practice deviation: inconsistent naming, undocumented adapter parameter choices, missing rate limiting on internal-only APIs |

## Remediation path decision tree

For each finding:

1. **Is this a missing inbound security policy (OAuth, API key)?**
   - Yes → `critical`. Add an OAuth V2.0 VerifyAccessToken policy as the first step in the API proxy PreFlow before any target invocation. Do not proceed with deployment without it.
   - No → continue.

2. **Is this a plaintext credential in iFlow configuration?**
   - Yes → `critical`. Move the credential to the Integration Suite Secure Parameter store (credential store). Reference via `{{secparam.credential-alias}}` syntax. Redeploy the iFlow.
   - No → continue.

3. **Is this a missing exception subprocess?**
   - Yes → `high`. Add an exception subprocess that at minimum: (a) logs the error context, (b) sends a failure notification or writes to an error queue, (c) does not silently swallow the exception.
   - No → continue.

4. **Is this a missing idempotency control on a retry-capable iFlow?**
   - Yes → `high`. Add an Idempotent Process Call step using the message's SAP-MessageId or a business key. Configure the idempotency repository (in-memory or persistence store depending on the retry window).
   - No → continue.

5. **Is this a governance or observability gap?**
   - Yes → `medium`. Implement correlation ID header propagation (map SAP-MessageId at iFlow entry), configure Cloud Integration alert rules for FAILED status, or isolate Event Mesh service keys per consumer application.
   - No → classify as `low` and provide best practice guidance.

## Workflow

1. **Receive artifacts** — iFlow export bundles, API proxy descriptors, Event Mesh config exports, or user descriptions.
2. **Classify each finding** by component domain and finding class.
3. **Assign risk level** (critical / high / medium / low).
4. **Apply remediation decision tree** per finding.
5. **Prioritize** — critical security findings first; then high operational reliability findings; then medium governance gaps; then low best-practice items.
6. **Return output** per the output contract below.

## Output contract

Return:

1. Component domain and specific finding class
2. Evidence label (documentation-based / user-provided evidence / context7-supplementary / inference)
3. Risk level per finding (critical / high / medium / low)
4. Recommended remediation action with specific implementation guidance
5. Integration posture after remediation
6. Prioritized remediation sequence
7. Escalation trigger if live Integration Suite tenant access is required to confirm or apply the finding
