# Mirrors, Overrides, And Invisible Redirection

The configuration outside the repository that decides where code actually comes from.

- A `provider_installation` block in the CLI configuration file can redirect every provider fetch in an environment to a filesystem or network mirror, and nothing in the repository under review reveals that it exists.
- A mirror is a trust decision, not a caching decision: whoever controls the mirror controls the code that runs with the estate's credentials, so an internal mirror needs the same provenance controls as the upstream registry it replaces.
- A mirror that serves packages without preserving the hashes the lock file can verify converts a verified installation into an unverified one, even when the mirror is operated by the same organization.
- `dev_overrides` deliberately bypasses version constraints and checksum verification for the overridden providers, because its purpose is local provider development; its presence in any shared image, runner, or non-developer environment removes verification entirely for those providers.
- Because CLI configuration lives outside the repository, its contents must be requested explicitly during review; assessing supply-chain posture from the configuration alone assumes a default installation path that may not be in use.
- An air-gapped installation is not automatically safer: it replaces registry verification with whatever process populates the mirror, and that process is frequently a manual copy with no attestation at all.
- A registry URL or module source that embeds a token exposes that credential to every log, cache, and state file the reference touches, which makes the embedded credential a finding independent of whether the source itself is trustworthy.
