# NIST ZTA Pillars Reference

NIST SP 800-207 Zero Trust Architecture pillars mapped to Salesforce controls,
configurations, and implementation guidance.

<!-- verify-before-merge:2026-05-21 --> NIST SP 800-207 was published August 2020.
Verify against any superseding NIST publications.

---

## Zero Trust Core Tenets (NIST SP 800-207)

1. All data sources and computing services are considered resources.
2. All communication is secured regardless of network location.
3. Access to individual enterprise resources is granted on a per-session basis.
4. Access is determined by dynamic policy including client identity, application,
   and observable state.
5. The enterprise monitors and measures the integrity and security posture
   of all owned and associated assets.
6. All resource authentication and authorization is dynamic and strictly enforced.
7. The enterprise collects information about assets, network infrastructure,
   and communications to improve security posture.

---

## Pillar 1: Identity

**Core question:** Is this user who they claim to be, and are they authorized
for this specific request at this moment?

### Salesforce Controls for Identity Pillar

| Control | Salesforce Feature | Maturity Level |
|---------|------------------|----------------|
| MFA enforcement | Setup > Identity > MFA | Required (Level 1) |
| Single Sign-On (SSO) | Auth. Provider + My Domain | Level 2 |
| Adaptive authentication | High Assurance sessions | Level 2 |
| Continuous session validation | Session timeout + IP lock | Level 2 |
| Privileged access management | Named admin accounts + IP restrictions | Level 3 |
| Just-in-time (JIT) provisioning | SSO JIT SAML provisioning | Level 3 |
| Identity governance | Quarterly permission reviews | Level 3 |

### Implementation Reference

```
My Domain:  Required prerequisite for SSO and Modern Auth.
Path: Setup > Company Settings > My Domain

Auth. Provider (OIDC/SAML):
Path: Setup > Identity > Auth. Providers

MFA:
Path: Setup > Identity > Identity Verification
Setting: Multi-Factor Authentication for User Interface Logins = Required

Session Security Level for High Assurance:
Path: Setup > Security > Session Settings > Session Security Levels
Operations requiring High Assurance: Manage Users, Connected Apps, Certificates
```

---

## Pillar 2: Device

**Core question:** Is the device used to access Salesforce healthy and trusted?

### Salesforce Controls for Device Pillar

| Control | Salesforce Feature | Maturity Level |
|---------|------------------|----------------|
| Trusted IP ranges | Network Access + Profile Login IPs | Level 1 |
| Device posture enforcement (MDM) | Salesforce Authenticator + MDM integration | Level 2 |
| Certificate-based device auth | Client certificates in Connected Apps | Level 3 |
| Mobile Device Management | Salesforce Mobile App MDM policies | Level 2 |

### Note on Salesforce's Position

Salesforce is a SaaS platform — direct device health attestation (TPM, secure
boot validation) must be implemented at the identity provider layer (your IdP /
MDM solution). Salesforce receives the result of device trust evaluation via
SSO claims or network-level controls.

```
Example pattern:
1. MDM marks device as compliant.
2. IdP (Okta/Azure AD/Ping) receives device compliance signal from MDM.
3. IdP issues SAML assertion with device-compliance claim.
4. Salesforce Auth. Provider validates assertion.
5. If device non-compliant: Auth. Provider denies session or downgrades
   to Standard session (blocking High Assurance operations).
```

---

## Pillar 3: Network

**Core question:** Is network traffic protected in transit and origin-verified?

### Salesforce Controls for Network Pillar

| Control | Salesforce Feature | Maturity Level |
|---------|------------------|----------------|
| TLS enforcement | Salesforce enforces TLS 1.2+ by default | Level 1 |
| IP allowlists | Trusted IP Ranges + Profile Login IP Ranges | Level 1 |
| HSTS | Salesforce enforces HSTS on all prod orgs | Level 1 |
| CSP enforcement | CSP Trusted Sites | Level 2 |
| Private Connect / PrivateLink | Salesforce Private Connect (Hyperforce) | Level 3 |
| Micro-segmentation | Named Credential per endpoint + IP restriction | Level 2 |

### Private Connect / AWS PrivateLink <!-- verify-before-merge:2026-05-21 -->

Salesforce Private Connect allows connections between Salesforce and AWS VPCs
(and other cloud resources) over AWS PrivateLink — traffic never traverses the
public internet.

```
Path: Setup > Integrations > Private Connect
Requirements:
  - Salesforce on Hyperforce (AWS)
  - AWS VPC with PrivateLink endpoint configured
  - Matching region between Salesforce instance and AWS VPC
```

---

## Pillar 4: Application

**Core question:** Is the user authorized for this specific application action?

### Salesforce Controls for Application Pillar

| Control | Salesforce Feature | Maturity Level |
|---------|------------------|----------------|
| Object-level CRUD | Profile + Permission Set permissions | Level 1 |
| Field-Level Security | FLS configuration | Level 1 |
| Record-level sharing | OWD + Sharing Rules + Role Hierarchy | Level 2 |
| Apex FLS enforcement | `WITH SECURITY_ENFORCED` / `stripInaccessible` | Level 2 |
| High Assurance for sensitive ops | Session Security Levels | Level 3 |
| Agentforce action safety tiers | Action confirmation + human handoff | Level 3 |

### Per-Request Authorization Check

In a mature ZTA Salesforce implementation, every API request should be
authorized on the following dimensions:
1. User identity (valid session, MFA verified)
2. User permission (CRUD, FLS on requested object/field)
3. Record access (sharing model permits access to this specific record)
4. Action safety (is this action in the user's permitted action tier)

---

## Pillar 5: Data

**Core question:** Is data protected at rest, in transit, and in use?

### Salesforce Controls for Data Pillar

| Control | Salesforce Feature | Maturity Level |
|---------|------------------|----------------|
| Data classification | Custom field: DataClassification__c | Level 1 |
| Encryption at rest | Shield Platform Encryption | Level 2 |
| Customer-managed keys | Bring Your Own Key (BYOK) / EKM | Level 3 |
| Data masking in non-prod | Sandbox Data Masking | Level 2 |
| Data Loss Prevention | DLP via Event Monitoring + CASB | Level 3 |
| Data residency | Hyperforce region selection | Level 2 |

---

## Zero Trust Mapping to Salesforce Audit Trail

Every relevant access event in a ZTA posture should be captured and monitored.

| Event Type | Salesforce Source | ZTA Pillar |
|------------|------------------|-----------|
| Login events | LoginHistory | Identity |
| Failed login | LoginHistory (Status = Failed) | Identity |
| Permission change | SetupAuditTrail | Identity, Application |
| Record access | EventMonitoring: ReportEvent, ListViewEvent | Data |
| API access | EventMonitoring: ApiEvent | Identity, Network |
| Data export | EventMonitoring: ReportExport | Data |
| Sensitive field view | EventMonitoring: FieldHistoryTracking | Data |
| Admin configuration | SetupAuditTrail | Application |

```bash
# Query Login History for failed logins (Identity pillar monitoring)
sf data query \
  --query "SELECT UserId, LoginTime, SourceIp, Status, Browser \
           FROM LoginHistory \
           WHERE Status != 'Success' \
             AND LoginTime = LAST_N_DAYS:7 \
           ORDER BY LoginTime DESC \
           LIMIT 1000" \
  -o prod-alias
```
