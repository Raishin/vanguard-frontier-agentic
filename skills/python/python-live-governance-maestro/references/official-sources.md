# Official Sources

Primary NIST, OWASP, and Python documentation the router relies on to classify a live task.

Primary sources, verified 2026-07-26 against official upstream documentation and standards. Governance framings are non-certifying (see docs/compliance/).

## Source register

- https://www.nist.gov/cyberframework
- https://csrc.nist.gov/pubs/sp/800/53/r5/upd1/final
- https://owasp.org/www-project-top-10-for-large-language-model-applications/
- https://docs.python.org/3/

## Provenance notes

- nist.gov's Cybersecurity Framework, csrc.nist.gov SP 800-53, and the OWASP LLM Top 10 are the authoritative upstreams for the risk-tiering and injection-defense framing the router applies; docs.python.org grounds the runtime/version context. These are control-design and documentation references, not a certification of any specific routing decision.

## Grounding rule

Documentation and standards describe expected behaviour and control intent. They do not prove the target's live state, that a control operated, or that a framework applies. Applicability and compliance are owner determinations; treat any such claim as `assumption` until independently observed and owner-confirmed.
