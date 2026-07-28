# Official Sources

Primary standards and legal-text provenance for the ownership/reconciliation and data-minimization claims.

Primary sources, verified 2026-07-26 against official upstream documentation and standards. Governance framings are non-certifying (see docs/compliance/).

## Source register

- https://csrc.nist.gov/pubs/sp/800/53/r5/upd1/final
- https://gdpr-info.eu/
- https://docs.python.org/3/library/decimal.html

## Provenance notes

- csrc.nist.gov (NIST SP 800-53) and gdpr-info.eu (GDPR) are the authoritative upstream references for the ownership/classification/reconciliation and data-minimization claims in this skill; docs.python.org documents the `decimal` module's exact-arithmetic behavior. These describe control and legal intent, not a certification that any specific data change or organization is compliant.

## Grounding rule

Documentation and standards describe expected behaviour and control intent. They do not prove the target's live state, that a control operated, or that a framework applies. Applicability and compliance are owner determinations; treat any such claim as `assumption` until independently observed and owner-confirmed.
