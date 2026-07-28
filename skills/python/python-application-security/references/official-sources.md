# Official Sources

Primary Python standard-library and OWASP/CWE security sources for this board.

Primary sources, verified 2026-07-26 against official upstream documentation and cross-checked via the Context7 MCP where a version-sensitive or security-sensitive claim was encoded. Blogs are used only for explanation, never as the sole source for normative behaviour.

## Source register

- https://docs.python.org/3/library/pickle.html
- https://docs.python.org/3/library/subprocess.html#security-considerations
- https://docs.python.org/3/library/secrets.html
- https://owasp.org/www-community/vulnerabilities/Deserialization_of_untrusted_data
- https://cheatsheetseries.owasp.org/cheatsheets/Injection_Prevention_Cheat_Sheet.html

## Provenance notes

- OWASP and MITRE CWE are the primary security sources for the severity model; docs.python.org is the primary source for standard-library sink behaviour (pickle, subprocess, hmac, secrets).
- Context7 MCP was not used as a separate source for this skill: the standard-library security semantics cited here (pickle untrusted-source warning, subprocess `shell=True` hazard, `hmac.compare_digest`) are stable across current CPython releases and are quoted directly from docs.python.org, which the repository treats as the authoritative upstream.

## Grounding rule

Documentation explains language, library, and platform behaviour in general. It does not prove the interpreter build, installed package versions, target configuration, or runtime the user actually ships. Treat any claim that depends on the user's specific versions or runtime as `assumption` until the source, lockfile, or build files confirm it.
