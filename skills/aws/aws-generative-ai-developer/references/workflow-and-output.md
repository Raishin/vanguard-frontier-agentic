# Workflow and output contract

Use this reference for full AWS generative AI application design, architecture review, or implementation guidance.

## Workflow

1. **Classify the workload**
   - Chat API or assistant backend
   - Retrieval-augmented generation flow
   - Async document or event-driven pipeline
   - Bedrock app with Guardrails and policy concerns
   - AgentCore runtime versus non-AgentCore application logic

2. **Enforce serverless-first architecture**
   - Start with Bedrock managed capabilities plus Lambda, API Gateway, Step Functions, EventBridge, S3, DynamoDB, SQS, SNS, and Cognito.
   - Only move to ECS, EKS, or EC2 if the user provides a concrete serverless blocker such as runtime incompatibility, long-lived GPU needs, or explicit platform constraints.
   - If a non-serverless path is chosen, call out why the serverless path was rejected.

3. **Review the application shape**
   - ingress/auth path
   - prompt / orchestration path
   - retrieval / data path
   - asynchronous work and failure handling
   - Guardrails / safety / policy path
   - observability / tracing / cost controls

4. **Validate**
   - Confirm IAM boundaries, prompt-injection defenses, logging, retry behavior, quotas, cost guardrails, and rollback path.
   - Distinguish documentation-based patterns from live deployed evidence.

## Output contract

Return:

1. Workload classification
2. Evidence level and current unknowns
3. Serverless-first architecture recommendation
4. Main risks / blockers
5. Safe next actions
6. Validation and rollback path
