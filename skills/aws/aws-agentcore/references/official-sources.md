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
- Browser tool
  https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/browser-tool.html
- Code Interpreter
  https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/code-interpreter.html
- AgentCore tools configuration
  https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/harness-tools.html
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

## Starter toolkit caveat

- The Bedrock AgentCore Starter Toolkit repository is still useful for migration and existing Python-based workflows.
- Based on current official AWS devguide evidence, do not present the starter toolkit as the recommended starting point for new projects when the newer `@aws/agentcore` CLI is available.

## Grounding rule

Docs explain service behavior. They do not prove the user's installed CLI version, active AWS account, IAM role, Region support, quotas, deployed AgentCore resources, or whether preview-only features are enabled in that account.
