# EVAL DEFINITION: finops-maestro-strategy

## Subject

Strategic business thesis "FinOps Maestro" — an AI-agent orchestration platform for Kubernetes and cloud cost accountability at Fortune 50 scale. Compressed thesis:

> FinOps Maestro is the autonomous Kubernetes and cloud cost accountability layer that turns engineering telemetry into board-grade unit economics with auditable AI agents that recommend, approve, and verify savings actions inside the workflows finance and engineering already use. Wedge: K8s cost attribution + chargeback for one BU at one cloud. ROI target: 8–15% reduction in addressable infrastructure spend in 90 days; payback 2–4 months. Differentiation: closed-loop workflow with audit trail; K8s depth bridged to finance; outcome-aligned pricing; in-tenant deployment; FOCUS + OpenCost as canonical data layer; agent governance posture exceeding NIST AI RMF baseline.

## Capability Evals (pass@3 >= 90%)

| ID | Eval | Owner Team | Grader |
|---|---|---|---|
| C1 | Competitive landscape: thesis defensible against Apptio, Flexera, Vantage, Finout, Kubecost, CAST AI, AWS/Azure/GCP native FinOps in 12–18 months? | Validation | Model |
| C2 | ROI model: $105M Y1 savings on $1B spend defensible per Fortune 50 customer? Payback 2–4 months credible? | Validation | Model |
| C3 | Technical feasibility: FOCUS + OpenCost + CMDB reconciliation to >95% K8s attribution; integration timeline | Validation | Model |
| C4 | Wedge sharpness: is "K8s attribution + chargeback for one BU" sharp enough? Can it be narrower? | Strengthening | Model |
| C5 | Buyer map: economic buyer, procurement timeline, top 3 objections neutralized | Strengthening | Model |
| C6 | Moat: 3 defensible moats beyond "AI agents," ranked by durability | Strengthening | Model |
| C7 | MVP design: 90-day MVP with ingestion, agents, workflows, anti-goals | Roadmap | Model |
| C8 | Risk catalog: top 10 enterprise risks with severity, probability, mitigation | Roadmap | Model |
| C9 | Proof plan: 30/60/90 validation plan with named proof points | Roadmap | Model |

## Regression Evals (pass^3 = 100%)

| ID | Eval | Grader |
|---|---|---|
| R1 | No contradictions with current FinOps Foundation maturity model assumptions | Model |
| R2 | No claims exceed FOCUS 1.2 actual capability | Model |
| R3 | No competitive claims that incumbents already match | Model |

## Success Criteria

- **Capability evals**: pass@3 >= 90% (at least 8/9 PASS across up to 3 cycles)
- **Regression evals**: pass^3 = 100% (all 3 must hold across 3 evaluations)
- **Board readiness**: CFO can explain ROI in 2 minutes; CTO sees no stack duplication

## Eval Anti-Patterns Avoided

- No flattery (graders instructed to be harsh)
- No happy-path-only checks (each eval explicitly tests adversarial scenarios)
- No overfitting (graders forbidden from rewriting evals)
- Cost/latency tracked (each cycle bounded)

## Output Artifacts

- `.claude/evals/finops-maestro-strategy.log` — cycle-by-cycle results
- `.claude/evals/finops-maestro-strategy.summary.md` — synthesized findings
- `docs/strategy/finops-maestro.md` — board-ready final strategy

## Cycle Plan

- **Cycle 1**: 9 capability + 3 regression evals in parallel
- **Cycle 2**: re-run failures only
- **Cycle 3**: final regression sweep
- **Stop conditions**: pass@3 + pass^3 targets met, OR 3 cycles exhausted
