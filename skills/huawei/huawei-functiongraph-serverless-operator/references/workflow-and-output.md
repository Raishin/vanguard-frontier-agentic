# Workflow and output contract

Use this reference only when performing a full serverless platform review or implementation guidance.

## Serverless areas to check

- FunctionGraph functions: trigger type, runtime, memory/timeout settings, VPC attachment, concurrency limit, reserved concurrency
- Cold start posture: VPC-attached functions without reserved concurrency, runtime initialization overhead
- Async invocation: event source, queue depth, failure handling policy (discard/DLQ), DLQ topic binding
- ServiceStage: application inventory, deployment type, rolling update strategy, health check endpoint
- CSE: microservice registry status, config namespace inventory, consumer dependency mapping
- Cost governance: function invocation volume, CU consumption, timeout efficiency

## Safe workflow

1. **Frame scope** — confirm target functions, trigger types, traffic profile, and non-goals
2. **Collect evidence** — prefer live invocation metrics and function configuration; label all evidence types
3. **Stress-test** — cold start latency under load, async queue saturation, CSE config blast radius
4. **Recommend safest action** — reserved concurrency tuning, DLQ wiring, staged CSE config changes

## Output contract

Return this structure:

```markdown
# Huawei Cloud Serverless Platform: <scope>
## Scope and evidence level
## FunctionGraph function inventory
## Cold start risk assessment
## Async invocation failure handling
## ServiceStage and CSE configuration
## Recommended actions
## Open questions
```

Each section must include an evidence level label.
