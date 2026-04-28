# MCP and Evidence Path

## Evidence path

Ground the review in this order:

1. **Live Azure evidence when available**
   - resource exposure,
   - identity model,
   - role assignments,
   - policy assignments/exemptions,
   - Key Vault configuration,
   - diagnostic settings and monitoring paths.
2. **Azure MCP evidence when supported by the client and enabled**
   - `keyvault` for vault inventory and secret/certificate posture,
   - `role` for RBAC evidence,
   - `policy` for guardrail posture,
   - `monitor` for diagnostic and logging checks,
   - `advisor` for supporting posture signals.
3. **Official Microsoft documentation** for design decisions and corrective guidance.
4. **Explicit assumptions** when live evidence is missing.

If the evidence is incomplete, say so. Do not claim the environment is secure from design intent alone.
