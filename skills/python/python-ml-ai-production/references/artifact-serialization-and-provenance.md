# Model-Artifact Serialization And Provenance

Safe model persistence, and evaluation/provenance requirements for deployment.

- scikit-learn/joblib model persistence uses pickle under the hood, which executes arbitrary code on load, so a model must be loaded only from a trusted, integrity-verified source.
- Evaluation and batch-vs-online paths must be consistent with deployment.
- Versioned artifacts and recorded model/prompt config enable rollback and audit.

## Sources

- https://scikit-learn.org/stable/model_persistence.html
- https://docs.python.org/3/library/pickle.html
