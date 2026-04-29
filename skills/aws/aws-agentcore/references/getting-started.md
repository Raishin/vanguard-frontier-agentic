# Getting Started with AgentCore

> Version note: AgentCore tooling is evolving. Verify exact CLI syntax against the installed toolkit and current official AWS docs before production use. Do not paste secrets into commands or files.

## First: classify the path

Do not blur these together:

1. **New project** — use the recommended npm CLI `@aws/agentcore`
2. **Existing code-based agent** — adapt/configure what exists; do not scaffold over it
3. **Import/migration from Amazon Bedrock Agents** — use the import flow if the goal is migration, not greenfield creation
4. **Managed harness** — a config-based path that current AWS docs describe as preview

If you pick the wrong path, your advice becomes cargo cult.

## Prerequisites

- Node.js **20 or later**
- npm
- AWS credentials configured through AWS CLI, environment variables, or a named profile
- Python **3.10 or later** for agent code
- IAM permissions to call AgentCore APIs and assume the CDK bootstrap roles used during deployment

Install the current recommended CLI for new projects:

```bash
npm install -g @aws/agentcore
```

Use the Python `bedrock-agentcore-starter-toolkit` only for legacy Python-based workflows or migration-era examples.

## Current local CLI surface to assume only after verification

Verified locally in this environment:

- `agentcore create`
- `agentcore dev`
- `agentcore deploy`
- `agentcore invoke`
- `agentcore status`
- `agentcore destroy`
- `agentcore stop-session`
- `agentcore configure`
- `agentcore identity`
- `agentcore gateway`
- `agentcore memory`
- `agentcore obs`
- `agentcore policy`
- `agentcore eval`

Do not promise undocumented or unverified commands just because a README or blog post mentioned them.

## New project path

Use `agentcore create` only when the user actually wants a new project.

```bash
agentcore create --non-interactive --project-name MyAgent
```

Useful flags verified locally:

- `--template basic|production`
- `--agent-framework Strands|LangChain_LangGraph|GoogleADK|OpenAIAgents|AutoGen|CrewAI`
- `--model-provider Bedrock|OpenAI|Anthropic|Gemini`
- `--iac CDK|Terraform`
- `--memory STM_ONLY|STM_AND_LTM|NO_MEMORY`
- `--venv` / `--no-venv`

Key assumption many people miss:

- For non-Bedrock providers, official harness docs say you need an **API key ARN**, not just “some secret somewhere”.

## Existing code-based agent path

If the agent already exists, do **not** run `agentcore create` on top of it.

Start by inspecting configuration:

```bash
agentcore configure --help
```

Current official docs say project configuration lives under:

- `agentcore/agentcore.json`
- `agentcore/aws-targets.json`

So stop assuming old one-file layouts are the only truth.

## Import / migration path

If the user is migrating an existing Amazon Bedrock Agent, treat that as a separate path.

Local CLI help currently shows import under `create`:

```bash
agentcore create import --help
```

Starter-toolkit docs and Context7 also reference `import-agent` style flows. That mismatch means you must verify the installed CLI before giving automation steps.

## Local development loop

Run the dev server:

```bash
agentcore dev
```

Useful verified flags:

- `--port`
- `--env KEY=VALUE`

Then test locally:

```bash
agentcore invoke --dev '{"prompt":"Hello"}'
```

Useful verified flags:

- `--agent`
- `--session-id`
- `--bearer-token`
- `--local`
- `--dev`
- `--port`
- `--user-id`
- `--headers`

## Harness-specific cautions

Official AWS docs describe:

- a **code-based agent** path
- a **managed harness** path

These are not the same thing. Current AWS docs describe the harness as preview. Do not casually recommend harness-only features as if they are universal AgentCore behavior.

Also:

- harness skills are **filesystem paths inside the runtime environment**
- `--skill-path` references a path; it does **not** upload the skill for you
- some harness docs mention `agentcore invoke --exec` shell access, but installed CLIs may not expose that surface the same way yet

## Security-critical realities

- SigV4 authentication does **not** give per-user identity propagation into downstream tool calls the way the OAuth bearer-token path can.
- Gateway security is not “create gateway and done”; policy design is a separate concern.
- Custom observability requires more than default metrics; official docs call out ADOT and CloudWatch prerequisites for richer traces.

## Minimum safe workflow

1. Verify CLI help locally
2. Classify new project vs existing agent vs import vs harness
3. Confirm region and feature support
4. Confirm IAM/bootstrap permissions
5. Create or adapt minimally
6. Run `agentcore dev`
7. Test with `agentcore invoke --dev`
8. Only then discuss deploy, Memory, Gateway, Policy, Browser, or Code Interpreter wiring
