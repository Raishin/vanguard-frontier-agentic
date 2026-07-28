# Model-Promotion-Control Review Checklist

The per-concern checklist applied to every model-promotion request.

- Immutability: exactly one immutable model artifact is promoted per approval, never a mutable or in-place-edited artifact.
- Integrity: the artifact is hash- and/or signature-verified before promotion; unverified provenance blocks promotion.
- Deserialization risk: a pickle/joblib artifact executes code on load, so an unverified artifact is treated as a remote-code-execution risk regardless of the source package's popularity.
- Risk classification: an AI-risk classification (AI RMF / EU AI Act role) is recorded before promotion.
- Evaluation: evaluation evidence matched to the deployment context exists before promotion.
- Rollback: live monitoring is configured and a rollback to the prior artifact is pre-approved before promoting.
