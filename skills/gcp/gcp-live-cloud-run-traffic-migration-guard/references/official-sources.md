# Official Sources — GCP Live Cloud Run Traffic Migration Guard

Authoritative GCP documentation for Cloud Run traffic management, revision lifecycle, and instance configuration.

## Core References

- **Rollouts, Rollbacks, and Traffic Migration** — https://cloud.google.com/run/docs/rollouts-rollbacks-traffic-migration
  How to split traffic between revisions, perform gradual rollouts, and manually roll back to a prior revision.

- **Configuring Min Instances** — https://cloud.google.com/run/docs/configuring/min-instances
  How min-instances settings affect cold-start behavior, cost, and availability; guidance for production traffic patterns.

- **Managing Revisions** — https://cloud.google.com/run/docs/managing/revisions
  How to list, describe, and delete Cloud Run revisions; restrictions on deleting revisions that hold traffic.

- **Gradual Rollouts** — https://cloud.google.com/run/docs/deploying#gradual-rollouts
  Step-by-step guidance for canary deployments and progressive traffic migration using percentage splits.

- **Configuring Concurrency** — https://cloud.google.com/run/docs/configuring/concurrency
  How concurrency settings interact with traffic volume and cold-start behavior during traffic migrations.

- **Viewing Logs for Cloud Run** — https://cloud.google.com/run/docs/logging
  How to view request logs, container logs, and audit logs for Cloud Run services in Cloud Logging.

- **Cloud Run Metrics** — https://cloud.google.com/run/docs/monitoring
  Available metrics for Cloud Run (request count, latency, instance count) and how to use them for post-migration health checks in Cloud Monitoring.

- **Cloud Run IAM** — https://cloud.google.com/run/docs/securing/managing-access
  IAM roles for Cloud Run (`run.viewer`, `run.developer`, `run.admin`) and how to configure service-level access control.
