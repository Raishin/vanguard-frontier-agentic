# Safety checklist — SAP Integration / Platform / Business Operations Protocol

Use before finalizing any finding, handoff package, or escalation trigger. This checklist is mandatory for all advisory sessions involving integration flow changes, credential rotations, or business-process outage declarations.

## Non-negotiables

- Do not accept or process OAuth tokens, Basic Authentication passwords, client secrets, API keys, or certificate private keys present in adapter configurations or trace logs. Refuse immediately and request redaction before proceeding.
- Do not recommend undeploying an active integration flow without first confirming (with the user) the business impact of stopping in-flight message processing. An undeploy during active transaction processing causes message loss.
- Do not invoke or recommend invoking `sap-integration-flow-guarded-operator-agent` from within this protocol. The operator gate is human-mediated only. This protocol produces the package; a human submits it.
- Do not recommend rotating adapter credentials without first confirming that the replacement credentials have been provisioned and tested. Rotating to invalid credentials immediately breaks all channels using those credentials.
- Do not classify a finding as `critical` without tracing the specific failure to user-provided evidence (log excerpt, alert export, incident record). Severity inflation without evidence is not permitted.
- Do not recommend restarting the SAP Integration Suite runtime node without first confirming with the user that they have the appropriate support access level to perform a restart and that the restart will not violate a maintenance window restriction.
- Do not produce a root cause hypothesis at `confirmed` confidence without user-provided evidence directly linking the hypothesis to the observed failure. `probable` or `possible` must be used when the evidence is indirect.

## What people get wrong

- **Conflating message processing errors with infrastructure failures**: A 503 response from a receiver system is a receiver-side issue, not an Integration Suite infrastructure failure. A message processing log showing a mapping exception is a flow design issue, not a platform issue. Classify correctly before recommending remediation.
- **Assuming retry exhaustion means message loss**: Retry exhaustion moves the message to an error state in the message processing log. It does not delete the message. Messages in error state can often be retried manually via the Operations view. Confirm whether manual retry is available before recommending more invasive remediation.
- **Conflating Cloud ALM health alerts with iFlow-level errors**: Cloud ALM health monitoring operates at the service level; iFlow errors are visible at the message processing log level. A Cloud ALM green health status does not mean individual messages are processing correctly.
- **Recommending credential rotation without checking all channels using those credentials**: A single OAuth client or Basic Authentication credential may be used by multiple iFlows or adapters across different packages. Rotating the credential without identifying all dependent channels breaks every one of them simultaneously.
- **Treating Event Mesh dead-letter queue accumulation as benign**: Messages in the dead-letter queue have exhausted all delivery retries. Without active reprocessing or discard, they consume storage and may indicate a subscription consumer failure that will grow indefinitely. Dead-letter queue accumulation is never a low-severity finding when the queue is growing.
- **Missing the on-premise Integration Agent in middleware instability triage**: SAP Integration Suite Cloud Integration can connect to on-premise systems via the SAP Connectivity service and Cloud Connector. Middleware instability may originate in the on-premise Cloud Connector or subaccount-level Connectivity service configuration, not in the cloud-side Integration Suite tenant.

## When to push back

- Push back when the user asks for a root cause determination without providing any message processing log excerpt, error message, or Cloud ALM alert data. Root cause analysis without evidence must be labeled `unknown` confidence.
- Push back when the user asks to directly execute an integration flow change, adapter credential rotation, or Event Mesh subscription change from within this protocol. This protocol is advisory only; redirect the user to the operator gate with the completed handoff package.
- Push back when the user provides adapter configurations or trace logs containing unredacted credentials. Request redaction before accepting the evidence.
- Push back when the user asks to declare a business-process outage resolved without the Business Process Owner confirming resolution. This protocol confirms integration health signals; the business declares resolution.
- Push back when a request requires live access to the SAP Integration Suite Operations view, Cloud ALM dashboard, or Event Mesh management console — this protocol accepts only user-provided artifacts. State clearly that live system access is out of scope.

## Evidence labels

- `documentation-based` — grounded in SAP Integration Suite documentation, SAP Cloud ALM documentation, SAP Event Mesh documentation, or SAP BTP Help Portal references
- `user-provided evidence` — message processing log excerpts, Cloud ALM alert exports, adapter configuration descriptions, incident timelines, or monitoring screenshots supplied by the user
- `inference` — derived reasoning not directly confirmed by official documentation or user evidence; always label explicitly and note the assumption being made
