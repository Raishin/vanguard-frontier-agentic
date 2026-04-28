# MCP and Evidence Path

## Official Azure MCP Linkage

Use only official Azure MCP capabilities that are actually exposed in the active client runtime. Do not invent tools.

Preferred official linkage for this role:

- `deploy` for Azure deployment workflows when the client exposes official Azure deploy tooling.
- `bicepschema` when the client exposes Azure MCP schema retrieval for Bicep-oriented infrastructure definition.
- `extension` when the user needs official guidance for Azure Developer CLI or related deployment tooling setup.

Rules:

- Verify available Azure MCP capabilities from the official Azure MCP tool inventory before leaning on them.
- Treat read-only discovery differently from mutating deployment operations.
- Do not imply that Bicep MCP authoring tools can deploy resources directly when the official documentation says they are for code-generation support rather than direct deployment.
- If the expected Azure MCP capability is absent, switch cleanly to documentation-based guidance instead of hallucinating a tool path.

## Platform-Agnostic Execution

This skill must work across macOS, Windows, Linux, and MCP-only clients.

- Prefer architecture decisions, pipeline stages, and control patterns over shell-specific commands.
- When examples are useful, show neutral placeholders such as `<subscription>`, `<management-group>`, `<pipeline>`, `<service-connection>`, and `<identity>`.
- Adapt command syntax only after the user’s actual runtime or CI system is known.
- Keep Bicep, Terraform, Azure DevOps, GitHub Actions, and Azure Developer CLI examples conceptually portable unless the user has already fixed the toolchain.

## Documentation Fallback When Live Data Is Unavailable

Live Azure MCP evidence beats static guidance for current-state validation. If live Azure inspection is unavailable, incomplete, unsafe, or denied:

- fall back to official Microsoft Learn and Azure Architecture Center guidance listed above,
- ask for sanitized pipeline structure, repo layout, stage diagram, or redacted deployment definitions if current-state evidence is required,
- label conclusions as `live evidence`, `documentation-based`, `user-provided sanitized evidence`, or `inference`,
- do not claim the user’s current pipeline is safe merely because a Microsoft pattern exists,
- do not claim a tool is available in the user’s runtime unless it was confirmed live.
