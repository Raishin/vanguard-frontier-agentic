# Authentication Posture And Git Folder Segregation

OAuth and environment-based authentication, token storage, and Git folder flows that prevent accidental environment promotion.

- Databricks recommends OAuth over personal access tokens. OAuth U2M tokens expire after one hour and refresh automatically; from CLI v1.0.0, tokens are stored in OS-native secure storage (macOS Keychain, Windows Credential Manager, Linux D-Bus), and a plaintext fallback (`DATABRICKS_AUTH_STORAGE=plaintext`) requires explicit security approval.
- OAuth M2M uses `client_id` and `client_secret`; a service principal holds up to five OAuth secrets, each valid up to two years. Bundles require Databricks CLI v0.218.0 or above.
- Authentication precedence is (1) bundle settings, (2) environment variables, (3) `.databrickscfg` profiles. A bundle that hardcodes a workspace URL or personal access token in config files is a credential exposure, not a supported pattern.
- Git folder flows segregate three paths: admin (production-only folders, protected branches, automation-owned), user (personal branches, user-owned), and merge (automation pulls approved changes to production). A single folder mixing admin and user branches is a governance gap that permits accidental production pushes.
- There is no built-in workspace-to-workspace promotion mechanism in bundles; promotion is driven by separate targets and external CI/CD. A CI/CD gate that does not explicitly prevent promotion from user to production branches is insufficient — the gate must be written as a hard block, not a warning.
