# Official Sources

Primary NIST identity, JIT-access, and separation-of-duties documentation this agent relies on.

Primary sources, verified 2026-07-26 against official upstream documentation and standards. Governance framings are non-certifying (see docs/compliance/).

## Source register

- https://csrc.nist.gov/pubs/sp/800/53/r5/upd1/final
- https://csrc.nist.gov/glossary/term/separation_of_duty
- https://owasp.org/www-project-top-10-for-large-language-model-applications/

## Provenance notes

- csrc.nist.gov SP 800-53 (AC-2, AC-5) and the separation-of-duty glossary term are the authoritative upstreams for identity, JIT access, and separation-of-duties framing; they describe control design, not proof that a specific principal's identity or authority claim is genuine.

## Grounding rule

Documentation and standards describe expected behaviour and control intent. They do not prove the target's live state, that a control operated, or that a framework applies. Applicability and compliance are owner determinations; treat any such claim as `assumption` until independently observed and owner-confirmed.
