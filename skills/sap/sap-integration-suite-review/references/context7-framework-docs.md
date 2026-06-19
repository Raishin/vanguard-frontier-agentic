# Context7 framework docs — SAP Integration Suite Review

**Role**: supplementary. Official SAP Integration Suite documentation (SAP Help Portal: Cloud Integration, API Management, Event Mesh) is the primary source for all integration review guidance. Context7-sourced CAP documentation supplements guidance specifically for OData service exposure and event handling patterns when a CAP-based application is an upstream or downstream integration partner.

**Library used**: SAP Cloud Application Programming Model (CAP)
Context7 library ID: `/websites/cap_cloud_sap`
Lookup target: OData service exposure, OpenAPI integration, event handling, error handling in CAP services consumed via Integration Suite
Skill: `sap-integration-suite-review`
Classification: supplementary

---

## CAP OData service exposure (supplementary)

Source: cap.cloud.sap (Context7 `/websites/cap_cloud_sap`)

CAP exposes services as OData V4 endpoints by default. When a CAP service is consumed by a Cloud Integration iFlow or proxied by an API Management proxy, the OData metadata contract is defined by the CDS service definition. CAP-generated OData V4 services use OASIS OData V4 metadata conventions. Mismatches between the CDS service definition and the API Management API spec are a common source of `odata-spec-mismatch` findings.

Example of exposing a remote OData entity as a CAP projection (relevant when a CAP application re-exposes an SAP S/4HANA OData service via Integration Suite):

```cds
using { API_BUSINESS_PARTNER as bupa } from '../srv/external/API_BUSINESS_PARTNER';

extend service RiskService with {
  entity BusinessPartners as projection on bupa.A_BusinessPartner;
}
```

**Relevance to Integration Suite review**: When an API Management proxy fronts a CAP OData V4 service, the proxy spec must match the CDS-generated metadata. Discrepancies between the CDS model and the API spec file registered in API Management are flagged as `odata-spec-mismatch` findings.

## CAP event handling and error responses (supplementary)

Source: cap.cloud.sap (Context7 `/websites/cap_cloud_sap`)

CAP event handlers that throw exceptions abort the current event's processing and roll back any active transaction. The OData V4 protocol adapter converts exceptions into OData error responses with HTTP error codes. When a Cloud Integration iFlow calls a CAP OData endpoint and receives an error response, the iFlow exception subprocess must handle OData-format error payloads.

Example OData error response from a CAP service:

```xml
<error xmlns="https://docs.oasis-open.org/odata/ns/metadata">
  <code>500</code>
  <message>Processing failed: no such entity</message>
</error>
```

**Relevance**: iFlow exception subprocesses that call CAP OData endpoints must parse OData-format error responses (XML or JSON, depending on Content-Type negotiation). Generic HTTP error handling that assumes plain-text error bodies will silently miss error detail from CAP services. This supplements the Cloud Integration exception subprocess guidance from official SAP docs.

## CAP event declaration for S/4HANA integration (supplementary)

Source: cap.cloud.sap (Context7 `/websites/cap_cloud_sap`)

When a CAP application consumes SAP Event Mesh events from S/4HANA, events that are not present in the imported API spec must be declared explicitly in CDS:

```cds
using { API_BUSINESS_PARTNER as S4 } from './API_BUSINESS_PARTNER';
extend service S4 with {
  event BusinessPartner.Created @(topic:'sap.s4.beh.businesspartner.v1.BusinessPartner.Created.v1') {
    BusinessPartner : String
  }
}
```

**Relevance**: When reviewing an Integration Suite topology where a CAP application subscribes to Event Mesh topics from S/4HANA, the topic name declared in the CAP event definition must match the Event Mesh topic namespace configured in the Event Mesh service instance. Mismatches cause silent event delivery failures. This supplements the Event Mesh topic design guidance from official SAP docs.

---

## Scope boundaries for Context7 usage

Context7 CAP documentation applies **only** when a CAP-based application is the upstream or downstream integration partner in the scope of an Integration Suite review. It does not apply to:

- Native Cloud Integration adapter behavior (HTTP, SOAP, IDoc, AS2, JMS) — use SAP Help Portal Cloud Integration docs
- API Management policy language (JavaScript, ExtractVariables, OAuthV2) — use SAP Help Portal API Management docs
- SAP Advanced Event Mesh broker configuration — use SAP Help Portal Event Mesh docs
- Security material management (keystore, credential store) — use SAP Help Portal Cloud Integration security docs

Always label Context7-sourced guidance as `context7-supplementary` in responses.
