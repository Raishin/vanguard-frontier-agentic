# Azure MCP Server

- Vendor: Microsoft
- Status: official Microsoft Azure MCP Server
- Docs: <https://learn.microsoft.com/en-us/azure/developer/azure-mcp-server/overview>
- Source: <https://github.com/microsoft/azure-mcp>
- Microsoft MCP catalog: <https://github.com/microsoft/mcp>
- Auth model: Microsoft Entra ID / Azure Identity patterns through supported clients.
- Mutation risk: tools can interact with Azure resources according to RBAC permissions.
- Last verified: 2026-04-27

## Install/config

Use the current Microsoft Learn setup instructions for your client. Avoid tenant-wide or subscription-wide roles unless required and approved.

## Other official Microsoft MCP servers listed in `microsoft/mcp`

The Azure MCP Server is only one entry in Microsoft's broader MCP catalog. Based on the `microsoft/mcp` README as viewed on **2026-04-27**, Microsoft also lists the following MCP servers.

### Catalog snapshot

| Name                                    | Repo / docs                              | Category                 | Type   | Endpoint / notes                                                                                   | What it is for                                                                                   |
| --------------------------------------- | ---------------------------------------- | ------------------------ | ------ | -------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| ✨ Microsoft Foundry                    | Microsoft Learn docs                     | Cloud and infrastructure | Remote | `https://mcp.ai.azure.com`                                                                         | Foundry tools for models, knowledge, evaluation, and related AI workflows.                       |
| Azure DevOps                            | Azure DevOps MCP Server                  | Developer tools          | Local  | Local server                                                                                       | Azure DevOps work from the editor.                                                               |
| ☸️ Azure Kubernetes Service (AKS)       | `Azure/aks-mcp`                          | Cloud and infrastructure | Local  | Local server                                                                                       | Natural-language interaction with AKS clusters.                                                  |
| GitHub                                  | `github/github-mcp-server`               | Developer tools          | Remote | `https://api.githubcopilot.com/mcp`                                                                | GitHub repositories, issues, and pull requests.                                                  |
| GitHub Awesome-Copilot                  | `github/awesome-copilot`                 | Developer tools          | Local  | Local server                                                                                       | Copilot instructions, prompts, and configs.                                                      |
| Markitdown                              | `microsoft/markitdown`                   | Developer tools          | Local  | Local server                                                                                       | Markdown conversion and transformation workflows.                                                |
| Microsoft 365 Agents Toolkit            | `OfficeDev/microsoft-365-agents-toolkit` | Developer tools          | Local  | Local server                                                                                       | Building apps and agents for Microsoft 365 and Copilot.                                          |
| Microsoft 365 Calendar                  | `bap-microsoft/MCP-Platform`             | Productivity             | Remote | `https://agent365.svc.cloud.microsoft/agents/tenants/{tenant_id}/servers/mcp_CalendarTools`        | Calendar events, invites, and availability.                                                      |
| Microsoft 365 Copilot Chat              | `bap-microsoft/MCP-Platform`             | Productivity             | Remote | `https://agent365.svc.cloud.microsoft/agents/tenants/{tenant_id}/servers/mcp_M365Copilot`          | Search and chat across Microsoft 365 content.                                                    |
| Microsoft 365 Mail                      | `bap-microsoft/MCP-Platform`             | Productivity             | Remote | `https://agent365.svc.cloud.microsoft/agents/tenants/{tenant_id}/servers/mcp_MailTools`            | Mail creation, replies, updates, delete, and search.                                             |
| Microsoft 365 User                      | `bap-microsoft/MCP-Platform`             | Productivity             | Remote | `https://agent365.svc.cloud.microsoft/agents/tenants/{tenant_id}/servers/mcp_MeServer`             | User, manager, team, and reportee details.                                                       |
| ⚙️ Microsoft Admin Center               | `bap-microsoft/MCP-Platform`             | Productivity             | Remote | `https://agent365.svc.cloud.microsoft/agents/tenants/{tenant_id}/servers/mcp_AdminTools`           | Admin Center actions and admin-facing APIs.                                                      |
| Microsoft Clarity                       | `microsoft/clarity-mcp-server`           | Data and analytics       | Local  | Local server                                                                                       | Clarity analytics export access for MCP clients.                                                 |
| Microsoft Dataverse                     | Microsoft Dataverse docs                 | Data and analytics       | Local  | Local server                                                                                       | Discover tables, query data, and update business records.                                        |
| Microsoft Dev Box                       | `@microsoft/devbox-mcp`                  | Developer tools          | Local  | npm package                                                                                        | Dev Box management and environment operations.                                                   |
| Microsoft Fabric (Public Preview)       | `microsoft/mcp`                          | Data and analytics       | Local  | Local-first                                                                                        | Fabric APIs, item definitions, and best-practice guidance without a live connection requirement. |
| Microsoft Fabric Real-Time Intelligence | RTI MCP Server docs                      | Data and analytics       | Local  | Local server                                                                                       | RTI querying and analysis workflows.                                                             |
| Microsoft Learn                         | `microsoftdocs/mcp`                      | Productivity             | Remote | `https://learn.microsoft.com/api/mcp`                                                              | Real-time access to official Microsoft documentation.                                            |
| Microsoft Sentinel Data Exploration     | Sentinel docs                            | Security                 | Remote | `https://sentinel.microsoft.com/mcp/data-exploration`                                              | Natural-language exploration of Sentinel data lake content.                                      |
| Microsoft SQL                           | MSSQL MCP Server docs                    | Developer tools          | Local  | Local server                                                                                       | Conversational SQL schema, table, and CRUD workflows.                                            |
| Microsoft Teams                         | `bap-microsoft/MCP-Platform`             | Productivity             | Remote | `https://agent365.svc.cloud.microsoft/agents/tenants/{tenant_id}/servers/mcp_TeamsServer`          | Teams chats, channels, users, and messages.                                                      |
| Microsoft Word                          | `bap-microsoft/MCP-Platform`             | Productivity             | Remote | `https://agent365.svc.cloud.microsoft/agents/tenants/{tenant_id}/servers/mcp_WordServer`           | Read, create, and collaborate on Word documents.                                                 |
| NuGet MCP Server                        | `NuGet/Home`                             | Developer tools          | Local  | Local server                                                                                       | NuGet package-management tooling and automation.                                                 |
| OneDrive and SharePoint                 | `bap-microsoft/MCP-Platform`             | Productivity             | Remote | `https://agent365.svc.cloud.microsoft/agents/tenants/{tenant_id}/servers/mcp_ODSPRemoteServer`     | OneDrive and SharePoint file integration.                                                        |
| SharePoint Lists                        | `bap-microsoft/MCP-Platform`             | Productivity             | Remote | `https://agent365.svc.cloud.microsoft/agents/tenants/{tenant_id}/servers/mcp_SharePointListsTools` | SharePoint list, library, and collaboration workflows.                                           |
| Playwright                              | `microsoft/playwright-mcp`               | Developer tools          | Local  | Local server                                                                                       | Structured browser interaction through accessibility snapshots.                                  |
| Wassette                                | `microsoft/wassette`                     | Developer tools          | Local  | Local server                                                                                       | Security-oriented WebAssembly component runtime via MCP.                                         |

### Emoji note

I copied the visible icon-style markers Microsoft uses in the catalog headings where they are explicit in the page markup:

- `✨` Microsoft Foundry
- `☸️` Azure Kubernetes Service (AKS)
- `⚙️` Microsoft Admin Center

Other entries in the GitHub-rendered page appear without a distinctive emoji marker in the captured listing, so I did not invent extra ones.

### Evidence note

The `microsoft/mcp` page mixes true Azure services, Microsoft 365/Graph endpoints, Fabric/data tooling, and general developer tooling. Do not assume every entry is Azure-scoped just because it appears in Microsoft's MCP catalog.

## Security notes

RBAC is the blast-radius boundary. Use least-privilege roles, narrow scopes, and time-bound access where possible.
