#!/usr/bin/env node
/**
 * Generate Kiro Powers from catalog/agents.json.
 *
 * Each Power is a directory under powers/ containing a single POWER.md.
 * Kiro Powers spec (kirodotdev/powers) restricts frontmatter to exactly
 * five fields: name, displayName, description, keywords, author. No
 * version, no repository, no license, no tags. The validator enforces
 * the strict-5 rule.
 *
 * Body content is templated from per-provider config (steering content
 * authored once, here) plus catalog facts (maestro id, live-guard list,
 * agent count) read at generate time. This keeps the steering tight
 * and the agent inventory accurate.
 *
 * Mode:
 *   --check  exit 1 if any on-disk Power does not match the generated one
 *   (default) write/overwrite all Powers
 */

import { readFileSync, writeFileSync, existsSync, mkdirSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = join(dirname(fileURLToPath(import.meta.url)), "..");
const catalogPath = join(repoRoot, "catalog", "agents.json");
const powersRoot = join(repoRoot, "powers");

const check = process.argv.includes("--check");

// Per-provider steering content. Description: max 3 sentences (Kiro
// constraint). Keywords: specific terms only — Kiro docs warn that broad
// keywords trigger false activations. Invariants: 2-4 bullets that the
// AI must keep in mind when handling this provider's work.
const PROVIDERS = {
  aws: {
    displayName: "Vanguard Frontier — AWS",
    description:
      "Curated AWS agents for IAM, EKS, Lambda, RDS, S3, and Bedrock with live-mutation guards. Routes via aws-maestro to specialist or live-guard agents based on task scope. Mutations on real AWS environments require account-ID, region, and approval confirmation before execution.",
    keywords: ["aws", "iam", "eks", "lambda", "rds", "s3", "bedrock", "live-guard"],
    invariants: [
      "Confirm AWS account ID and region before any live mutation.",
      "Live-guard agents (aws-live-*) must never be auto-dispatched; require explicit approval and rollback plan.",
      "IAM least-privilege review applies to every policy attachment, role assumption, and trust relationship.",
      "Cross-account access via assume-role must be reviewed by aws-iam-review-agent before activation.",
    ],
  },
  azure: {
    displayName: "Vanguard Frontier — Azure",
    description:
      "Curated Azure agents for Entra ID, AKS, App Service, Key Vault, Cosmos DB, and ARM/Bicep with live-mutation guards. Routes via azure-maestro to specialist or live-guard agents. Mutations on real Azure environments require subscription ID, tenant ID, resource group, and approval confirmation.",
    keywords: ["azure", "entra-id", "aks", "app-service", "key-vault", "cosmos-db", "bicep", "live-guard"],
    invariants: [
      "Confirm Azure subscription ID, tenant ID, and resource group before any live mutation.",
      "Live-guard agents (azure-live-*) must never be auto-dispatched; require approval and rollback plan.",
      "PIM (Privileged Identity Management) elevation is a separate decision from RBAC role assignment.",
      "Management group SCP-equivalent policies cascade — review blast radius before org-level changes.",
    ],
  },
  gcp: {
    displayName: "Vanguard Frontier — GCP",
    description:
      "Curated Google Cloud agents for IAM, GKE, Cloud Run, BigQuery, Vertex AI, and AlloyDB with live-mutation guards. Routes via gcp-maestro to specialist or live-guard agents. Mutations require project ID, region, and approval confirmation; org-level changes need additional review.",
    keywords: ["gcp", "iam", "gke", "cloud-run", "bigquery", "vertex-ai", "alloydb", "live-guard"],
    invariants: [
      "Confirm GCP project ID and region/zone before any live mutation.",
      "Live-guard agents (gcp-live-*) must never be auto-dispatched; require approval and rollback plan.",
      "IAM Conditions and workload identity federation are reviewed by gcp-iam-review-agent before activation.",
      "Org policy constraints take precedence over project-level IAM grants.",
    ],
  },
  oci: {
    displayName: "Vanguard Frontier — OCI",
    description:
      "Curated Oracle Cloud agents for IAM, OKE, Autonomous Database, Vault, and Resource Manager with live-mutation guards. Routes via oci-maestro to specialist or live-guard agents. Distinguishes commercial vs gov-cloud realm; mutations require tenancy, compartment, and region confirmation.",
    keywords: ["oci", "oracle-cloud", "iam", "oke", "autonomous-database", "vault", "resource-manager", "live-guard"],
    invariants: [
      "Confirm OCI tenancy OCID, compartment, and region before any live mutation.",
      "Live-guard agents (oci-live-*) must never be auto-dispatched; require approval and rollback plan.",
      "Commercial and government cloud realms have separate identity domains — verify realm before action.",
      "Compartment hierarchy enforces policy scope; review parent-compartment grants before sub-compartment changes.",
    ],
  },
  alibaba: {
    displayName: "Vanguard Frontier — Alibaba Cloud",
    description:
      "Curated Alibaba Cloud agents for RAM, ACK, PolarDB, OSS, and MaxCompute with live-mutation guards and China-region compliance. Routes via alibaba-maestro to specialist or live-guard agents. China mainland (cn-*) and international regions have separate billing and regulatory scope — always confirm context.",
    keywords: ["alibaba-cloud", "ram", "ack", "polardb", "oss", "maxcompute", "mlps-2", "live-guard"],
    invariants: [
      "Confirm region: China mainland (cn-hangzhou, cn-beijing, etc.) and international regions have separate billing accounts and different regulatory scope.",
      "MLPS 2.0 Level 3 mandates specific service configurations — alibaba-china-compliance-agent flags gaps before live changes.",
      "Live-guard agents (alibaba-live-*) must never be auto-dispatched; require approval and rollback plan.",
      "OSS bucket policies must be reviewed for public exposure and PIPL/DSL data-residency compliance before live changes.",
    ],
  },
  huawei: {
    displayName: "Vanguard Frontier — Huawei Cloud",
    description:
      "Curated Huawei Cloud agents for IAM, CCE, GaussDB, OBS, DEW (KMS+CSMS), and ModelArts with live-mutation guards and MLPS 2.0 compliance. Routes via huawei-maestro to specialist or live-guard agents. Enterprise Projects are billing constructs, not security boundaries — verify IAM and SCP scope independently.",
    keywords: ["huawei-cloud", "iam", "cce", "gaussdb", "obs", "dew", "modelarts", "live-guard"],
    invariants: [
      "Confirm Huawei Cloud account ID, region, and Enterprise Project before any live mutation.",
      "Enterprise Projects are billing/attribution constructs, NOT security boundaries — verify IAM policy and SCP scope independently.",
      "MLPS 2.0 Level 3 (GB/T 22239-2019) requires specific service configurations — huawei-compliance-sovereignty-agent flags gaps.",
      "Live-guard agents (huawei-live-*) must never be auto-dispatched; require approval and rollback plan.",
    ],
  },
  ovhcloud: {
    displayName: "Vanguard Frontier — OVHcloud",
    description:
      "Curated OVHcloud agents for IAM, Managed Kubernetes, networking, and KMS with live-mutation guards. Routes via ovhcloud-maestro to specialist or live-guard agents. EU-headquartered sovereignty cloud; mutations require project ID and region confirmation.",
    keywords: ["ovhcloud", "ovh", "iam", "managed-kubernetes", "kms", "eu-sovereignty", "live-guard", "data-residency"],
    invariants: [
      "Confirm OVHcloud project ID and region before any live mutation.",
      "Live-guard agents (ovhcloud-live-*) must never be auto-dispatched; require approval and rollback plan.",
      "EU sovereignty cloud — review data-residency requirements before cross-region replication.",
    ],
  },
  scaleway: {
    displayName: "Vanguard Frontier — Scaleway",
    description:
      "Curated Scaleway agents for IAM, Kapsule (managed Kubernetes), networking, and cost optimization with live-mutation guards. Routes via scaleway-maestro to specialist or live-guard agents. EU-region only (PAR, AMS, WAW); mutations require organization ID and region confirmation.",
    keywords: ["scaleway", "iam", "kapsule", "managed-kubernetes", "cost-optimizer", "eu-region", "live-guard"],
    invariants: [
      "Confirm Scaleway organization ID and region (PAR, AMS, WAW) before any live mutation.",
      "Live-guard agents (scaleway-live-*) must never be auto-dispatched; require approval and rollback plan.",
      "Kapsule rollout changes require PDB audit and health-signal verification.",
    ],
  },
  hetzner: {
    displayName: "Vanguard Frontier — Hetzner",
    description:
      "Curated Hetzner agents for infrastructure review, cost optimization, capacity planning, and live server-lifecycle and firewall-rule guards. Routes via the Hetzner pattern to specialist agents. EU-headquartered provider; mutations on real Hetzner projects require project ID and region confirmation.",
    keywords: ["hetzner", "infrastructure-review", "cost-optimizer", "capacity-planner", "server-lifecycle", "firewall-rules", "live-guard"],
    invariants: [
      "Confirm Hetzner project ID and location before any live mutation.",
      "Live-guard agents (hetzner-live-*) must never be auto-dispatched; require approval and rollback plan.",
      "Firewall rule changes require capture of current ruleset and explicit egress-blocking review.",
    ],
  },
  contabo: {
    displayName: "Vanguard Frontier — Contabo",
    description:
      "Curated Contabo agents for security hardening, cost optimization, capacity planning, and live instance-lifecycle and storage-operations guards. Routes via the Contabo pattern to specialist agents. Mutations on real Contabo accounts require account context and region confirmation.",
    keywords: ["contabo", "security-hardening", "cost-optimizer", "capacity-planner", "instance-lifecycle", "storage-operations", "live-guard"],
    invariants: [
      "Confirm Contabo account context and region before any live mutation.",
      "Live-guard agents (contabo-live-*) must never be auto-dispatched; require approval and rollback plan.",
      "Storage operations on object storage and block storage require backup verification before destructive actions.",
    ],
  },
  ionos: {
    displayName: "Vanguard Frontier — IONOS",
    description:
      "Curated IONOS agents for security and compliance review, datacenter design, cost optimization, Managed Kubernetes operations, and live database-lifecycle guards. Routes via ionos-maestro to specialist or live-guard agents. Mutations require contract ID and datacenter confirmation.",
    keywords: ["ionos", "security-compliance", "datacenter-designer", "managed-kubernetes", "database-lifecycle", "live-guard", "eu-sovereignty"],
    invariants: [
      "Confirm IONOS contract ID and datacenter before any live mutation.",
      "Live-guard agents (ionos-live-*) must never be auto-dispatched; require approval and rollback plan.",
      "DBaaS lifecycle mutations require backup verification and replication-status review.",
    ],
  },
  kubernetes: {
    displayName: "Vanguard Frontier — Kubernetes",
    description:
      "Curated Kubernetes agents for RBAC review, workload identity, Pod Security Admission, admission policies, network policies, ArgoCD GitOps, and live mutation guards across RBAC, admission, mesh, network, and rollout planes. Routes via kubernetes-maestro to specialist or live-guard agents. Cluster context and namespace must be confirmed before any live mutation.",
    keywords: ["kubernetes", "rbac", "workload-identity", "pod-security-admission", "admission-policies", "argocd", "live-guard"],
    invariants: [
      "Confirm cluster context (kubeconfig + namespace) before any live mutation.",
      "Live-guard agents (kubernetes-live-*) must never be auto-dispatched; require approval and rollback plan.",
      "RBAC ClusterRole and ClusterRoleBinding changes affect every namespace — review blast radius first.",
      "Admission policies (Kyverno, ValidatingAdmissionPolicy) apply at cluster scope; review for unintended workload rejection.",
    ],
  },
  terraform: {
    displayName: "Vanguard Frontier — Terraform",
    description:
      "Curated Terraform agents for plan/apply review, state safety, deletion protection, and blast-radius assessment of IaC changes. Routes via terraform-maestro to the terraform-reviewer for plan analysis. Plan review is required before any apply targeting real infrastructure; state-modifying commands require explicit approval.",
    keywords: ["terraform", "iac", "plan-review", "state-safety", "deletion-protection", "blast-radius"],
    invariants: [
      "Plan review (terraform plan output) must precede any apply on real infrastructure.",
      "Resource destruction and replacement (terraform plan: '-/+') require explicit confirmation with backup verification.",
      "State-modifying commands (terraform state rm, mv, push) require explicit approval — they bypass plan review.",
      "Workspace context (workspace, var-file, backend) must be confirmed before running plan or apply.",
    ],
  },
  nvidia: {
    displayName: "Vanguard Frontier — NVIDIA",
    description:
      "Curated NVIDIA agents for GPU resource governance, NIM model deployment, NGC registry hygiene, supply-chain integrity, and runtime evidence gating. Routes via nvidia-maestro to specialist agents and through the runtime-evidence-gate before runtime-affecting mutations. GPU resource changes require capacity, cost, and supply-chain review.",
    keywords: ["nvidia", "ngc", "nim", "gpu-governance", "runtime-evidence", "supply-chain"],
    invariants: [
      "Runtime mutations require evidence via nvidia-runtime-evidence-gate before execution.",
      "GPU resource allocation must be reviewed for capacity, cost, and tenant isolation impact.",
      "NGC container provenance and SBOM must be validated before deployment to runtime hosts.",
      "Driver and CUDA version changes have node-wide blast radius — review compatibility matrix first.",
    ],
  },
  salesforce: {
    displayName: "Vanguard Frontier — Salesforce",
    description:
      "Curated Salesforce agents for admin review, development, security, integration, revenue ops, service ops, marketing ops, Agentforce/AI risk, and compliance — static review only, no org mutations. Routes via salesforce-maestro to specialist agents covering Sales Cloud, Service Cloud, Experience Cloud, Marketing Cloud, MuleSoft, Tableau, and industry verticals. All Salesforce terminology and API surfaces are drift-prone; agents always verify against current official documentation before rendering findings.",
    keywords: ["salesforce", "agentforce", "crm", "apex", "lwc", "mulesoft", "compliance", "static-review"],
    invariants: [
      "Static review only — agents never request org credentials, session tokens, or user PII, and never mutate a Salesforce org.",
      "Salesforce API versions and feature availability vary by org edition and release; verify org context (edition, API version, enabled features) before applying any recommendation.",
      "Agentforce and Einstein AI configurations are adversarially reviewed for prompt-injection risk, ungrounded automation, and missing human-handoff controls before any approve-or-merge decision.",
      "Live-guard agent (salesforce-live-guard-agent) must never be auto-dispatched; require explicit approval, target org confirmation, and rollback plan.",
    ],
  },
};

const catalog = JSON.parse(readFileSync(catalogPath, "utf8"));

function summarize(provider) {
  const entries = catalog.filter(
    (e) => e.type === "agent" && e.provider === provider,
  );
  const kiroEntries = entries.filter(
    (e) => Array.isArray(e.harnesses) && e.harnesses.includes("kiro"),
  );
  const maestro = entries.find((e) => e.id.endsWith("-maestro-agent"));
  const liveGuards = entries
    .filter((e) => /-live-/.test(e.id))
    .map((e) => e.id)
    .sort();
  return {
    total: entries.length,
    kiroAvailable: kiroEntries.length,
    maestro,
    liveGuards,
  };
}

function renderPower(provider, cfg) {
  const { total, kiroAvailable, maestro, liveGuards } = summarize(provider);
  const frontmatter = [
    "---",
    `name: "vanguard-${provider}"`,
    `displayName: "${cfg.displayName}"`,
    `description: "${cfg.description}"`,
    `keywords: [${cfg.keywords.map((k) => `"${k}"`).join(", ")}]`,
    `author: "Raishin"`,
    "---",
  ].join("\n");

  const liveGuardSection = liveGuards.length
    ? liveGuards.map((id) => `- \`${id}\` — never auto-dispatched; gate_mode only`).join("\n")
    : "- *(none — this provider has no live-mutation guards in the catalog)*";

  const maestroLine = maestro
    ? `- **\`${maestro.id}\`** — classifies and routes the task to the right specialist`
    : `- *(no maestro for this provider; reference agents directly under \`agents/${provider}/\`)*`;

  const adapterNote =
    kiroAvailable === total
      ? `All ${total} agents in this provider ship a Kiro adapter (\`harnesses/kiro-ide.agent.md\`, \`kiro-cli.agent.json\`).`
      : kiroAvailable === 0
        ? `This provider's ${total} agents do not yet ship Kiro adapters — this Power supplies steering content only. Use \`npx vfa-export-agents --platform kiro --provider ${provider}\` from the npm package once Kiro adapters land.`
        : `${kiroAvailable} of ${total} agents in this provider ship a Kiro adapter; the rest provide steering context only.`;

  const body = [
    "",
    `# ${cfg.displayName}`,
    "",
    cfg.description,
    "",
    "## When to engage this Power",
    "",
    `Activate when the task references ${provider === "kubernetes" ? "Kubernetes, cluster, namespace, RBAC, or admission policy" : provider === "terraform" ? "Terraform, IaC, plan, apply, or state" : provider === "nvidia" ? "NVIDIA, NGC, NIM, GPU, or CUDA" : `${cfg.displayName.replace(/^Vanguard Frontier — /, "")} services, resources, or operations`}. Do not activate on unrelated requests — narrow keyword matching is required to avoid false activations (Kiro Powers convention).`,
    "",
    "## Routing pattern",
    "",
    maestroLine,
    "",
    "Use the maestro as the entry point: classify the task, then dispatch to one specialist or a parallel team of specialists. Never have the maestro itself execute a live mutation.",
    "",
    "## Live-guard agents (gate_mode only)",
    "",
    liveGuardSection,
    "",
    "Live-guard agents enforce approval, target confirmation, evidence capture, and rollback plans before executing a mutation. They are never auto-dispatched — the maestro must place them in `live-guard-gate` or `runtime-evidence-gate` mode.",
    "",
    "## Invariants",
    "",
    cfg.invariants.map((s) => `- ${s}`).join("\n"),
    "",
    "## Where the agents live",
    "",
    `Agent specs and adapters are part of the [Vanguard Frontier Agentic](https://github.com/Raishin/vanguard-frontier-agentic) marketplace. For this provider, see \`agents/${provider}/\` in that repository. ${adapterNote}`,
    "",
    "## Companion install paths",
    "",
    "- **Claude Code:** `/plugin marketplace add Raishin/vanguard-frontier-agentic` then `/plugin install vanguard-frontier-agentic@vanguard-frontier-agentic`",
    `- **Codex / Copilot / Cursor / Gemini CLI / Kiro (file export):** \`npx vfa-export-agents --platform <harness> --provider ${provider} --repo .\``,
    "",
  ].join("\n");

  return frontmatter + body;
}

const errors = [];
const written = [];

for (const [provider, cfg] of Object.entries(PROVIDERS)) {
  const dir = join(powersRoot, `vanguard-${provider}`);
  const file = join(dir, "POWER.md");
  const next = renderPower(provider, cfg);

  if (check) {
    if (!existsSync(file)) {
      errors.push(`${relative(repoRoot, file)} is missing`);
      continue;
    }
    if (readFileSync(file, "utf8") !== next) {
      errors.push(`${relative(repoRoot, file)} is stale; run npm run kiro-powers:write`);
    }
  } else {
    if (!existsSync(dir)) mkdirSync(dir, { recursive: true });
    writeFileSync(file, next);
    written.push(relative(repoRoot, file));
  }
}

if (check) {
  if (errors.length) {
    errors.forEach((e) => console.error(`ERROR: ${e}`));
    process.exit(1);
  }
  console.log(
    `OK: ${Object.keys(PROVIDERS).length} Kiro Powers are in sync`,
  );
} else {
  console.log(`OK: wrote ${written.length} Kiro Powers`);
  written.forEach((f) => console.log(`  ${f}`));
}
