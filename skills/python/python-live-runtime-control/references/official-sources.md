# Official Sources

Primary Python interpreter/process/diagnostics documentation this agent relies on.

Primary sources, verified 2026-07-26 against official upstream documentation and standards. Governance framings are non-certifying (see docs/compliance/).

## Source register

- https://docs.python.org/3/library/sys.html
- https://docs.python.org/3/library/gc.html
- https://docs.python.org/3/library/faulthandler.html

## Provenance notes

- docs.python.org (sys, gc, faulthandler) is the authoritative upstream for these introspection APIs; it documents what each call observes, not the operational significance of any specific reading in a given deployment.

## Grounding rule

Documentation and standards describe expected behaviour and control intent. They do not prove the target's live state, that a control operated, or that a framework applies. Applicability and compliance are owner determinations; treat any such claim as `assumption` until independently observed and owner-confirmed.
