# Safety checklist — SAP Integration Suite Review

Use before making any remediation recommendation, especially for findings that affect iFlow deployment, API proxy policies, or Event Mesh consumer access.

## Non-negotiables

- Do not access, connect to, or request access to any live Integration Suite tenant, Cloud Integration API, API Management admin portal, or Event Mesh namespace. This skill reviews artifacts only.
- Do not accept or request tenant OAuth tokens, service keys, credential store values, keystore passwords, API Management admin credentials, or Event Mesh management credentials.
- Do not recommend deploying or modifying iFlows or API proxies in a production tenant based on review alone. All recommendations must be tested in a development or QA tenant first.
- Do not classify a finding as `critical` without a traceable path to the specific security exposure or data loss scenario from user-provided artifacts or official documentation.
- Do not recommend removing exception subprocess error suppression without first confirming how the failed message will be handled (dead-letter queue, alert, or manual reprocess).
- Do not treat synchronous and asynchronous integration patterns interchangeably. Idempotency recommendations differ between synchronous request-reply and asynchronous JMS/event-based patterns.
- Do not assert that an OData V4 or OpenAPI spec mismatch exists without user-provided spec content to compare against. Spec mismatch findings require both the declared spec and the deployed implementation for assessment.

## What people get wrong

- **Assuming all retry = idempotency needed**: Simple synchronous adapter retries (HTTP retry on connection failure before any processing) do not create duplicate-processing risk. Only retries that re-submit a message that may have already been partially processed (JMS, AS2, polling) require idempotency controls.
- **Recommending API Management policies for Cloud Integration adapter security**: API Management policies apply only to API proxy flows. Adapter-level security in Cloud Integration (e.g., HTTPS sender adapter with client certificate) is configured separately in the iFlow adapter parameters, not in API Management.
- **Conflating SAP Advanced Event Mesh with the legacy SAP Event Mesh service**: SAP Advanced Event Mesh (AEM) and the SAP Event Mesh service (BTP) have different topic hierarchy conventions, access control models, and monitoring tools. Confirm which product is in scope before applying documentation.
- **Missing the Secure Parameter store syntax**: Referencing credentials in iFlow properties uses `{{secparam.alias}}` syntax. If the user's iFlow uses a hardcoded value in the property field rather than this reference syntax, it is a `critical` plaintext credential finding regardless of whether the property is visible in the UI.
- **Ignoring dead-letter behavior on unbounded retry**: An iFlow with JMS retry configured to `Max. Retry Count: -1` (unlimited) and no dead-letter destination will retry poison messages indefinitely, consuming broker resources and masking other message failures.
- **Treating context7-supplementary CAP OData guidance as Cloud Integration guidance**: Context7 CAP documentation describes OData service exposure from CAP applications — it does not describe how Cloud Integration OData sender or receiver adapters work. Apply Context7 guidance only when a CAP-based service is the upstream or downstream partner in the integration.

## When to push back

- Push back when the user asks to confirm an iFlow security finding from a description alone without providing the iFlow export or configuration details.
- Push back when the user asks to recommend specific retry counts or timeout values without providing message volume and SLA data.
- Push back when the user asks to modify a production iFlow or API proxy directly — redirect to test tenant first.
- Push back when a request requires live tenant API access, iFlow deployment, or API proxy deployment — state clearly that live mutation is out of scope.

## Evidence labels

- `documentation-based` — grounded in SAP Cloud Integration, API Management, or Event Mesh Help Portal documentation
- `user-provided evidence` — iFlow exports, API proxy descriptors, Event Mesh config exports, monitoring screenshots, or descriptions provided by the user
- `context7-supplementary` — CAP OData service or event handling patterns from Context7 (supplementary; applies only when a CAP-based upstream or downstream service is in scope)
- `inference` — derived reasoning not directly confirmed by official docs or user evidence; must always be labeled as such
