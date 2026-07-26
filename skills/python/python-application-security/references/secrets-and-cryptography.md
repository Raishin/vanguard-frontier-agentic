# Secrets Handling And Cryptography

Secret storage/comparison and correct use of the standard-library crypto primitives.

- `hmac.compare_digest` performs a constant-time comparison and must be used for comparing secrets, tokens, and signatures; a plain `==` comparison leaks length and content through timing.
- The `secrets` module (not `random`) must be used to generate tokens, API keys, and password-reset nonces, because `random` is not cryptographically secure.
- Password storage must use a memory-hard KDF (argon2/scrypt/bcrypt), not a bare MD5/SHA-1/SHA-256 digest; symmetric encryption must be authenticated (e.g. AES-GCM) with a unique random nonce, never ECB or a static IV.

## Sources

- https://docs.python.org/3/library/hmac.html#hmac.compare_digest
- https://docs.python.org/3/library/secrets.html
- https://cheatsheetseries.owasp.org/cheatsheets/Cryptographic_Storage_Cheat_Sheet.html
