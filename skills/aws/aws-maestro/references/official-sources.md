# Official sources

Use this reference when grounding a routing decision in AWS service documentation or verifying service-specific behavior.

## AWS general documentation

- https://docs.aws.amazon.com/
- https://docs.aws.amazon.com/wellarchitected/latest/framework/welcome.html
- https://docs.aws.amazon.com/wellarchitected/latest/framework/definitions.html

## Bedrock and AgentCore

- https://docs.aws.amazon.com/bedrock/latest/userguide/what-is-bedrock.html
- https://docs.aws.amazon.com/bedrock/latest/userguide/agents.html
- https://docs.aws.amazon.com/bedrock/latest/userguide/agentcore.html
- https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/what-is-bedrock-agentcore.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, or operational state. Use documentation to ground routing decisions and specialist selection, not to assert the user's live AWS state. Always prefer user-provided sanitized evidence or read-only discovery when available.

## Using documentation for routing

When a user describes a service or scenario and you are unsure which domain or specialist to select, consult the relevant AWS service documentation to confirm the service category before dispatching. Do not dispatch on a guess. If the domain is ambiguous after checking documentation, ask the user one clarifying question before routing.
