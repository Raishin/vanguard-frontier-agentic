# Workflow and output contract

Use this reference only when performing a full FunctionGraph production readiness assessment for a Huawei Cloud serverless workload.

## Review domains

Check these areas before giving a verdict:

- VPC configuration: whether the function is bound to a VPC and whether security group rules permit connectivity to required private resources
- Concurrency and reserved instances: whether the concurrency limit is set, whether reserved instances match expected baseline load, and whether burst handling is documented
- Cold-start optimization: package size, dependency layer usage, initialization code volume, and whether pre-stop hooks are configured
- Observability: whether LTS log group and stream are bound, whether AOM alarms are configured on error rate and p99 duration, and whether distributed tracing is enabled
- Timeout configuration: whether the function timeout is shorter than the trigger and upstream caller timeout, and whether retry semantics are intentional
- Dependency management: deployment package size, whether FunctionGraph layers are used for shared dependencies, and whether unused dependencies are removed
- Runtime selection: whether managed or custom runtime is used and whether the choice is documented
- ServiceStage lifecycle: whether health check endpoints are configured and whether deployment strategy is documented

## Safe workflow

1. **Frame scope**
   - Function(s) in scope and their event triggers:
   - Region and account context:
   - Current-state evidence:
   - Required latency and reliability targets:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live Huawei Cloud console evidence if available.
   - Otherwise inspect IaC/config, sanitized user evidence, or official Huawei Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test the configuration**
   - What VPC resources are unreachable if VPC binding is absent or security group rules are too restrictive?
   - What errors are invisible in production if LTS log binding is not configured?
   - What p99 latency impact occurs if reserved instances are absent during traffic bursts?
   - What duplicate processing occurs if the function timeout exceeds the trigger timeout and retries fire?
   - What security vulnerabilities are present if a custom runtime is not patched?
   - What evidence is missing to confirm the function meets its latency and reliability targets?
4. **Recommend the smallest safe action**
   - Prefer targeted fixes, staged changes, and verification steps.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:

```markdown
# Huawei Cloud Serverless Production Readiness: <scope>
## VPC configuration and network connectivity assessment
## Concurrency and reserved instance planning
## Cold-start optimization assessment
## Observability coverage via LTS and AOM
## Timeout configuration and dependency chain analysis
## Dependency package size and layer management review
## Runtime selection rationale
## ServiceStage lifecycle and health check status
## Prioritized production readiness improvements
```

Each section must include an evidence level label.
