# Compliance Applicability Engine

> How the Python live control plane decides **which** control profiles apply to a given
> action — before any control is evaluated. Applicability is a *proposal* the engine
> produces for an accountable owner to confirm; it is never a legal classification.

> [!IMPORTANT]
> Two failure modes are forbidden:
> - **Never apply a framework merely because its name is familiar.** Recognizability is
>   not applicability.
> - **Never omit a framework merely because the system is internally developed.** Internal
>   build is not an exemption.
>
> The engine surfaces candidate frameworks and the reasons they may apply. A qualified
> owner (legal, privacy, compliance, security, the process owner) confirms or rejects
> each. No agent declares a framework applicable or a system compliant.

## Inputs the engine must determine

Before mapping controls, the `python-live-policy-gate-agent` (with inputs from inventory,
identity, and change-plan agents) records each of these as a labeled fact with an evidence
quality tag — `unknown` is a valid, and blocking-for-R3+, value:

- Organization and legal entity
- Jurisdiction(s)
- Industry / sector
- Business process served
- Data categories processed
- Categories of affected individuals
- System purpose
- Whether the target is a **production** environment
- Financial-reporting impact (SOX relevance)
- Payment-card scope (PCI DSS)
- Health-data scope (HIPAA)
- Personal-data scope (GDPR and other privacy law)
- Employment / worker impact
- Critical-infrastructure relevance (NIS2 and sector rules)
- AI-system role, and whether the org is **provider, deployer, importer, distributor, or
  internal user** for that system (EU AI Act roles)
- Contractual requirements and customer commitments
- Applicable internal policies

## From inputs to a candidate profile

The engine emits a **candidate control profile**: the set of control objects
(`schemas/control-object.schema.json`) whose `applicability` matches the recorded inputs,
each carrying `mapping_confidence: candidate` until an owner confirms it. The profile also
records, for every framework it did **not** include, the input that excluded it — so an
owner can challenge an omission as easily as an inclusion.

## Action risk tiers (R0–R5)

Applicability is scoped by action risk as well as by framework:

| Tier | Meaning | Gate posture |
|---|---|---|
| R0 | Read-only, non-sensitive | Observe; audit event recommended |
| R1 | Read-only, sensitive data in scope | Read-only-runtime; runtime-evidence-gate; redaction |
| R2 | Reversible non-production change | Plan + policy gate; approval per policy |
| R3 | Production change, bounded, reversible | Independent approval + JIT + rollback + audit event; fail closed if audit unavailable |
| R4 | Production change, broad blast radius | R3 controls + blast-radius assessment + change window |
| R5 | Irreversible / high-blast-radius production action | R4 controls + second approver + explicit risk acceptance |

## Non-negotiables

- The engine produces **applicability proposals and control candidates**, never
  compliance conclusions or legal determinations.
- An `unknown` for any R3+-relevant input **blocks** until resolved by an owner.
- Adding or removing a framework from a profile is a governed change with its own audit
  event; it is never inferred silently by an agent from prompt text.
