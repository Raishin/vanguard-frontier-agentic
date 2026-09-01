# Workflow And Output

Architectural review sequence and output contract for platform topology assessment.

## Workflow

1. Establish the complete workspace inventory: how many workspaces, which regions, compute type (serverless/classic), data classification, business purpose.
2. Check the metastore-per-region mapping: one metastore per region, no gaps, no redundancy. Flag any region without a metastore.
3. Evaluate workspace segmentation: is the workspace count justified? Does each workspace map to a legitimate segregation driver (environment, regulated data, business unit, residency, capability)?
4. Assess compute placement: is serverless used for PII and sensitive workloads? Is classic appropriate for the data classification?
5. Determine catalog organisation: is it domain-based, environment-based, or mixed? Does it align with access-control design and cost allocation?
6. Identify cross-region and cross-org access patterns: D2D OpenSharing for metastore replication? Clean Rooms for external collaboration?
7. Calculate quota headroom and identify any load-bearing or deferrable workloads near limits.

## Evidence labels

Label every claim: `confirmed` (artifact or first-party documentation provided) > `inference` (partial artifact) > `assumption` (artifact absent) > `unknown`. Distinguish documentation evidence (how Databricks behaves) from workspace evidence (how this deployment is configured). Never present an assumption as confirmed, and never let a documentation claim stand in for workspace state.

## Output contract

- A verdict (scalable-as-designed / scalable-with-conditions / architecture-risk) with explicit confidence.
- Metastore-per-region inventory and any regions without a metastore, stated upfront.
- Workspace segmentation analysis: count, drivers, justification against the 50–100 guidance.
- Compute placement review: serverless vs classic alignment with data classification.
- Quota headroom analysis and any load-bearing workloads near limits.
- Well-Architected Framework mapping and open questions about future scale.
