# Official sources

Use this reference when grounding current Amazon Bedrock AgentCore behavior.

## Amazon Bedrock AgentCore

- Get started with Amazon Bedrock AgentCore CLI
  https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/agentcore-get-started-cli.md
- Available interfaces for using Amazon Bedrock AgentCore
  https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/develop-agents.html
- AgentCore harness overview
  https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/harness.html
- Get started with the harness
  https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/harness-get-started.html
- Environment and Skills
  https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/harness-environment.html
- Security and access control
  https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/harness-security.html
- What is Amazon Bedrock AgentCore
  https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/what-is-bedrock-agentcore.html
- Runtime getting started
  https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/runtime-get-started.html
- AgentCore Memory
  https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/memory.html
- AgentCore Gateway
  https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/gateway.html
- AgentCore Identity
  https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/identity.html
- AgentCore Observability
  https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/observability-configure.html
- AgentCore generated observability data
  https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/observability-service-provided.html
- Browser tool
  https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/browser-tool.html
- Code Interpreter
  https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/code-interpreter.html
- AgentCore Evaluations
  https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/evaluations.html
- AgentCore Registry
  https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/registry.html
- AgentCore Payments
  https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/payments.html
- AgentCore tools configuration
  https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/harness-tools.html
- Runtime file system configurations
  https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/runtime-filesystem-configurations.html
- Runtime custom header passthrough
  https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/runtime-header-allowlist.html
- Gateway MCP sessions
  https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/gateway-sessions.html
- Gateway MCP elicitation
  https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/gateway-mcp-elicitation.html
- Gateway MCP sampling
  https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/gateway-mcp-sampling.html
- Gateway MCP progress notifications
  https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/gateway-mcp-progress.html
- Gateway MCP logging notifications
  https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/gateway-mcp-logging.html
- Long-term memory metadata filtering
  https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/long-term-memory-metadata.html
- Policy in AgentCore
  https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/policy.html
- Create a policy
  https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/policy-create-policies.html
- Core concepts for policy
  https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/policy-core-concepts.html
- Harness operations
  https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/harness-operations.html

## Grounded notes from Context7

- Current AWS devguide guidance says the recommended CLI install for new projects is `npm install -g @aws/agentcore`.
- The AgentCore CLI is the Node.js command-line tool for creating, configuring, deploying, and managing agents and uses project JSON config under the `agentcore/` directory, including `agentcore.json` and `aws-targets.json`.
- AgentCore tool configuration can include remote MCP servers, AgentCore Gateway, Browser, Code Interpreter, and inline functions.
- For managed credential rotation and API key storage, prefer AgentCore Gateway and AgentCore Identity over raw authentication headers.
- AgentCore Observability uses CloudWatch metrics for runtime, memory, gateway, built-in tools, and identity resources; custom runtime metrics need instrumentation such as AWS Distro for OpenTelemetry.
- AgentCore offers two distinct paths: code-based agents and the managed harness. Current AWS docs describe the code-based agent path as generally available and the config-based harness path as preview.
- Skills referenced with `--skill-path` must already exist inside the runtime container or session environment. The path reference does not upload the skill for you.
- Gateway creation is not the end of the security model. Official policy docs show Cedar-based policy enforcement is a separate control plane you must design explicitly.
- Harness security docs say SigV4 callers do not get per-user identity propagation into downstream tools. User-scoped token vault and on-behalf-of flows require the OAuth bearer-token path.
- Harness overview docs note regional rollout is ongoing and the currently documented harness availability is limited; do not assume every AgentCore feature is live in every region.
- Official harness docs describe `agentcore invoke --exec` shell access to the session environment, but installed CLI surfaces can drift. Verify the local CLI before promising that workflow.

## Current release-note deltas to account for

The following items were missing or underrepresented in earlier skill guidance and must now be considered before giving implementation advice:

- **Core service map expanded** — current overview docs list Runtime, Harness, Memory, Gateway, Identity, Code Interpreter, Browser, Observability, Payments, Evaluations, Policy, and Registry as AgentCore services/capabilities.
- **Gateway MCP is now stateful and more interactive** — release notes describe MCP sessions, response streaming, elicitation pass-through, sampling messages, progress notifications, and logging notifications. Gateway advice must cover session scope, timeouts, human-in-the-loop prompts, client streaming behavior, and audit/log handling.
- **Runtime and Harness support attached filesystems** — release notes describe Amazon S3 Files and Amazon EFS access point mounts for sharing skills, prompt templates, datasets, and intermediate outputs. Treat mounts as data-perimeter and lifecycle risks, not just convenience.
- **Agent quality loop is first-class** — release notes describe optimization, batch evaluation, user simulation, and A/B testing. Do not treat evaluation as an external afterthought when the user asks for production readiness.
- **Payments are preview** — payment flows add wallet, spending-limit, x402 endpoint, and transaction-governance risks. Keep this separate from GA runtime/gateway guidance.
- **Node.js direct code deployment exists** — do not imply Python/container-only deployment paths.
- **VPC egress and OBO token exchange exist** — Identity, Gateway, and Runtime can reach private resources; user-scoped access can use on-behalf-of token exchange. Verify network and identity boundaries instead of assuming public egress or raw static credentials.
- **Memory supports structured metadata filtering** — namespace design is no longer enough; indexed metadata keys and retrieval filters can materially change isolation and relevance.
- **Observability requires setup** — current docs call out CloudWatch Transaction Search setup, tracing enablement for memory resources, consistent session IDs, distributed tracing, custom attributes, resource usage monitoring, and alerts.
- **AgentCore Registry is in preview** — registry-based discovery and governance can reduce sprawl but introduces approval, publication, semantic-search, and access-control decisions.
- **AgentCore MCP server exists in awslabs/mcp** — coding agents may operate AgentCore resources through a user-configured AWS credential chain. Treat this as live AWS access requiring the user's read-only or explicitly approved profile.

## Starter toolkit caveat

- The Bedrock AgentCore Starter Toolkit repository is still useful for migration and existing Python-based workflows.
- Based on current official AWS devguide evidence, do not present the starter toolkit as the recommended starting point for new projects when the newer `@aws/agentcore` CLI is available.

## Current MCP/documentation refresh (2026-06-01)

Evidence used for this refresh:

- `AwsDocumentationMcpServer` search surfaced current Amazon Bedrock pages for server-side tool use with AgentCore Gateway and Agent Toolkit skill usage. Keep AgentCore guidance grounded in the current docs URLs below instead of training-memory CLI syntax.
- AWS Agent Toolkit documentation confirms skills can be bundled, installed locally, or discovered at runtime through AWS MCP Server, and that agents should load `SKILL.md` plus references progressively. This reinforces this skill's lazy-reference pattern.
- AWS managed MCP regional availability queries returned `isAvailableIn` for `Amazon Bedrock AgentCore`, `Amazon Bedrock`, `Amazon CloudWatch`, and `AWS IAM Identity Center` in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.
- AWS managed MCP API availability also surfaced AgentCore operations for Memory records/extraction, runtime invocation/commands, Browser, Code Interpreter, Evaluations, A/B tests, recommendations, workload access tokens, and payment operations. Some payment operations were visible only in `us-east-1` and `us-west-2` in the sampled regions. Treat this as live availability evidence, not a substitute for the user's account/Region validation.

Best-practice implication: keep AgentCore Gateway, Identity, Memory, Browser, Code Interpreter, Evaluations, Registry, Payments, filesystem, and harness advice in references, and re-query docs/tooling before promising exact CLI flags, API coverage, maturity status, or regional support.

## MCP grounding

- Use `AwsDocumentationMcpServer.search_documentation` to find current AWS documentation, then `read_documentation` or `read_sections` for the cited behavior. Use `recommend` when newly released or adjacent service guidance may matter.
- Use an AWS managed MCP server through the user's configured read-only AWS profile for exposed read-only live evidence such as AWS Regions or regional service/resource availability. Treat that evidence as complementary to documentation, not as proof of the user's account configuration and never as permission to mutate AWS state.

## Grounding rule

Docs explain service behavior. They do not prove the user's installed CLI version, active AWS account, IAM role, Region support, quotas, deployed AgentCore resources, or whether preview-only features are enabled in that account.
