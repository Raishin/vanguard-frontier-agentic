# Registry, Monitoring, and Lifecycle

What a model registry must carry for a prediction to be explainable, and which monitoring signal actually measures whether a model is still right. Load for production-readiness and post-deployment reviews.

## The registry as source of truth

- The registry is only governance if it is the path to production. A registry that is written to after deployment is a record of what happened, not a control over what happens.
- Each version should carry: the training data reference, the code revision, the dependency set, the evaluation metrics and the evaluation set, the feature definitions used, and the approver.
- Promotion should be an explicit decision with a comparison against the incumbent version on a common evaluation set — not a copy of the newest artifact.
- Lineage must connect prediction to model version to feature set to source data. Without that chain, 'why did the model decide this' has no answer, and that question arrives eventually for any model that affects people.

## Three monitoring signals, one of which measures correctness

- **Input drift** — the feature distributions have moved. Cheap, immediate, and only a proxy: a model can be fine under drifted inputs and broken under stable ones.
- **Prediction drift** — the output distribution has moved. Also a proxy, and also cheap. Useful as an early warning and as a detector of an upstream pipeline break.
- **Performance against realized outcomes** — the only signal that measures whether the model is still right. It is the one most often missing, because outcomes arrive with a delay and building the join is work.
- Where outcomes arrive too late to be useful, say so and state what proxy is being used instead, plus the residual risk. Do not present a proxy as a performance measurement.
- Every signal needs a threshold and a named owner. A dashboard with no threshold is a decoration; a threshold with no owner is a notification nobody reads.

## Lifecycle: rollback, retraining, retirement

- **Rollback** — the exact steps to return to a prior version, the time it takes, and what happens to predictions produced under the bad version. A model rollback that leaves incorrect decisions in place is only half a rollback; state the remediation for those decisions too.
- **Retraining** — a policy with a trigger (scheduled, drift-threshold, or performance-threshold), a validation gate, and an approval. 'When it looks bad' means nobody is watching in between.
- **Retirement** — what happens to the predictions a retired model already produced, whether they remain in use downstream, and whether they must be recomputed or annotated.
- Every one of these is cheaper to design before deployment than to invent during an incident, and this is the point in the lifecycle where that is still possible.

## Sources

Primary sources for the claims above. Each line states what that page establishes — a URL with no claim attached is a bibliography, not a reference.

- https://docs.snowflake.com/en/developer-guide/snowflake-ml/model-registry/overview — What the model registry stores, how versions are managed, and how registered models are invoked
- https://docs.snowflake.com/en/developer-guide/snowflake-ml/model-observability — The monitoring capabilities available for deployed models and what each observes
- https://docs.snowflake.com/en/developer-guide/snowflake-ml/feature-store/overview — Feature definitions and entity modelling — the structural basis for eliminating training/serving skew
