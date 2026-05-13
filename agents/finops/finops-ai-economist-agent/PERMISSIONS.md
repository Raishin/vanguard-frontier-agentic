# Permissions: FinOps AI Workload Economist

## Read-only posture

The FinOps AI Workload Economist fetches data from **public, unauthenticated** pricing endpoints only. It does not read from, write to, or mutate any cloud environment or AI provider account.

No cloud credentials, API keys, account IDs, tenant IDs, or org IDs are required or accepted. Refusal to accept such data is a hard constraint, not a preference.

---

## Allowed tool surface

- **WebFetch** — fetch live public pricing pages and APIs (URLs listed below).
- **Read / Grep / Glob** — read repository files (skill definitions, reference docs).

**Explicitly denied**: Bash, Write, Edit. This agent is read-only at the tool level.

---

## Anthropic

No authentication required. Pricing is published on a public documentation page:

```
https://docs.anthropic.com/en/docs/about-claude/pricing
```

This agent fetches and parses that page for input token price, output token price, prompt-cache-read price, prompt-cache-write price, and batch discount per model.

---

## OpenAI

No authentication required. Pricing is published on a public documentation page:

```
https://platform.openai.com/docs/pricing
```

---

## AWS Bedrock

No authentication required. AWS Bedrock pricing is published on a public page:

```
https://aws.amazon.com/bedrock/pricing/
```

If the user also wants to enumerate their actual deployed Bedrock endpoints (optional inventory mode), the following read-only IAM actions are sufficient:

```json
{
  "Effect": "Allow",
  "Action": [
    "bedrock:ListFoundationModels",
    "bedrock:GetFoundationModel",
    "sagemaker:ListEndpoints",
    "sagemaker:DescribeEndpoint"
  ],
  "Resource": "*"
}
```

This agent does **not** need or use billing API access (`ce:GetCostAndUsage`, `ce:GetCostForecast`). It builds estimates from public list prices only.

---

## Azure OpenAI

No authentication required. The Azure Retail Prices API is public and unauthenticated:

```
https://prices.azure.com/api/retail/prices
```

Filter example for Azure OpenAI: `serviceName eq 'Azure OpenAI'`.

If the user also wants to enumerate their deployed Azure OpenAI resources (optional inventory mode), the following read-only RBAC action is sufficient:

```json
{
  "Actions": ["Microsoft.CognitiveServices/accounts/read"]
}
```

No Cost Management Reader or Billing Reader role is needed.

---

## Google Vertex AI (Generative AI)

No authentication required. Vertex AI Generative AI pricing is published on a public page:

```
https://cloud.google.com/vertex-ai/generative-ai/pricing
```

If the user also wants to enumerate their deployed Vertex AI endpoints (optional inventory mode), the following read-only IAM permission is sufficient:

```
aiplatform.endpoints.list
```

---

## OCI Generative AI

No authentication required. OCI Generative AI pricing is published on a public page:

```
https://www.oracle.com/cloud/ai/generative-ai/
```

If the user also wants to enumerate their OCI Generative AI resources (optional inventory mode), the following OCI policy is sufficient (read-only, compartment-scoped):

```
Allow group FinOpsAIEconomistReadOnly to inspect generative-ai-family in compartment <compartment-name>
```

---

## Explicit DENY list

The following access patterns are **never required and must never be requested**:

- Billing API access of any kind: `ce:GetCostAndUsage`, `ce:GetCostForecast`, Azure Cost Management Reader, GCP `billing.viewer`, OCI cost-analysis policies.
- Write or mutate operations on any cloud resource.
- Collection of API keys, bearer tokens, account IDs, subscription IDs, tenant IDs, or org IDs.
- Private cost exports, billing exports, or invoice data.

If a user offers any of the above, decline and explain that list-price analysis does not require it.

---

## Inventory mode (optional, never required for list-price work)

Optional read-only inventory roles are documented per provider above. They are listed for reference only — the agent's primary function (list-price comparison and TCO analysis) requires none of them. Inventory enumeration is an enhancement that a human operator may configure separately.
