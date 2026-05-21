# Hyperforce Deployment Controls Reference

Reference for Salesforce Hyperforce
deployment controls covering
region selection, data residency commitments, and Infrastructure Access boundary
management.

---

## What Hyperforce Is

Hyperforce is Salesforce's public cloud-based infrastructure built on top of
major cloud providers (AWS, Azure, GCP, Alibaba Cloud — varies by region).

Hyperforce is distinct from legacy Salesforce infrastructure ("Classic") in
that:
- Customer data is hosted within defined cloud regions with contractual data
  residency boundaries.
- Compute and storage are on public cloud infrastructure with Salesforce's
  security controls applied on top.
- Upgrades and scaling occur at the public cloud layer.

---

## Region Selection

### Available Hyperforce Regions

Region availability changes as Hyperforce expands.
Verify current available regions with Salesforce documentation or account team.

As of the last validated date, Hyperforce regions include (not exhaustive):

| Region | Cloud Provider | Data Residency Boundary |
|--------|---------------|------------------------|
| US East | AWS | United States |
| US West | AWS | United States |
| EU (Frankfurt) | AWS | European Union (Germany) |
| EU (London) | AWS | United Kingdom |
| APAC (Tokyo) | AWS | Japan |
| APAC (Singapore) | AWS | Singapore/ASEAN |
| India | AWS | India |
| Australia | AWS | Australia |

### Region Selection Criteria

1. **Regulatory requirement:** GDPR requires EU personal data to remain in EEA
   or countries with adequacy decisions. Select an EU region for EU-resident data.
2. **Data sovereignty:** Government and financial regulators in some countries
   require in-country data storage. Verify with legal counsel.
3. **Latency:** Select the region closest to the majority of end users.
4. **Availability:** Not all Salesforce products are available in all Hyperforce
   regions on the same timeline.

### How to Identify Your Org's Hyperforce Region

```bash
# Check the org's instance name and location
sf org display -o your-org-alias --json | jq '.result.instanceUrl'
# e.g., https://mycompany.my.salesforce.com

# Cross-reference instance name with Salesforce Trust status page instance list
# to identify the hosting region
```

Salesforce Trust (trust.salesforce.com) lists all instances with region labels.
Hyperforce instances are typically labeled with their cloud region
(e.g., `CS102` for a US instance, `EU64` for an EU instance).

---

## Data Residency Controls

### What Hyperforce Guarantees

Hyperforce provides:
- Data-at-rest encrypted and stored within the contracted region.
- Metadata (configuration, schema) may also be region-bound depending on the
  product and contract.
- Salesforce support access controls via the Customer Trust Access Management
  feature.

### What Hyperforce Does NOT Guarantee by Default

- Prevention of data flowing to Salesforce support systems outside the region
  during incident investigation (unless Customer Trust Access Management is enabled).
- Restricting CDN edge nodes to a specific region (traffic routing optimizations
  may traverse geographic boundaries at the network layer).

### Verifying Data Residency via Contract

Data residency is a contractual commitment, not purely a technical one. Review:
- Order Form for the "Data Residency Option" or "Hyperforce region" specification.
- Data Processing Addendum (DPA) for region binding commitments.
- Business Associate Agreement (BAA) if HIPAA-regulated data is processed.

---

## Infrastructure Access Controls

### What Infrastructure Access Means

Infrastructure Access refers to whether Salesforce support engineers and
infrastructure teams can access customer org data for troubleshooting.

By default, Salesforce support has time-limited access to org data for support
purposes. This is detailed in the Salesforce Privacy and Security Documentation.

### Customer Trust Access Management

Hyperforce customers can enable Customer Trust Access Management to require
explicit customer approval before Salesforce support personnel access production
org data.

Controls available (subject to contract and product tier):
- Require customer approval for all Salesforce support access to production data.
- Access requests expire after a defined time window (e.g., 4 hours).
- Access events are logged and visible to the customer.

To review access logs (if enabled):
```sql
SELECT Id, Action, ActorName, ActorType, EventDate, Summary
FROM SetupAuditTrail
WHERE Action LIKE '%Access%' OR Action LIKE '%Support%'
ORDER BY EventDate DESC
LIMIT 200
```

### Admin Lockout Controls

Hyperforce includes the ability to restrict Salesforce admin-level access to
specific named individuals. This is part of the Enterprise Key Management and
Infrastructure Access offering.

---

## Shield Encryption and Key Management

Salesforce Shield Platform Encryption
provides encryption
at-rest for selected fields and files. On Hyperforce, encryption key management
options include:

| Option | Description | Key Custody |
|--------|-------------|------------|
| Salesforce-managed keys | Default; Salesforce manages key lifecycle | Salesforce |
| Customer-managed keys (Bring Your Own Key) | Customer uploads and rotates keys | Customer |
| External Key Management (EKM) | Keys stored in customer's external HSM or KMS | Customer HSM/KMS |

### Key Management Audit Points

- [ ] Verify encryption tenant secret rotation schedule (recommended: 90 days).
- [ ] Confirm key derivation history shows at least one manual rotation in the
  last 12 months.
- [ ] If using BYOK, confirm the master HSM/KMS is geographically co-located
  with the Hyperforce region.
- [ ] Verify Shield encryption covers all regulated field types (PII, PHI, financial).

```sql
// Query encrypted field configuration (requires Shield)
SELECT EntityDefinition.QualifiedApiName, QualifiedApiName, Label,
       IsEncrypted
FROM FieldDefinition
WHERE IsEncrypted = true
ORDER BY EntityDefinition.QualifiedApiName, QualifiedApiName
```

---

## Hyperforce Deployment Readiness Checklist

- [ ] Org instance confirmed as Hyperforce (not legacy Classic infrastructure).
- [ ] Hyperforce region documented and matches regulatory data residency requirement.
- [ ] Contract includes Data Residency Option for the required region.
- [ ] Data Processing Addendum (DPA) executed with correct region binding.
- [ ] Customer Trust Access Management evaluated and configured if required.
- [ ] Salesforce Shield Platform Encryption coverage reviewed for regulated fields.
- [ ] Encryption key rotation schedule documented and tested.
- [ ] BYOK/EKM configured if customer-controlled keys are contractually required.
- [ ] Salesforce Trust status subscriptions configured for the org's specific instance.
- [ ] Incident response plan includes Hyperforce region-specific escalation contacts.
