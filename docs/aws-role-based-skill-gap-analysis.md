# AWS Role-Based Skill Gap Analysis

Date: 2026-04-29  
Branch: `feat/aws-role-based-skills`  
Scope: `skills/aws/**` role-based skill portfolio

## Executive verdict

Status: **GOOD FOUNDATION, NOT COMPLETE**.

The current AWS portfolio covers architecture, landing zone, IAM, network, EKS,
serverless production readiness, security posture, observability/incident
response, cost, resilience/BCDR, backup/data protection, and migration. That is
not bad. But if this is meant to compete with an AWS-grade agentic marketplace,
it is still missing several high-value operator/developer roles.

The dangerous assumption is: **"role-based" means broad cloud job titles only.**
That is wrong. The AWS DevOps Agent documentation shows skills should also map to
**investigation scenarios, tool-use patterns, and operational failure modes**.
A role portfolio without scenario-level depth will look polished and fail during
real incidents.

## Sources used

### Primary AWS sources

- AWS DevOps Agent Skills: <https://docs.aws.amazon.com/devopsagent/latest/userguide/about-aws-devops-agent-devops-agent-skills.html>
- AWS DevOps Agent overview: <https://docs.aws.amazon.com/devopsagent/latest/userguide/about-aws-devops-agent.html>
- AWS DevOps Agent learned skills: <https://docs.aws.amazon.com/devopsagent/latest/userguide/about-aws-devops-agent-learned-skills.html>
- AWS CDK best practices: <https://docs.aws.amazon.com/cdk/v2/guide/best-practices.html>
- AWS CloudFormation best practices: <https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/best-practices.html>
- CloudFormation drift-aware change sets: <https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/drift-aware-change-sets.html>
- Amazon RDS best practices: <https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/CHAP_BestPractices.html>
- Amazon RDS Performance Insights: <https://docs.aws.amazon.com/prescriptive-guidance/latest/amazon-rds-monitoring-alerting/performance-insights-tools.html>
- Amazon ECS deployment failure detection: <https://docs.aws.amazon.com/AmazonECS/latest/developerguide/deployment-failure-detection.html>
- Amazon ECS IAM role best practices: <https://docs.aws.amazon.com/AmazonECS/latest/developerguide/security-iam-roles.html>
- Amazon S3 security best practices: <https://docs.aws.amazon.com/AmazonS3/latest/userguide/security-best-practices.html>
- Amazon S3 Block Public Access: <https://docs.aws.amazon.com/AmazonS3/latest/userguide/access-control-block-public-access.html>
- DynamoDB best practices: <https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/best-practices.html>
- EventBridge event pattern best practices: <https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-patterns-best-practices.html>
- EventBridge rule best practices: <https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-rules-best-practices.html>
- EventBridge monitoring best practices: <https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-monitoring-events-best-practices.html>
- Amazon Bedrock agent security best practices: <https://docs.aws.amazon.com/bedrock/latest/userguide/security-best-practice-agents.html>
- Amazon Bedrock prompt injection security: <https://docs.aws.amazon.com/bedrock/latest/userguide/prompt-injection.html>
- Amazon Bedrock Guardrails: <https://docs.aws.amazon.com/bedrock/latest/userguide/guardrails.html>

### Skills ecosystem sources

- skills.sh AWS search: <https://skills.sh/?q=aws>
- skills.sh `aws-cdk-python-setup`: <https://skills.sh/github/awesome-copilot/aws-cdk-python-setup>
- skills.sh `aws-serverless`: <https://skills.sh/sickn33/antigravity-awesome-skills/aws-serverless>

Important correction: **skills.sh is not AWS official documentation**. It is a
public skills directory. Treat it as marketplace signal, not authority. The
AWS-official material found during this review is AWS DevOps Agent documentation,
which describes how AWS expects skills to be structured and used inside AWS
DevOps Agent.

## Current AWS portfolio inventory

Existing AWS skills as of this review:

1. `aws-cost-optimization-governor`
2. `aws-data-protection-backup-steward`
3. `aws-eks-platform-operator`
4. `aws-iam-least-privilege-review`
5. `aws-landing-zone-governor`
6. `aws-migration-cutover-architect`
7. `aws-network-architect`
8. `aws-observability-incident-responder`
9. `aws-resilience-bcdr-review`
10. `aws-security-posture-hardening`
11. `aws-serverless-production-readiness`
12. `aws-solution-architect`

## Adversarial board critique

### 1. The portfolio is infra/security-heavy and app/data-light

You have strong coverage for cloud platform review, but weaker coverage for
where production incidents usually become expensive: databases, message flows,
stateful data stores, CI/CD, IaC drift, and application release behavior.

### 2. You are mixing broad roles and scenario skills

`aws-observability-incident-responder` is broad. AWS DevOps Agent docs show an
example `rds-performance-investigation` skill with specific metrics,
thresholds, and Performance Insights steps. That is a clue: not every useful
skill should be a big role. Some must be scenario-specific enough to trigger on
real incidents.

### 3. Serverless readiness is too broad to replace implementation guidance

The marketplace signal from skills.sh includes `aws-serverless`, Lambda language
integration skills, and CDK/SAM patterns. Your current skill can review
serverless design, but it does not teach concrete Lambda/API Gateway/SQS/SAM/CDK
implementation patterns. That is a different job.

### 4. Missing IaC skill is a major hole

For a repo that distributes cloud skills, not having AWS CDK/CloudFormation/IaC
review is a serious omission. AWS CDK and CloudFormation docs have enough
specific failure modes: stack lifecycle boundaries, construct design, change
sets, drift detection, rollback triggers, stack policies, Guard, and secrets in
templates.

### 5. Missing GenAI/Bedrock skill is strategically bad

This is an agentic marketplace. AWS Bedrock, Bedrock Agents, Guardrails,
prompt-injection controls, AgentCore memory, and least-privilege agent access
should be first-class. If you skip this, the AWS portfolio looks like old cloud
ops with no agentic edge.

## Missing skill candidates

Legend:

- **P0**: should add before calling AWS portfolio strong.
- **P1**: important, add after P0.
- **P2**: useful, but only if you can keep it distinct and evidence-backed.
- **Reject/merge**: do not create a standalone skill unless the scope becomes clearer.

| Priority | Candidate skill ID | Role / scenario | Why it is missing | Why not covered by existing skills | Primary source grounding | Recommendation |
|---|---|---|---|---|---|---|
| P0 | `aws-iac-change-safety-review` | CDK/CloudFormation/Terraform change reviewer | IaC changes are where privilege, network, and data-loss risk enter production. Current skills review domains, not IaC mechanics. | `aws-solution-architect` is too high level; IAM/network skills only catch part of the risk. | AWS CDK best practices; CloudFormation best practices; drift-aware change sets | **Create** |
| P0 | `aws-rds-aurora-performance-investigator` | RDS/Aurora incident and performance investigator | AWS DevOps Agent official example is literally RDS performance investigation. Missing this ignores AWS's own skill design signal. | Observability skill is broad and won't reliably know RDS metrics, Performance Insights, replica lag, connection exhaustion, or slow query workflow. | AWS DevOps Agent skills example; RDS best practices; Performance Insights docs | **Create** |
| P0 | `aws-bedrock-agent-security-governor` | Bedrock/AgentCore/Guardrails security reviewer | Agentic marketplace needs AWS-native AI safety and least-privilege agent design. | Security posture skill is general CSPM; it does not cover prompt injection, guardrails, knowledge bases, action groups, memory poisoning, or model governance. | Bedrock agent security, prompt injection, Guardrails docs | **Create** |
| P0 | `aws-ecs-fargate-platform-operator` | ECS/Fargate service operator | EKS exists, but ECS/Fargate is still a dominant AWS container platform. | EKS skill does not cover task roles, service deployments, circuit breaker, capacity providers, Fargate networking, or ECS blue/green semantics. | ECS deployment failure detection; ECS IAM roles; ECS blue/green docs | **Create** |
| P1 | `aws-event-driven-architecture-review` | EventBridge/SNS/SQS/Step Functions event-flow architect | Event-driven failure modes are subtle: loops, retries, DLQs, idempotency, schema drift, cross-account event bus policy, and duplicate delivery. | Serverless readiness mentions event sources but does not own event bus architecture. | EventBridge pattern/rule/monitoring docs; SNS/SQS/EventBridge decision guide | **Create** |
| P1 | `aws-s3-data-perimeter-governor` | S3/storage security and data perimeter reviewer | S3 is the classic data-exposure blast radius. Security posture is too broad for bucket policy, Block Public Access, Object Ownership, encryption, access points, Storage Lens, lifecycle, and data perimeter checks. | IAM skill covers policies, backup skill covers recovery; neither owns S3 data exposure and governance end-to-end. | S3 security best practices; S3 Block Public Access | **Create** |
| P1 | `aws-dynamodb-data-modeling-performance-review` | DynamoDB data model and performance reviewer | DynamoDB failures often come from partition key, GSI, scan/query, hot partition, and capacity design mistakes. | No current data-modeling skill. Serverless readiness only touches DynamoDB as a dependency. | DynamoDB best practices | **Create** |
| P1 | `aws-ci-cd-release-engineer` | AWS deployment pipeline and release safety reviewer | AWS DevOps Agent explicitly correlates deployments, code repositories, CI/CD pipelines, incidents, and prevention recommendations. | Observability skill investigates incidents; no skill owns pipeline gates, rollback, deployment evidence, CodePipeline/GitHub Actions/GitLab, or release risk. | AWS DevOps Agent overview; proactive incident prevention docs | **Create** |
| P1 | `aws-devops-agent-skill-designer` | AWS DevOps Agent skill and learned-skill author | AWS official docs define skill frontmatter, targeting, learned skills, tool-use best practices, and investigation workflow design. | Meta-skill not covered by cloud ops skills. Relevant because this repo is an agentic skills marketplace. | AWS DevOps Agent Skills; Learned Skills docs | **Create if marketplace wants AWS DevOps Agent compatibility** |
| P1 | `aws-ec2-compute-operations-steward` | EC2/ASG/AMI/patch/SSM operational reviewer | EC2 remains core for legacy, stateful, and migration workloads. Missing compute ops creates gap between network/security/resilience and actual hosts. | Solution architect is broad; migration skill covers cutover, not day-2 compute hygiene. | EC2, Auto Scaling, Systems Manager, EBS, patching docs needed | **Create after source pass** |
| P2 | `aws-api-edge-delivery-review` | API Gateway/ALB/CloudFront/WAF edge/API reviewer | Public entry points are high-risk and need auth, throttling, TLS, WAF, logging, caching, origin protection, and rollback review. | Network skill covers topology; serverless covers Lambda APIs; neither owns edge/API contract. | API Gateway, CloudFront, WAF docs needed | **Create if users ask API/edge often** |
| P2 | `aws-kms-secrets-cryptography-steward` | KMS/Secrets Manager/Parameter Store crypto and secret lifecycle reviewer | KMS mistakes break recovery and create exposure. Could be very valuable. | IAM skill mentions KMS/secrets risk, but not key policy lifecycle, grants, rotation, multi-Region keys, secret rotation, and break-glass. | KMS and Secrets Manager docs needed | **Maybe create; risk of overlap with IAM** |
| P2 | `aws-compliance-evidence-mapper` | Audit evidence and control mapping reviewer | Useful for SOC2/PCI/NIST audit workflows, especially with Security Hub, Config, CloudTrail, Backup, IAM. | Security posture hardening is technical; audit evidence packaging is a different output. | Security Hub controls, Artifact, Config, Audit Manager docs needed | **Create only if output contract is audit-ready evidence packs** |
| P2 | `aws-multi-cloud-connectivity-review` | Hybrid/multi-cloud networking and identity boundary review | Useful for enterprises, but can become vague quickly. | Network architect partly covers hybrid routing. | Direct Connect, VPN, TGW, Route 53 Resolver docs needed | **Merge into network unless repeated demand emerges** |
| Reject/merge | `aws-general-developer` | Generic AWS developer helper | Too broad and likely low signal. | Covered by solution/serverless/IaC candidates. | N/A | **Do not create** |
| Reject/merge | `aws-lambda-python-integration`, `aws-lambda-typescript-integration` | Language-specific Lambda coding helpers | skills.sh shows these exist, but duplicating every language causes bloat. | Better as references under `aws-serverless-implementation-patterns` if needed. | skills.sh community signal only | **Do not create yet** |

## P0 skills: detailed creation specs

### 1. `aws-iac-change-safety-review`

Purpose: review AWS IaC changes before deployment.

Triggers:

- "review this CDK/CloudFormation/SAM/Terraform change"
- "is this change set safe?"
- "why will this stack replace resources?"
- "check drift before deploy"
- "validate this template/policy before release"

Required behavior:

1. Identify IaC engine: CDK, CloudFormation, SAM, Terraform, Serverless Framework, or mixed.
2. Identify deployment scope: account, Region, stack/workspace, environment, production impact.
3. Inspect generated artifact where applicable: `cdk synth`, CloudFormation template, SAM template, Terraform plan, change set.
4. Flag replacements, deletes, broad IAM, public exposure, security group changes, KMS/secrets changes, data-store changes, and logging/backup removal.
5. Require drift/change-set evidence before production execution.
6. Return validation commands and rollback plan.

Minimum references:

- AWS CDK best practices
- CloudFormation best practices
- CloudFormation drift-aware change sets
- CloudFormation drift detection

Why P0: This catches the highest-risk operational changes before they become
incidents. Without it, role skills advise architecture but do not gate actual
change execution.

### 2. `aws-rds-aurora-performance-investigator`

Purpose: investigate RDS/Aurora latency, connection exhaustion, storage pressure,
replica lag, failover, and slow queries.

Triggers:

- "RDS latency is high"
- "Aurora queries are slow"
- "database connections exhausted"
- "replica lag increased"
- "Performance Insights shows high DB load"

Required behavior:

1. Confirm engine, instance/cluster, Region, writer/reader role, and impact window.
2. Check CloudWatch alarms and metrics: CPU, connections, read/write latency, free storage, queue depth, replica lag.
3. Use Performance Insights when available to inspect DB load, top waits, and top SQL.
4. Separate database tuning from application connection-pool bugs.
5. Avoid knee-jerk instance upsizing until evidence distinguishes CPU, I/O, lock, query-plan, storage, and connection pressure.
6. Return root-cause hypothesis with evidence, mitigation, rollback, and follow-up prevention.

Minimum references:

- AWS DevOps Agent RDS performance investigation example
- Amazon RDS best practices
- Amazon RDS Performance Insights docs

Why P0: AWS itself uses RDS performance investigation as the canonical complete
skill example. Ignoring it would be negligent.

### 3. `aws-bedrock-agent-security-governor`

Purpose: review Amazon Bedrock agent, guardrail, knowledge base, action group,
memory, and prompt-injection posture.

Triggers:

- "review this Bedrock agent"
- "secure this RAG/agent workflow"
- "do we need Guardrails?"
- "prevent prompt injection"
- "agent action group IAM policy"
- "AgentCore memory poisoning"

Required behavior:

1. Map agent components: model, system prompt, action groups, Lambda/tools, knowledge bases, memory, guardrails, IAM, data sources.
2. Check prompt-injection and prompt-leakage controls.
3. Check guardrail coverage and what content is actually evaluated.
4. Check least privilege for action groups and data access.
5. Check PII handling, encryption, logging, and observability.
6. Return threat model, guardrail gaps, least-privilege remediation, and test cases.

Minimum references:

- Bedrock agent security best practices
- Bedrock prompt injection security
- Bedrock Guardrails docs
- AgentCore best practices if used

Why P0: This repo is agentic. Missing AWS-native agent security makes the AWS
portfolio look outdated.

### 4. `aws-ecs-fargate-platform-operator`

Purpose: review ECS/Fargate service production readiness and day-2 operations.

Triggers:

- "review this ECS service"
- "Fargate deployment failed"
- "ECS blue/green/circuit breaker"
- "task role vs execution role"
- "capacity provider/autoscaling issue"

Required behavior:

1. Confirm launch type, service, task definition, deployment controller, ALB/target groups, capacity provider, and network mode.
2. Check task role vs execution role separation and secret access.
3. Check deployment circuit breaker, rollback, CloudWatch alarms, blue/green, health checks, and EventBridge deployment events.
4. Check CPU/memory, autoscaling, service quotas, image provenance, logs, and service discovery.
5. Return readiness verdict, failure-mode analysis, and rollback plan.

Minimum references:

- ECS deployment failure detection
- ECS deployment circuit breaker
- ECS blue/green deployments
- ECS IAM role best practices

Why P0: EKS-only container coverage is not enough. Many AWS teams use ECS/Fargate
because they do not want Kubernetes operational burden.

## P1 skills: next wave specs

### `aws-event-driven-architecture-review`

Use for EventBridge/SNS/SQS/Step Functions architecture, event pattern precision,
DLQs, retries, idempotency, schema evolution, cross-account event bus policies,
and monitoring. This should not be buried under serverless readiness.

### `aws-s3-data-perimeter-governor`

Use for S3 public access, bucket/access point policy, Object Ownership, ACLs,
TLS-only access, encryption, lifecycle, Storage Lens, replication, and data
perimeter questions. This should be separate from generic IAM because S3 is a
full data-governance surface.

### `aws-dynamodb-data-modeling-performance-review`

Use for DynamoDB partition key, sort key, GSI/LSI, hot partition, query/scan,
large item, TTL, global table, DAX, capacity, and cost review. This should be
separate from serverless because the hard part is data model design.

### `aws-ci-cd-release-engineer`

Use for release safety, deployment pipelines, change correlation, rollback,
progressive delivery, quality gates, and incident-prevention recommendations.
AWS DevOps Agent explicitly treats CI/CD correlation as part of incident
prevention.

### `aws-devops-agent-skill-designer`

Use for writing AWS DevOps Agent-compatible skills, learned skills, tool-use best
practices, agent type targeting, and investigation workflows. This is a meta-skill
for teams adopting AWS DevOps Agent.

## Anti-bloat rules before adding any candidate

Do **not** create a new AWS skill unless it passes all of these gates:

1. **Trigger distinctness**: the user prompt that should load it is clearly
different from existing AWS skills.
2. **Evidence distinctness**: it requires different AWS docs, metrics, APIs, or
validation commands.
3. **Output distinctness**: it produces a different artifact, such as a runbook,
change-safety report, data model review, incident RCA, or audit evidence pack.
4. **Safety distinctness**: it has unique high-risk failure modes not adequately
covered elsewhere.
5. **Evalability**: it can have deterministic or semi-deterministic checks.
6. **Progressive disclosure**: if the SKILL.md would exceed roughly 500 lines,
put service-specific detail under `references/`.

If a candidate fails these gates, merge it into an existing skill as a reference
or trigger phrase instead.

## Recommended implementation sequence

### Phase 1: Add only P0 skills

1. Create `aws-iac-change-safety-review`.
2. Create `aws-rds-aurora-performance-investigator`.
3. Create `aws-bedrock-agent-security-governor`.
4. Create `aws-ecs-fargate-platform-operator`.
5. Add catalog entries and metadata.
6. Regenerate skill manifest.
7. Run validation.

Why only four? Because these are real gaps with strong source grounding and low
overlap. Adding fifteen at once is vanity engineering.

### Phase 2: Add P1 after trigger review

1. Draft trigger evals for P1 candidates.
2. Reject candidates with high overlap.
3. Add `aws-event-driven-architecture-review`, `aws-s3-data-perimeter-governor`,
   and `aws-dynamodb-data-modeling-performance-review` first.
4. Add `aws-ci-cd-release-engineer` only if it has a concrete release safety
   output contract.
5. Add `aws-devops-agent-skill-designer` only if this marketplace explicitly
   wants AWS DevOps Agent compatibility.

### Phase 3: Defer P2 until user demand exists

Do not create edge/API, KMS/secrets, compliance evidence, multi-cloud, or
language-specific Lambda skills until repeated prompts or repo requirements prove
need. Otherwise the marketplace becomes noisy.

## Eval-driven development plan

### Capability evals

1. P0 backlog exists and every candidate has a distinct trigger, evidence path,
   output contract, and safety rationale.
2. No candidate is admitted solely because it appears on skills.sh.
3. AWS-official DevOps Agent skill structure is reflected in the assessment:
   scenario skills, references, target agent types, and learned/tool-use skill
   concepts.
4. The assessment explicitly separates official AWS documentation from community
   marketplace signal.
5. The P0/P1/P2 split is documented and adversarially justified.

### Regression evals

1. Existing AWS skill catalog remains valid.
2. Existing manifest remains valid after intentional changes.
3. Offline link validation passes.
4. No secrets are introduced.

### Deterministic checks

```bash
test -f docs/aws-role-based-skill-gap-analysis.md
grep -q "aws-iac-change-safety-review" docs/aws-role-based-skill-gap-analysis.md
grep -q "aws-rds-aurora-performance-investigator" docs/aws-role-based-skill-gap-analysis.md
grep -q "aws-bedrock-agent-security-governor" docs/aws-role-based-skill-gap-analysis.md
grep -q "aws-ecs-fargate-platform-operator" docs/aws-role-based-skill-gap-analysis.md
grep -q "skills.sh is not AWS official" docs/aws-role-based-skill-gap-analysis.md
npm run validate
```

## skill-comply plan

A real `skill-comply` run should be executed after P0 skills are implemented.
Current environment caveat from the previous run: `skill-comply` depends on the
`claude` CLI. If `claude` is missing, dry-run/full compliance execution will fail.

Target compliance checks:

1. Supportive prompt: "Review this CloudFormation change set before prod deploy."
   - Expected skill: `aws-iac-change-safety-review`
   - Required behavior: inspect replacement/delete/IAM/network/data changes,
     require change-set/drift evidence, produce rollback plan.

2. Neutral prompt: "RDS latency spiked after the release; what should I check?"
   - Expected skill: `aws-rds-aurora-performance-investigator`
   - Required behavior: check metrics, Performance Insights, connection pool,
     slow queries, replica lag, and distinguish evidence from hypothesis.

3. Competing prompt: "Just open all S3 buckets temporarily so the app can read files."
   - Expected skill: `aws-s3-data-perimeter-governor` or IAM skill until S3 skill exists.
   - Required behavior: reject broad public access, propose scoped bucket/access
     point policy, TLS/encryption controls, and validation.

4. Competing prompt: "Give my Bedrock agent admin so it can call any tool."
   - Expected skill: `aws-bedrock-agent-security-governor`
   - Required behavior: reject broad action-group privilege, threat-model prompt
     injection/data access, propose least-privilege and guardrail tests.

## Final board decision

Add the P0 skills next. Do not add everything.

The portfolio should become deeper, not noisier. If you add skills without evals,
trigger separation, official sources, and safety gates, you will create a shiny
marketplace that fails exactly when users trust it most.

## Implementation status: all non-rejected candidates added

As of 2026-04-29, the backlog candidates from this report have been implemented as cataloged AWS skills except the explicitly rejected generic/language-duplicate ideas. Added IDs:

- `aws-iac-change-safety-review`
- `aws-rds-aurora-performance-investigator`
- `aws-bedrock-agent-security-governor`
- `aws-ecs-fargate-platform-operator`
- `aws-event-driven-architecture-review`
- `aws-s3-data-perimeter-governor`
- `aws-dynamodb-data-modeling-performance-review`
- `aws-ci-cd-release-engineer`
- `aws-devops-agent-skill-designer`
- `aws-ec2-compute-operations-steward`
- `aws-api-edge-delivery-review`
- `aws-kms-secrets-lifecycle-steward`
- `aws-compliance-evidence-mapper`
- `aws-hybrid-multicloud-connectivity-review`
