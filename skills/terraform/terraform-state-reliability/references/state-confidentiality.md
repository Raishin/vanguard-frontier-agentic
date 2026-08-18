# State Confidentiality And Engine Encryption

What state records in the clear, and the one engine that can encrypt it natively.

- State records resource attributes as returned by the provider, including values the configuration marks `sensitive`. The `sensitive` marker suppresses display in plan and apply output; it does not encrypt, redact, or omit the value from state.
- Any secret passed through a resource argument — a generated password, an access key, a private key, a connection string — is therefore recoverable by anyone with read access to the state file, which makes state read access equivalent to secret read access.
- OpenTofu supports native encryption of state and plan files at rest; Terraform does not offer an equivalent engine-level feature, so a Terraform estate's state confidentiality depends entirely on the backend's own encryption and access control.
- OpenTofu's encryption supports several key providers — a passphrase-derived key via PBKDF2, AWS KMS, GCP KMS, Azure Key Vault, and OpenBao — and AES-GCM is the only production encryption method, requiring a 16, 24, or 32 byte key.
- Key rollover works through a fallback block: when a read fails under the new method, OpenTofu tries the fallback, but every write uses the new method, so the fallback is a migration aid rather than a permanent dual-key arrangement.
- Encrypted state whose key is lost is permanently unrecoverable. Enabling encryption therefore trades a confidentiality risk for an availability risk, and is only a net gain when key custody, rollover, and a tested recovery procedure are established first.
- Renaming a key provider or method after encryption breaks the metadata references that let OpenTofu find the right key, unless an explicit encrypted-metadata alias preserves the old name — which makes the naming decision effectively permanent.
- A saved plan file also records sensitive values in cleartext, so plan artifacts moving through CI need the same handling as state; OpenTofu can encrypt plan files as well as state, while a Terraform estate must protect the artifact by other means.
