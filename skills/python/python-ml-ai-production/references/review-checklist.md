# ML/AI Production Review Checklist

The per-concern checklist applied to every ML/AI production review.

- Artifact trust: model files are loaded only from a trusted, integrity-verified source; never load an untrusted pickle/joblib artifact.
- Skew: the same feature-transformation code and library versions run on the training and serving paths.
- Leakage: preprocessing (scaler/encoder/imputer) is fit on the training fold only; no target or future feature leaks into training.
- Reproducibility: training uses a fixed seed, pinned library versions, and a recorded dataset/version alongside the artifact.
- Evaluation: the split and metric match how the model is deployed (time-ordering, production class balance).
- Provenance: deployed models and prompts/configs are versioned and recorded for rollback and audit.
