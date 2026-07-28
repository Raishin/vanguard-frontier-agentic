# Official Sources

Primary NIST standards-body and Python documentation for the governance and evidence claims.

Primary sources, verified 2026-07-26 against official upstream documentation and cross-checked via the Context7 MCP where a version-sensitive or security-sensitive claim was encoded. Blogs are used only for explanation, never as the sole source for normative behaviour.

## Source register

- https://csrc.nist.gov/pubs/sp/800/53/r5/upd1/final
- https://csrc.nist.gov/glossary/term/separation_of_duty
- https://docs.python.org/3/library/logging.html
- https://peps.python.org/pep-0020/

## Provenance notes

- csrc.nist.gov (NIST SP 800-53r5 and the separation-of-duty glossary entry) is the authoritative standards-body source for the segregation-of-duties control claim here; docs.python.org and PEP 20 ground the evidence-logging and clarity claims.
- Context7 NOT separately used — the control claims are grounded in NIST SP 800-53r5 (separation of duties AC-5) as the standards-body source; this agent maps controls and does not make accounting, legal, or regulatory determinations.

## Grounding rule

Documentation explains language, library, and platform behaviour in general. It does not prove the interpreter build, installed package versions, target configuration, or runtime the user actually ships. Treat any claim that depends on the user's specific versions or runtime as `assumption` until the source, lockfile, or build files confirm it.
