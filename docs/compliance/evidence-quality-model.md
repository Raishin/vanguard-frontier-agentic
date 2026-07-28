# Evidence Quality Model

> Multi-dimensional evidence quality for the Python live control plane. Extends — does
> not replace — the `evidence_level` field (`verified` | `partial` | `assumed`) in
> [evidence-output-spec.md](../evidence-output-spec.md).

`evidence_level` alone cannot tell a reviewer how much weight a piece of evidence can
carry. A value can be "verified" and still be self-reported, stale, or restricted. Every
evidence item produced or referenced by a live-control-plane agent SHOULD be tagged along
the dimensions below so a control owner and an independent assessor can judge it.

> [!IMPORTANT]
> None of these dimensions turn evidence into proof. High-quality evidence is a stronger
> *input* to control testing; the auditor still decides acceptance, and legal compliance
> remains the organization's responsibility. See
> [What a response does and does not establish](../evidence-output-spec.md#what-a-response-does-and-does-not-establish).

## Dimensions

| Dimension | Values | Meaning |
|---|---|---|
| **Source** | `live` \| `user-supplied` \| `generated` \| `third-party` | Where the evidence originated. `generated` (an agent produced it) is weakest for independence. |
| **Integrity** | `signed` \| `hashed` \| `unsigned` | Tamper-evidence. `unsigned` evidence cannot be relied on for R3+ actions. |
| **Freshness** | `current` \| `stale` \| `unknown` | Whether the captured state still reflects reality. `unknown` freshness downgrades to `assumed`. |
| **Completeness** | `complete` \| `partial` \| `missing` | Whether the full required evidence set is present. `missing` for a required item blocks. |
| **Independence** | `self-reported` \| `independently-observed` | Whether the executor is also the attester. A verifier reusing the executor's own claim is `self-reported` and fails segregation of duties. |
| **Sensitivity** | `public` \| `internal` \| `confidential` \| `restricted` | Drives redaction, storage, and access control. |
| **Control stage** | `designed` \| `implemented` \| `observed` \| `tested` \| `attested` | Which stage of the control lifecycle this evidence covers. Design ≠ operation; one observation ≠ tested effectiveness. |
| **Retention** | `temporary` \| `policy-retained` \| `legal-hold` | Governs deletion. `legal-hold` items must never be purged. |
| **Assessor status** | `unreviewed` \| `owner-reviewed` \| `independently-assessed` | Who, if anyone, has assessed it. `unreviewed` internal evidence does not substitute for independent assessment. |

## Rules

- A required evidence item that is `missing`, `unsigned` (for R3+), or of `unknown`
  freshness **blocks** the action rather than downgrading silently.
- `verdict: approved` with `evidence_level: assumed` is never sufficient for an R3+
  action — it means the baseline could not be captured, so the control cannot be shown to
  have operated.
- Independence is not optional for approval and verification: the approver must not be the
  requester, and the verifier must not reuse the executor's claim
  (`self-reported` → fail).
- Sensitivity drives handling: `confidential`/`restricted` evidence is redacted or
  tokenized before it enters any prompt, log, or third-party tool (see the privacy
  controls in the live agents).
- Control stage must be stated explicitly. Evidence at stage `observed` supports "the
  control ran once," not "the control is operating effectively" (which requires `tested`
  over a population and, for reliance, `independently-assessed`).

## Relationship to the audit event

Each `control_results[].evidence_digest` and the event-level `evidence_digest` in
[audit-event.schema.json](../../schemas/audit-event.schema.json) seal the evidence bundle
these dimensions describe. The digest proves the bytes did not change after sealing; it
does not prove the evidence was accurate or the control effective.
