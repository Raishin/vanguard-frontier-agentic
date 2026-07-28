# Injection, SSRF, And Unsafe File Handling

Subprocess/shell injection, SSRF, and path-traversal/zip-slip controls.

- The subprocess security-considerations documentation states that `shell=True` is a security hazard when combined with untrusted input; passing an argument list with `shell=False` avoids shell metacharacter interpretation entirely.
- SSRF (CWE-918) is prevented by resolving the target host and validating it against an allowlist, and by blocking loopback, link-local (including 169.254.169.254), and private ranges — a string prefix check on the raw URL is insufficient because of DNS rebinding and redirects.
- Path traversal (CWE-22) and zip-slip are prevented by canonicalizing the final path with `os.path.realpath` and asserting it remains under a fixed base directory before any read or write, and by rejecting archive members containing `..` or absolute paths.

## Sources

- https://docs.python.org/3/library/subprocess.html#security-considerations
- https://cwe.mitre.org/data/definitions/918.html
- https://cwe.mitre.org/data/definitions/22.html
