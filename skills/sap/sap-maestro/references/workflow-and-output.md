# Workflow and output contract — SAP Maestro

Use this reference for all routing, classification, and dispatch decisions.

## Domain taxonomy

The following top-level SAP domain buckets are used for classification:

| Domain | Description | Example Products |
|--------|-------------|-----------------|
| `btp-platform` | SAP BTP platform services (not application-layer) | BTP Cockpit, Cloud Foundry, Kyma, BTP CLI |
| `btp-integration` | Integration Suite and hybrid integration | SAP Integration Suite, API Management, Event Mesh |
| `btp-ai` | AI and ML services on BTP | SAP AI Core, AI Launchpad, GenAI Hub, HANA ML |
| `btp-data` | Data and analytics on BTP | SAP Datasphere, SAP Analytics Cloud, HANA Cloud |
| `s4hana-core` | S/4HANA core ERP logic | Finance, Procurement, Manufacturing modules |
| `s4hana-extensibility` | S/4HANA custom code, clean core, and extensibility | ABAP Cloud, RAP, BAdIs, key-user apps |
| `s4hana-transport` | S/4HANA system landscape and transport management | TMS, CTS+, STMS, landscape definitions |
| `successfactors` | SAP SuccessFactors HCM | Employee Central, Recruiting, Learning |
| `ariba` | SAP Ariba procurement | Sourcing, Contracts, Buying |
| `grc` | Governance, Risk, and Compliance | SAP GRC Access Control, Process Control |
| `identity` | Identity and access management | SAP IAS, SAP IPS, Identity Provisioning |
| `cross-domain` | Spans two or more of the above | Integration + S/4 + BTP in one request |

## Routing table

| Domain | Matched skill or agent | Dispatch mode | Live tier |
|--------|----------------------|---------------|-----------|
| `btp-platform` | `sap-live-readonly-landscape-discovery` | gated (read-only) | read-only-runtime |
| `btp-integration` | `sap-live-readonly-landscape-discovery` | gated (read-only) | read-only-runtime |
| `s4hana-extensibility` | `sap-clean-core-debt-review` | direct | none |
| `s4hana-transport` | `sap-guarded-transport-import` | gated (mutating) | mutating-runtime |
| `cross-domain` | split into domain-scoped sub-requests | split | varies |
| `btp-ai`, `btp-data`, `successfactors`, `ariba`, `grc`, `identity` | `unrouted` — no matching skill declared | unrouted | N/A |

## Dispatch modes

- **direct**: Load the named skill immediately. No live-guard gate required (static advisory skill).
- **gated (read-only)**: Confirm with the user that read-only live access is authorized and credentials are in scope before loading.
- **gated (mutating)**: Full live-guard sequence applies. Do not dispatch without explicit step-by-step approval per `sap-guarded-transport-import` protocol.
- **split**: Decompose the request into domain-scoped sub-requests. Route each sub-request independently through the table above.
- **unrouted**: No declared skill covers this domain. Return the classification, explain the gap, and suggest the user consult SAP official documentation or engage an SAP partner.

## Live-guard gate

Before dispatching to any skill with `live tier` of `read-only-runtime` or `mutating-runtime`:

1. Confirm the user has authorized live-system access in this session.
2. Confirm the user understands which live system will be accessed.
3. Confirm the downstream skill's live-environment rules have been reviewed.
4. For `mutating-runtime`: confirm the 17-step guarded mutation sequence in `sap-guarded-transport-import` is understood and approved.

Do not skip the live-guard gate. Do not propose live dispatch based on classification alone.

## Workflow

1. **Receive and parse** the incoming request.
2. **Classify** into one or more domain taxonomy buckets.
3. **Look up** the routing table. If `cross-domain`, decompose.
4. **Select dispatch mode** per the table.
5. **Apply live-guard gate** if dispatch mode is `gated`.
6. **Return output** per the output contract below.

## Output contract

Return:

1. Domain classification (bucket name + rationale)
2. Evidence label (documentation-based / catalog-evidence / user-provided / inference)
3. Routing table entry that fired
4. Dispatch mode selected
5. Live-guard gate status (not applicable / pending / cleared)
6. Recommended next action (load skill / split request / escalate / unrouted explanation)
7. Refusal trigger if any live-guard condition is unmet
