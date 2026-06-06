# Safety checklist

Use before AgentCore deployment, tool exposure, Memory/Gateway changes, Identity integration, Browser/Code Interpreter enablement, or production recommendations.

## Non-negotiables

- Do not ask for or print AWS credentials, API keys, OAuth client secrets, access tokens, private keys, account IDs, customer data, or tenant identifiers.
- Do not hardcode credentials into MCP headers, agent code, environment files, or reference examples.
- Do not tell users the Python starter toolkit is the preferred new-project path when current official AWS docs recommend the npm CLI `@aws/agentcore`.
- Prefer AgentCore Gateway and AgentCore Identity for managed credential handling where applicable.
- Keep action/tool permissions least-privilege and scoped to the task.
- Confirm logging, metrics, tracing, and CloudWatch visibility before production rollout.
- Confirm CloudWatch Transaction Search / trace destination setup when relying on AgentCore observability.
- Confirm whether the answer relies on code-based agent GA behavior or harness preview behavior.
- Confirm whether the target region supports the specific AgentCore capability being recommended.
- Require explicit approval before deployment, exposing tools, enabling browser/code execution, attaching filesystems, enabling payment flows, or changing memory persistence.

## Component risks

- **Runtime:** wrong entrypoint, broad execution role, public network mode, missing logs, unbounded session lifetime.
- **Memory:** PII retention, namespace leakage, actor/session mixups, metadata-filter bypass, memory poisoning, untested deletion/expiry behavior.
- **Gateway/MCP tools:** overbroad tool access, raw bearer tokens, unsafe OpenAPI targets, excessive scopes, no tool audit trail, unsafe session reuse, ungoverned elicitation/sampling/streaming/log notifications.
- **Identity:** unmanaged secrets, unclear credential provider, missing rotation, broad OAuth scopes, assuming SigV4 preserves end-user identity when official docs say it does not.
- **Environment/skills/filesystems:** assuming `--skill-path` uploads local folders, forgetting ECR/VPC/container prerequisites, overbroad S3 Files/EFS mounts, path collisions, or binding nonexistent in-container paths.
- **Policy:** creating Gateway tools without Cedar policy boundaries or clear principal/resource conditions.
- **Browser:** browsing sensitive sites, recording exposure, egress risk, uncontrolled side effects.
- **Code Interpreter:** data exfiltration, untrusted code, package/network risk, missing sandbox boundaries.
- **Evaluations/optimization:** optimizing against unreviewed production traces, leaking prompt/tool data into eval datasets, promoting A/B winners without guardrail review.
- **Registry:** publishing unreviewed agents/tools/skills, semantic search surfacing sensitive tools, unclear approval workflow.
- **Payments:** preview maturity, wallet custody, spending limits, x402 endpoint trust, transaction auditability, refund/dispute handling.

## Evidence labels

Use `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's live AgentCore deployment.
