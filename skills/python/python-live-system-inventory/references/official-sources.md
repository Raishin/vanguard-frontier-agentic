# Official Sources

Primary Python packaging/metadata documentation and NIST asset-inventory framing this agent relies on.

Primary sources, verified 2026-07-26 against official upstream documentation and standards. Governance framings are non-certifying (see docs/compliance/).

## Source register

- https://docs.python.org/3/library/importlib.metadata.html
- https://packaging.python.org/en/latest/
- https://csrc.nist.gov/pubs/sp/800/53/r5/upd1/final

## Provenance notes

- docs.python.org (importlib.metadata) and packaging.python.org are the authoritative upstreams for Python package/distribution metadata; csrc.nist.gov SP 800-53 informs the CM-8 asset-inventory framing, which is a control-design reference, not a certification of this agent's output.

## Grounding rule

Documentation and standards describe expected behaviour and control intent. They do not prove the target's live state, that a control operated, or that a framework applies. Applicability and compliance are owner determinations; treat any such claim as `assumption` until independently observed and owner-confirmed.
