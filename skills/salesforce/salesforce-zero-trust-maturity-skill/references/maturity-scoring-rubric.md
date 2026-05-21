# Maturity Scoring Rubric Reference

Scoring matrix for assessing Zero Trust maturity in Salesforce orgs across
five NIST SP 800-207 pillars with gap analysis template.

---

## Maturity Level Definitions

| Level | Label | Description |
|-------|-------|-------------|
| 0 | None | No controls in place for this area |
| 1 | Initial | Ad hoc controls; inconsistently applied |
| 2 | Developing | Defined controls; applied to most cases; manual processes |
| 3 | Established | Defined, documented, and consistently applied; some automation |
| 4 | Advanced | Automated, measured, and continuously improved |
| 5 | Optimized | Predictive and adaptive controls; industry-leading posture |

---

## Pillar 1: Identity Maturity

| Check | Level 0 | Level 1 | Level 2 | Level 3 | Level 4 | Level 5 |
|-------|---------|---------|---------|---------|---------|---------|
| MFA enforcement | No MFA | MFA optional | MFA required for some users | MFA required for all internal | MFA + risk-adaptive step-up | Biometric + hardware key everywhere |
| SSO integration | Username/password only | SSO available but not required | SSO required for most | SSO required for all; JIT provisioning | SSO with device signal | Continuous posture evaluation |
| Session management | Default (8h) timeout | Timeouts configured | IP-locked sessions | High Assurance for admin ops | Adaptive timeout by risk | Continuous session re-evaluation |
| Privileged access | Shared admin accounts | Named admin accounts | Named + IP restricted | Named + IP + audit alerts | Just-in-time privilege elevation | Full PAM with recorded sessions |
| Identity governance | No review process | Annual review | Quarterly review | Automated drift detection | Real-time anomaly alerts | ML-driven identity behavior baseline |

**Target minimum for production orgs: Level 3**

### Level 3 Requirements for Identity

- [ ] MFA required for all active internal users via org-wide setting.
- [ ] SSO configured; username/password login disabled for internal users (if possible).
- [ ] Admin accounts named, IP-restricted, and documented.
- [ ] High Assurance required for: Manage Users, Connected Apps, Certificates.
- [ ] Permission set assignments reviewed quarterly.
- [ ] Login History exported to SIEM.

---

## Pillar 2: Device Maturity

| Check | Level 0 | Level 1 | Level 2 | Level 3 | Level 4 | Level 5 |
|-------|---------|---------|---------|---------|---------|---------|
| Device trust | No controls | IP restrictions on profiles | MDM enrollment required | MDM compliance gates SSO | Device posture in IdP token | Continuous device health |
| Mobile access | Unrestricted | Mobile app configured | MDM PIN/encrypt required | App wrapping via MDM | Container isolation | Biometric device access |
| Network access | Any network | VPN recommended | VPN required for sensitive | VPN + IP restrictions | Zero Trust Network Access (ZTNA) | Software-defined perimeter |

**Target minimum for production orgs: Level 2**

---

## Pillar 3: Network Maturity

| Check | Level 0 | Level 1 | Level 2 | Level 3 | Level 4 | Level 5 |
|-------|---------|---------|---------|---------|---------|---------|
| Transport security | HTTP allowed | HTTPS enforced | TLS 1.2+ enforced | HSTS + pinning | Private Connect/PrivateLink | Full mTLS everywhere |
| Egress control | Open callouts | Remote Site Settings managed | Named Credentials only | Named Credentials + IP allowlist | Outbound proxy filtering | Micro-segmented egress |
| CSP | No CSP | Basic CSP Trusted Sites | CSP for all Salesforce pages | CSP + Report-URI monitoring | Real-time CSP violation alerting | Adaptive CSP |

**Target minimum for production orgs: Level 2-3**

### Level 3 Requirements for Network

- [ ] All callout endpoints use Named Credentials (no hardcoded URLs).
- [ ] All Remote Site Settings reviewed; none have DisableProtocolSecurity=true.
- [ ] CSP Trusted Sites contain only HTTPS origins; no wildcards.
- [ ] HSTS confirmed on custom domain.
- [ ] Private Connect evaluated for regulated data integrations.

---

## Pillar 4: Application Maturity

| Check | Level 0 | Level 1 | Level 2 | Level 3 | Level 4 | Level 5 |
|-------|---------|---------|---------|---------|---------|---------|
| Object CRUD | Default profiles | CRUD on profiles | CRUD via Permission Sets | Least-privilege Permission Sets | Automated access review | Dynamic permission grant |
| FLS enforcement | No FLS controls | FLS on standard objects | FLS on custom sensitive fields | FLS + Apex enforcement | Automated FLS coverage scan | Real-time FLS anomaly detection |
| API access | API Enabled broadly | API limited by license | API only for named roles | API + IP restrictions | API + JWT Bearer only | Zero standing API access |
| Code security | No static analysis | Manual review | PMD/ESLint in CI | SCA gates blocking deployment | DAST + runtime monitoring | Self-healing security controls |

**Target minimum for production orgs: Level 3**

---

## Pillar 5: Data Maturity

| Check | Level 0 | Level 1 | Level 2 | Level 3 | Level 4 | Level 5 |
|-------|---------|---------|---------|---------|---------|---------|
| Data classification | No classification | Some fields labeled | All PII fields classified | Classification drives FLS | Automated classification | Dynamic classification at ingest |
| Encryption at rest | No encryption | Salesforce default encryption | Shield Encryption on PII fields | Shield + BYOK | EKM with HSM | Post-quantum key management |
| Data masking | No masking | Manual masking | Partial automated masking | Full automated post-refresh masking | Continuous masking monitoring | Synthetic data generation |
| Data residency | Unknown | Data center known | Contract specifies region | Hyperforce + DPA executed | Customer Trust Access Management | Dedicated infrastructure |
| DLP monitoring | No monitoring | Manual data export review | Event Monitoring for exports | CASB integration | Real-time DLP alerts | Behavioral DLP |

**Target minimum for production orgs: Level 2-3**

---

## Composite Scoring

### Per-Pillar Score

Average the maturity levels across all checks within each pillar:

```
Pillar Score = Sum of check levels / Number of checks
```

### Weighted Composite Score

| Pillar | Weight | Your Score (0-5) | Weighted Score |
|--------|--------|-----------------|---------------|
| Identity | 30% | ___ | ___ |
| Device | 15% | ___ | ___ |
| Network | 20% | ___ | ___ |
| Application | 25% | ___ | ___ |
| Data | 10% | ___ | ___ |
| **Composite** | 100% | — | ___ |

### Rating Bands

| Composite Score | Rating | Recommended Action |
|----------------|--------|--------------------|
| 4.0 - 5.0 | Advanced/Optimized | Maintain; focus on continuous improvement |
| 3.0 - 3.9 | Established | Close specific gaps; automate manual controls |
| 2.0 - 2.9 | Developing | Prioritize Identity and Application pillars first |
| 1.0 - 1.9 | Initial | Emergency remediation plan; executive escalation |
| 0.0 - 0.9 | None | Immediate stop-gap controls; full remediation program |

---

## Gap Analysis Template

For each pillar where the current level is below the target:

```
PILLAR: [Identity / Device / Network / Application / Data]
CURRENT LEVEL: [0-5]
TARGET LEVEL: [0-5]
GAP: [Target - Current]

KEY GAPS:
1. [Specific control that is missing or incomplete]
2. [...]

REMEDIATION ACTIONS:
Priority 1 (< 30 days):
  - Action: [Specific action]
  - Owner: [Team or role]
  - Effort: [Hours / Days / Weeks]
  - Success Metric: [How to verify completion]

Priority 2 (30-90 days):
  - Action: [...]

Priority 3 (90-180 days):
  - Action: [...]

BLOCKERS:
  - [Budget / Resource / Technical dependency]

NEXT ASSESSMENT DATE: [Date 6-12 months from now]
```

---

## Maturity Assessment Cadence

| Assessment Type | Frequency | Scope |
|----------------|-----------|-------|
| Self-assessment | Quarterly | All five pillars |
| Internal audit | Annually | All five pillars + evidence |
| Third-party assessment | Bi-annually or for compliance | All five pillars |
| Post-incident review | After any security incident | Affected pillars |
| Post-major-change review | After significant org changes | Affected pillars |
