# Framework Profiles

> Configurable control profiles for the Python live control plane. A profile is a named,
> versioned bundle of control objects
> ([control-object.schema.json](../../schemas/control-object.schema.json)) selected by the
> [applicability engine](applicability-engine.md) and confirmed by an owner. A profile is
> a **research and governance aid, never a certification.**

## Supported frameworks

Profiles may compose controls mapped to any of these. Each entry states the honest,
non-overclaiming framing this repo requires.

| Framework | Framing (do not overclaim) |
|---|---|
| **NIST SP 800-53** | A **customizable catalog** of security and privacy controls, tailored by baseline and overlay — not a universal one-size-fits-all checklist. |
| **NIST Cybersecurity Framework (CSF)** | An outcome-oriented risk framework (Govern/Identify/Protect/Detect/Respond/Recover). Guides organization, not a pass/fail certification. |
| **NIST AI RMF** | Used for AI **risk governance** (Govern/Map/Measure/Manage). Not a certification and not evidence of legal compliance. |
| **NIST Generative AI Profile** | A companion profile to AI RMF for GenAI risks. Risk governance, not certification. |
| **ISO/IEC 27001** | An ISMS standard; certification is issued by accredited bodies, never by an agent or a response. |
| **ISO/IEC 42001** | An AI management-system standard; same certification caveat as 27001. |
| **SOC 2** | Trust Services Criteria assessed by an independent CPA firm over a period. Agent output is at most an input to that assessment. |
| **PCI DSS** | Applies only in defined cardholder-data scope; validated by QSA/SAQ, not by agent output. |
| **HIPAA Security Rule** | Applies where ePHI is in scope; legal obligation on covered entities/business associates. |
| **SOX ITGC + automated controls** | IT general controls and automated business controls relevant to financial-reporting integrity; management asserts, auditors test. |
| **GDPR** | Legal obligation for personal-data processing; lawful basis, DPIA, and data-subject rights are owner/DPO determinations. |
| **EU AI Act** | **Risk-based**: unacceptable / high-risk / limited / minimal. High-risk requirements include risk management, data governance, logging, transparency, human oversight, accuracy/robustness, cybersecurity, and post-market monitoring. **Legal classification and the org's role (provider/deployer/importer/distributor/user) must be confirmed by qualified owners.** |
| **NIS2** | Sector- and size-scoped cybersecurity-risk-management and reporting obligations where applicable. |
| **Internal secure-development policy** | Org-defined; the profile references the specific policy version. |
| **Internal change-management policy** | Org-defined; drives the approval/rollback controls. |
| **Internal AI-use policy** | Org-defined; drives model-promotion and GenAI controls. |
| **Customer-specific contractual controls** | Per-contract; referenced by control objects with a `customer-contractual` mapping. |

## Threat-source inputs (not compliance frameworks)

- **OWASP agentic-security / LLM guidance** is used as a **threat source** that informs
  detective and preventive mechanisms in control objects. It is not a compliance
  certificate and is never mapped as one.

## Profile rules

- A profile is **versioned** (`policy_bundle_version` in the audit event) so every action
  records exactly which controls were in force.
- Framework inclusion in a profile is a **candidate** (`mapping_confidence: candidate`)
  until an accountable owner confirms it (`owner-confirmed`). Agents never set
  `owner-confirmed`.
- A profile never asserts certification, accreditation, attestation, or legal compliance.
  Those come from accredited bodies, independent assessors, and the organization's
  qualified owners.
- Composing a profile does not establish that its controls **operate**; operation is
  shown by tested evidence over time (see
  [evidence-quality-model.md](evidence-quality-model.md)).

## Relationship to the control plane

The [applicability engine](applicability-engine.md) proposes the profile; the
`python-live-policy-gate-agent` evaluates the profile's control objects against an action;
the `python-live-control-evidence-agent` collects and seals the required evidence; and the
`python-live-continuous-control-testing-agent` re-checks that the profile's controls keep
operating — opening findings rather than silently remediating.
