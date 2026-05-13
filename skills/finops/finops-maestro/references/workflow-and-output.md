# Routing table and domain taxonomy

Use this reference when classifying a task or selecting the right specialist(s).

## Routing table

| Signal keywords | Agent ID | Domain | Live-guard? |
|---|---|---|---|
| LLM cost, token price, foundation model pricing, input/output tokens, context caching, batch inference, Claude pricing, GPT pricing, Gemini pricing, Bedrock pricing, Azure OpenAI pricing, Vertex AI pricing, OCI Generative AI pricing, GPU-hour, $/M tokens, model selection, AI cost optimization, AI spend | finops-ai-economist-agent | AI/ML model cost economics | No |
| pod rightsizing, CPU request, memory limit, p95, p99, VPA, Karpenter, node consolidation, namespace cost allocation, workload cost, OpenCost, pod request, resource quota, idle capacity, cluster cost, node SKU | finops-kubernetes-rightsizer-agent | Kubernetes cost rightsizing and allocation | No |
| cloud pricing, on-demand price, AWS price, Azure price, OCI price, GCP price, instance type cost, S3 cost, Blob Storage cost, compute pricing, pricing API, cost estimate, prototype cost, architecture cost, pre-provisioning estimate, reserved instance, savings plan | finops-cloud-price-advisor-agent | Cloud resource on-demand pricing | No |

## Domain taxonomy

| Domain | Keywords and signals |
|---|---|
| `ai-economics` | Foundation model pricing, token economics, input/output/cached/batch pricing, context window cost, GPU-hour pricing, inference endpoint cost, model comparison, AI cost per request, cost per 1M tokens |
| `kubernetes-rightsizing` | Pod CPU/memory requests and limits, p50/p95/p99 utilization, VPA recommendations, Karpenter consolidation, namespace cost allocation, workload cost attribution, OpenCost export, idle vs allocated cost, node bin-packing |
| `cloud-pricing` | Public pricing API, on-demand compute, storage, data-transfer pricing, multi-cloud price comparison, cost estimate, prototype architecture pricing, pricing by region and tier |

## Dispatch examples

### Example 1: AI model cost comparison

**User request:** "Compare the per-token cost of Claude Sonnet 4 on Bedrock versus Azure OpenAI GPT-4o."

**Routing:**
```
Route: finops-ai-economist-agent
Reason: Task is a pure foundation model pricing comparison across two providers — single AI-economics domain.
Mode: single
```

`finops-ai-economist-agent` fetches current public pricing for both models, builds a cost-per-token comparison table labeled with provenance, and highlights cached-token pricing where available.

---

### Example 2: Kubernetes pod rightsizing + AI inference cost

**User request:** "Rightsize our GPU pods running inference workloads and estimate the model cost for the requests they serve."

**Routing:**
```
Route: finops-kubernetes-rightsizer-agent, finops-ai-economist-agent
Reason: Task spans Kubernetes pod rightsizing (CPU/memory/GPU requests from p95 metrics) and AI model cost economics (per-token inference spend) — two distinct FinOps domains.
Mode: parallel (2)
```

`finops-kubernetes-rightsizer-agent` reviews pod specs and utilization metrics to produce right-sized request/limit recommendations; `finops-ai-economist-agent` estimates the per-token cost curve for the served models, including batch vs real-time pricing trade-offs.

---

### Example 3: Multi-cloud compute pricing comparison

**User request:** "What is the on-demand price for a 96-vCPU compute-optimized instance on AWS, Azure, and GCP in us-east?"

**Routing:**
```
Route: finops-cloud-price-advisor-agent
Reason: Task is a multi-cloud on-demand compute price comparison — single cloud-pricing domain.
Mode: single
```

`finops-cloud-price-advisor-agent` fetches live prices from AWS, Azure, and GCP public pricing APIs for the requested instance family and region, returns a line-item table with source timestamps, and notes any equivalent SKU caveats.

---

### Example 4: Full FinOps posture review — AI costs, K8s allocation, and cloud pricing

**User request:** "Review our AI inference spend, identify over-provisioned pods, and benchmark our cloud pricing against alternatives."

**Routing:**
```
Route: finops-ai-economist-agent, finops-kubernetes-rightsizer-agent, finops-cloud-price-advisor-agent
Reason: Task spans AI model cost economics, Kubernetes workload rightsizing, and cloud on-demand pricing benchmarking — three distinct FinOps domains.
Mode: parallel (3)
```

`finops-ai-economist-agent` analyzes AI inference spend by model and provider; `finops-kubernetes-rightsizer-agent` reviews pod utilization metrics and produces rightsizing recommendations with estimated monthly savings; `finops-cloud-price-advisor-agent` benchmarks current compute pricing against equivalent SKUs on alternative clouds. Hard-ceiling of 4 specialists; this stays under the limit.

---

### Refused request: live billing mutation

**User request:** "Apply a budget alert on my AWS account for the ML team at $5,000/month."

**Routing:**
```
Route: REFUSED
Reason: This request requires a live write to a billing system. No live-guard agents exist in v1. Escalate to a human operator with AWS Cost Management console access.
Mode: N/A
```

No agent in this catalog executes billing mutations, budget writes, or alert configuration. The human operator must apply this change using the AWS Cost Management console or Terraform. The `finops-cloud-price-advisor-agent` can estimate the projected spend, but no agent writes to external systems.

---

## Provenance label protocol

Every value produced by a routed specialist must carry one of these labels:

| Label | Meaning |
|---|---|
| `live-evidence` | Fetched from a live public API or tool output within this session |
| `documentation-based` | Sourced from official documentation; may not reflect the current live price |
| `inference` | Derived by the skill from inputs using documented methodology |
| `excluded` | Data that was intentionally excluded from the output and why |
