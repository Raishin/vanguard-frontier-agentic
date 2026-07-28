# Application-Security Review Checklist

The per-sink checklist applied to every Python application-security review.

- Deserialization: no `pickle`/`marshal`/`shelve` load and no `yaml.load` without `SafeLoader` is reachable from untrusted input.
- Dynamic execution: no `eval`/`exec`/`compile`/`__import__` receives an attacker-influenced string.
- Subprocess: every external command uses an argument list with `shell=False`; no untrusted value is interpolated into a shell string.
- SSRF: every outbound request target is allowlisted and loopback/link-local/private ranges are blocked after DNS resolution.
- Filesystem: untrusted filenames and archive members are canonicalized and confined under a fixed base; `..` and absolute members are rejected.
- Secrets: no credential/token/key is hardcoded or written to logs or exception messages.
- Cryptography: password storage uses a memory-hard KDF; encryption is authenticated with a random IV; secret comparison uses `hmac.compare_digest`.
- Error handling: authentication, authorization, and signature-verification steps fail closed, not open.
