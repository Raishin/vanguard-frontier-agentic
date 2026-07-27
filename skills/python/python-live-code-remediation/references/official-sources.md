# Official Sources

Primary NIST change-control and Python packaging documentation this agent relies on.

Primary sources, verified 2026-07-26 against official upstream documentation and standards. Governance framings are non-certifying (see docs/compliance/).

## Source register

- https://csrc.nist.gov/pubs/sp/800/53/r5/upd1/final
- https://packaging.python.org/en/latest/
- https://owasp.org/www-project-top-10-for-large-language-model-applications/

## Provenance notes

- csrc.nist.gov SP 800-53 and packaging.python.org are the authoritative upstreams for the change-control and dependency-remediation framing this agent applies; they describe control design and packaging practice, not proof that a specific PR was reviewed or merged correctly.

## Grounding rule

Documentation and standards describe expected behaviour and control intent. They do not prove the target's live state, that a control operated, or that a framework applies. Applicability and compliance are owner determinations; treat any such claim as `assumption` until independently observed and owner-confirmed.
