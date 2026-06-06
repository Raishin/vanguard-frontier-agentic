# Workflow and output contract

Use this reference for full AgentCore implementation, deployment, or production-readiness work.

## Workflow

1. **Classify the path**
   - Existing agent adaptation
   - New project scaffold with the recommended npm CLI
   - Import/migration from an existing Amazon Bedrock Agent
   - Code-based agent versus config-based harness
   - Runtime/local invoke loop
   - Environment/skills path loading
   - Memory integration
   - Gateway/MCP/tool integration
   - Identity/credential flow
   - Observability/instrumentation
   - Browser or Code Interpreter tool usage
   - Evaluations, batch evaluations, user simulation, optimization, or A/B testing
   - Registry publishing/discovery/governance
   - Payments / x402 / wallet-backed agent transactions
   - Runtime or harness filesystem mounts
   - Policy/Cedar controls for Gateway tool access

2. **Verify tooling and docs**
   - Check installed AgentCore CLI/toolkit version when local commands will run.
   - Check whether the user is on a code-based agent path or the harness preview path.
   - For new projects, default to the npm CLI package `@aws/agentcore` unless the user is intentionally staying on an older Python toolkit workflow.
   - Prefer `AwsDocumentationMcpServer` for current official docs, then Context7 or configured AgentCore MCP documentation tools.
   - If AWS docs and starter-toolkit examples disagree, prefer the newer AWS devguide wording and call out the mismatch.
   - Treat bundled commands as examples until verified against local tooling.
   - Confirm region/API support before recommending AgentCore Runtime, Harness, Gateway, Memory, Evaluations, Browser, Code Interpreter, Payments, Registry, or Policy workflows.

3. **Implement minimally**
   - Existing agents: wrap/adapt runtime entrypoint; do not scaffold over code.
   - New projects: prefer non-interactive generation with the recommended CLI only when project creation is requested.
   - Skills: treat `--skill-path` as a container/session filesystem reference, not a skill uploader.
   - Filesystems: treat S3 Files/EFS mounts as data-perimeter resources with explicit path, lifecycle, and tenant-isolation decisions.
   - Memory/Gateway/Identity: create and test resources separately before wiring production flows.
   - Gateway: pair tool onboarding with policy, identity, session, streaming, elicitation, sampling, progress, and logging decisions; do not stop at endpoint creation.
   - Harness: call out preview status and identity propagation caveats if you recommend it.
   - Evaluations: use batch evaluation/user simulation/A-B validation for quality-sensitive production changes.
   - Registry: require publication, approval, access, and discovery governance before recommending broad catalog rollout.
   - Payments: treat as preview; require spending limits, wallet/provider choice, transaction audit, and rollback/disable path.

4. **Validate**
   - Run local invoke/test loop.
   - Confirm IAM role, network mode, environment variables, package dependencies, logs, metrics, tracing prerequisites, and rollback path.
   - Confirm which commands actually exist in the installed CLI before giving automation steps.
   - Confirm CloudWatch Transaction Search or equivalent telemetry destination if the recommendation relies on AgentCore observability.

## Output contract

Return:

1. AgentCore component in scope
2. Evidence level and current unknowns
3. Minimal implementation plan
4. Commands/code to run or verify
5. Security caveats
6. Rollback or cleanup path
7. Feature maturity caveat if the answer touches preview-only harness behavior
