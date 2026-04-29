# Official sources

Primary AWS documentation used to shape this guarded live-AWS role:

- https://docs.aws.amazon.com/AmazonECS/latest/developerguide/deployment-circuit-breaker.html
- https://docs.aws.amazon.com/AmazonECS/latest/developerguide/deployment-alarm-failure.html
- https://docs.aws.amazon.com/AmazonECS/latest/developerguide/deployment-failure-detection.html
- https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs_service_deployment_events.html

## Source-grounding rule

Use official AWS documentation as the source of truth for service behavior. Use Context7 only as a supplementary guideline for agent-guardrail design when it helps clarify stricter instruction patterns; do not let secondary examples override AWS service docs.
