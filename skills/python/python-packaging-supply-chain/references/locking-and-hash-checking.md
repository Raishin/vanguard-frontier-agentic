# Locking And Hash-Checking

Reproducible installs, lockfiles, and pip's all-or-nothing hash-checking mode.

- A lockfile with exact versions makes a resolution reproducible; without one, `pip` re-resolves against the live index at each install and can pick a newer (possibly compromised) release.
- pip's hash-checking mode verifies each downloaded artifact against local hashes in the requirements file, protecting against index compromise and tampering.
- Hash-checking is all-or-nothing: if a hash is provided for any requirement, hashes must be provided for all requirements and all their transitive dependencies, and every requirement must be pinned to an exact version; `--require-hashes` forces this mode.

## Sources

- https://pip.pypa.io/en/stable/topics/secure-installs/
- https://pip.pypa.io/en/stable/topics/repeatable-installs/
