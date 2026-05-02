# Workflow and Output Contract

## Workflow

### Step 1 — Identify scope and collect raw evidence

1. Confirm the review target: a specific Rollout resource, an AnalysisTemplate, a traffic provider configuration, or a PDB compatibility question.
2. List all Rollouts and their strategies:
   ```bash
   kubectl get rollout -A -o yaml
   ```
   For each Rollout, note the strategy type (`canary` or `blueGreen`) and whether `spec.strategy.canary.steps` is non-empty.
3. List all AnalysisTemplates:
   ```bash
   kubectl get analysistemplate -A -o yaml
   kubectl get clusteranalysistemplate -o yaml 2>/dev/null
   ```
4. Check current Rollout status and any active AnalysisRuns:
   ```bash
   kubectl argo rollouts status <rollout-name> -n <namespace>
   kubectl get analysisrun -A -o yaml
   ```

### Step 2 — Audit Rollout strategy and steps

A Rollout without steps behaves like a standard Deployment — no progressive traffic shifting occurs.

1. Check whether `spec.strategy.canary.steps` is non-empty and includes analysis gates:
   ```yaml
   # CORRECT: canary with weight steps and analysis gate
   strategy:
     canary:
       canaryService: my-app-canary
       stableService: my-app-stable
       trafficRouting:
         nginx:
           stableIngress: my-app-ingress
       steps:
         - setWeight: 10
         - pause: {duration: 5m}
         - analysis:
             templates:
               - templateName: error-rate-check
         - setWeight: 50
         - pause: {duration: 10m}
         - analysis:
             templates:
               - templateName: error-rate-check

   # RISKY: no steps — immediately shifts all traffic
   strategy:
     canary:
       maxSurge: "100%"
       maxUnavailable: 0
   ```
2. Flag as **HIGH** if `maxSurge: 100%` is set with no steps — 100% of replicas are replaced before any analysis runs.
3. For blue-green Rollouts, check whether `autoPromotionEnabled` is set:
   ```yaml
   # Requires manual promotion
   strategy:
     blueGreen:
       activeService: my-app-active
       previewService: my-app-preview
       autoPromotionEnabled: false
   ```
   `autoPromotionEnabled: true` in production without a `prePromotionAnalysis` is a high finding.

### Step 3 — Audit AnalysisTemplate success and failure conditions

This is the most critical control — conditions that always evaluate true defeat automated rollback entirely.

1. For each AnalysisTemplate metric, inspect:
   - `spec.metrics[].successCondition` — when is the metric considered passing?
   - `spec.metrics[].failureCondition` — when should it fail?
   - `spec.metrics[].failureLimit` — how many failures are tolerated?
   - `spec.metrics[].provider` — Prometheus, Datadog, web, job, etc.
2. Example of a correctly configured error-rate AnalysisTemplate:
   ```yaml
   apiVersion: argoproj.io/v1alpha1
   kind: AnalysisTemplate
   metadata:
     name: error-rate-check
   spec:
     metrics:
       - name: error-rate
         interval: 2m
         count: 5
         failureLimit: 0
         provider:
           prometheus:
             address: http://prometheus.monitoring.svc.cluster.local:9090
             query: |
               sum(rate(http_requests_total{status=~"5..",deployment="{{args.deployment-name}}"}[2m]))
               /
               sum(rate(http_requests_total{deployment="{{args.deployment-name}}"}[2m]))
         successCondition: result[0] < 0.01
         failureCondition: result[0] >= 0.05
   ```
3. Flag as **CRITICAL** if `successCondition` evaluates true for all possible metric values:
   - `result >= 0` (always true for any non-negative counter)
   - `true` (literal boolean true)
   - `result != "error"` (only fails on error, never on bad metric values)
4. Flag as **HIGH** if `failureCondition` is absent — the metric can only succeed, never explicitly fail.
5. Flag as **MEDIUM** if `failureLimit` is set to 100 or greater on an error-rate metric — 100 failures will be tolerated before marking Degraded.
6. Flag as **HIGH** if the Prometheus query template references `{{args.deployment-name}}` but no `args` are passed in the Rollout's analysis step — the query evaluates against all deployments, returning misleading results.

### Step 4 — Audit canaryService and stableService isolation

Without separate Services, canary pods receive the same traffic distribution as stable — canary traffic isolation does not exist.

1. Check whether both `canaryService` and `stableService` are specified:
   ```bash
   kubectl get rollout <name> -o jsonpath='{.spec.strategy.canary.canaryService},{.spec.strategy.canary.stableService}'
   ```
2. Verify the Services exist and have the correct selector labels:
   ```bash
   kubectl get svc <canaryService> <stableService> -o yaml | grep -A 5 "selector"
   ```
   Argo Rollouts manages the `rollouts-pod-template-hash` selector on these Services automatically — verify neither has a hardcoded hash that bypasses Rollouts management.
3. Flag as **HIGH** if `canaryService` is absent — all traffic hits the stable Service regardless of setWeight steps.

### Step 5 — Audit traffic provider alignment

A misconfigured traffic provider silently ignores all weight changes.

1. Check the traffic routing provider specified in the Rollout:
   ```bash
   kubectl get rollout <name> -o jsonpath='{.spec.strategy.canary.trafficRouting}'
   ```
2. Verify the specified provider is actually installed:
   ```bash
   # For Istio
   kubectl get virtualservice -A | head -5
   kubectl get destinationrule -A | head -5

   # For Nginx
   kubectl get ingressclass | grep nginx

   # For AWS ALB
   kubectl get ingressclass | grep alb

   # For Traefik
   kubectl get traefikservice -A 2>/dev/null | head -5
   ```
3. Common mismatches:
   - Rollout specifies `trafficRouting.nginx` but the cluster uses AWS ALB Ingress Controller.
   - Rollout specifies `trafficRouting.istio` but Istio is not installed or not managing the service's namespace.
4. Flag as **HIGH** if the provider specified does not match installed ingress — weight steps are silently no-ops and all traffic remains on stable.

### Step 6 — Audit PDB compatibility with Rollout surge settings

A PDB that prevents pod eviction can deadlock a canary rollout that requires replacing existing pods.

1. Check PDBs in the same namespace as the Rollout:
   ```bash
   kubectl get pdb -n <namespace> -o yaml
   ```
2. Check Rollout maxUnavailable and maxSurge:
   ```bash
   kubectl get rollout <name> -o jsonpath='{.spec.strategy.canary.maxUnavailable},{.spec.strategy.canary.maxSurge}'
   ```
3. Identify deadlock conditions:
   - `maxUnavailable: 0` in the Rollout means old pods cannot be removed until new pods are Ready.
   - A PDB with `minAvailable: 100%` (or `maxUnavailable: 0`) means no pod can be evicted.
   - Combined: new pods can never start because the cluster has no capacity, and old pods cannot be removed due to PDB — **deadlock**.
4. Example of a safe PDB configuration alongside a canary Rollout:
   ```yaml
   # PDB: allow 1 unavailable pod during updates
   apiVersion: policy/v1
   kind: PodDisruptionBudget
   metadata:
     name: my-app-pdb
   spec:
     maxUnavailable: 1
     selector:
       matchLabels:
         app: my-app

   # Rollout: maxSurge allows creating new pods above desired count
   strategy:
     canary:
       maxSurge: "25%"
       maxUnavailable: 0
   ```
5. Flag as **HIGH** if `maxUnavailable: 0` in the Rollout and `maxUnavailable: 0` (or `minAvailable: 100%`) in a PDB matching the same pods.

### Step 7 — Audit rollback posture and history

1. Verify `revisionHistoryLimit` is set to retain enough history for a safe rollback:
   ```bash
   kubectl get rollout <name> -o jsonpath='{.spec.revisionHistoryLimit}'
   ```
   The default is 10. A limit of 1 means only one previous revision is retained — if the rollback target was already overwritten, rollback fails.
2. Check `abortScaleDownDelaySeconds` for the canary:
   ```bash
   kubectl get rollout <name> -o jsonpath='{.spec.strategy.canary.abortScaleDownDelaySeconds}'
   ```
   Default is 30 seconds. Setting this to 0 means canary pods are immediately deleted on abort — useful for fast rollback but removes the ability to inspect the canary pods post-abort.
3. To manually trigger a rollback:
   ```bash
   kubectl argo rollouts abort <rollout-name> -n <namespace>
   kubectl argo rollouts undo <rollout-name> -n <namespace>
   ```
4. Verify automated abort is wired to the AnalysisRun:
   ```bash
   kubectl get analysisrun -A -o yaml | grep -A 5 "phase"
   ```
   An AnalysisRun in `Failed` phase should trigger the Rollout to transition to `Degraded` and initiate rollback automatically.

### Step 8 — Verify Argo Rollouts controller health

A degraded or missing Argo Rollouts controller means all Rollout objects are frozen — no progression, no rollback, no weight changes.

1. Check controller health:
   ```bash
   kubectl get pods -n argo-rollouts
   kubectl describe deployment argo-rollouts -n argo-rollouts
   ```
2. Check for recent controller errors:
   ```bash
   kubectl logs -n argo-rollouts -l app.kubernetes.io/name=argo-rollouts --tail=50 | grep -i error
   ```
3. Flag as **HIGH** if the argo-rollouts controller has unavailable replicas and any Rollout is mid-canary — the canary will not progress or roll back automatically until the controller recovers.

## Output

Return:

- **target**: Rollout name, namespace, and strategy type, with evidence source,
- **evidence level**: `live evidence` / `documentation-based` / `sanitized user evidence` / `inference`,
- **strategy correctness**: steps present/absent, analysis gates present/absent, blue-green autoPromotion setting,
- **AnalysisTemplate audit**: successCondition and failureCondition correctness, failureLimit values, Prometheus query argument wiring,
- **service isolation**: canaryService and stableService presence, selector management,
- **traffic provider alignment**: specified provider vs installed ingress controller,
- **PDB compatibility**: deadlock risk with Rollout maxSurge/maxUnavailable settings,
- **rollback posture**: revisionHistoryLimit, abortScaleDownDelaySeconds, automated abort wiring,
- **controller health**: argo-rollouts controller pod state,
- **risk findings** (with severity: critical / high / medium / low),
- **safest next actions** with sample YAML,
- **assumptions and missing facts**.

## Security notes

- Never recommend bypassing AnalysisTemplate gates to force a canary promotion — fix the underlying metric or analysis query instead.
- Never recommend setting `successCondition: true` or equivalent always-passing conditions to unblock a stuck rollout.
- A Rollout with `autoPromotionEnabled: true` and no `prePromotionAnalysis` in production is equivalent to a standard Deployment — progressive delivery provides no safety gate.
- Always verify the AnalysisTemplate Prometheus query actually targets the canary deployment specifically, not the entire service or namespace — a query that averages stable and canary traffic can mask canary errors.
- Do not recommend increasing `failureLimit` as a fix for a legitimate analysis failure — investigate the root cause first.
