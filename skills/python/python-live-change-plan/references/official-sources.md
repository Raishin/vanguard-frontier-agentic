# Official Sources

Primary NIST configuration-change-control documentation this agent relies on.

Primary sources, verified 2026-07-26 against official upstream documentation and standards. Governance framings are non-certifying (see docs/compliance/).

## Source register

- https://csrc.nist.gov/pubs/sp/800/53/r5/upd1/final
- https://docs.python.org/3/
- https://owasp.org/www-project-top-10-for-large-language-model-applications/

## Provenance notes

- csrc.nist.gov SP 800-53 (CM-3 configuration-change control) is the authoritative upstream for the plan-approval-execution binding this agent implements; it is a control-design reference, not proof that any specific plan was executed as approved.

## Grounding rule

Documentation and standards describe expected behaviour and control intent. They do not prove the target's live state, that a control operated, or that a framework applies. Applicability and compliance are owner determinations; treat any such claim as `assumption` until independently observed and owner-confirmed.
