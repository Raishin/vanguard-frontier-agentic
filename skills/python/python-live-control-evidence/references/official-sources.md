# Official Sources

Primary standards and library documentation provenance for the evidence-integrity and control-mapping claims.

Primary sources, verified 2026-07-26 against official upstream documentation and standards. Governance framings are non-certifying (see docs/compliance/).

## Source register

- https://csrc.nist.gov/pubs/sp/800/53/r5/upd1/final
- https://www.aicpa-cima.com/resources/landing/system-and-organization-controls-soc-suite-of-services
- https://docs.python.org/3/library/hashlib.html

## Provenance notes

- csrc.nist.gov (NIST SP 800-53), the AICPA SOC 2 Trust Services Criteria, and docs.python.org (`hashlib`) are the authoritative upstream references for the evidence-integrity and control-mapping claims in this skill; they describe control intent and cryptographic hashing behavior, not a certification that any sealed evidence proves control effectiveness.

## Grounding rule

Documentation and standards describe expected behaviour and control intent. They do not prove the target's live state, that a control operated, or that a framework applies. Applicability and compliance are owner determinations; treat any such claim as `assumption` until independently observed and owner-confirmed.
