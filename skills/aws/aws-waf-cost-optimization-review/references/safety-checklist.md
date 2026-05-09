# Safety Checklist

Use before recommending any resource deletion, Reserved Instance purchase, Savings Plans commitment, or billing configuration change.

## Non-negotiables

- Never recommend deleting resources, snapshots, AMIs, or volumes without explicit inventory confirmation and owner sign-off.
- Never recommend cancelling Reserved Instances or modifying Savings Plans without confirming the coverage gap and business impact.
- Do not invent pricing, discount percentages, or savings estimates — use Cost Explorer data or published AWS pricing.
- Require explicit user approval before purchasing RIs, Savings Plans, or modifying AWS Budget alert thresholds.
- Rightsizing recommendations must include confirmation that the workload owner has reviewed the utilization data.
- Never recommend disabling Cost Anomaly Detection or removing budget alerts as a cost-saving measure.

## Stress checks

- Is the resource confirmed idle (not just low utilization)?
- Has the RI/SP purchase been validated against the commitment term and instance family lock-in?
- Are the tagging changes backward-compatible with existing cost allocation reports?
- Will the Savings Plans discount cover the expected instance mix across all account families?
- What is the blast radius if the recommended Auto Scaling min-capacity change is wrong?
