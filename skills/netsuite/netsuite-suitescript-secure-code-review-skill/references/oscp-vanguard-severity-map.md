# Oscp Vanguard Severity Map

Mapping of OSCP-001 through OSCP-048 pitfall IDs to Vanguard severity taxonomy and CI gate recommendations

Scope: Reviews SuiteScript 2.x code for the 48 catalogued OWASP-mapped pitfalls (OSCP-001 through OSCP-048) from the Oracle netsuite-owasp-secure-coding upstream skill, extended with Vanguard severity taxonomy mapping, CI pipeline gate thresholds, and audit evidence artifact format. Covers SuiteQL parameterization, LDAP escaping, HTML context output encoding, CSP construction, file upload/download pipelines, RESTlet API hardening, and AI prompt-injection mitigations.

- SuiteQL injection review — parameterized query usage, dynamic string concatenation in N/query or N/search calls, ROWNUM limit enforcement, NVL wrapping for null safety
- Output encoding for five HTML contexts — HTML body, HTML attribute, JavaScript, CSS, and URL encoding correctness in SuiteScript Suitelet and RESTlet responses
- CSP construction review — Content-Security-Policy header presence and policy strength in RESTlet and Suitelet responses
- File upload and download pipeline security — MIME type validation, path traversal prevention, size limits, server-side validation in file cabinet operations
- RESTlet API hardening — authentication enforcement, input validation, error response sanitization, rate-limiting awareness
- CSRF prevention — token presence and validation in state-changing SuiteScript operations
- DOM XSS and postMessage origin validation — client-side SuiteScript patterns using document.write, innerHTML, or postMessage without origin checks
- AI prompt-injection mitigations — SuiteScript code that passes user-controlled input to AI APIs without sanitization or boundary enforcement
