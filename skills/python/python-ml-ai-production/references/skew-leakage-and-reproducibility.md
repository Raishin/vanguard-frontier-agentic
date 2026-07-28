# Training-Serving Skew, Leakage, And Reproducibility

Feature-parity requirements, leakage-safe preprocessing, and reproducible training.

- Training-serving skew comes from different feature code/versions on the two paths and requires a shared transformation.
- Data leakage (fitting preprocessing before the split, or target/future features) inflates offline metrics — scikit-learn's common-pitfalls guidance is to fit preprocessing within the training fold only.
- Reproducibility requires a fixed seed, pinned versions, and a recorded data snapshot.

## Sources

- https://scikit-learn.org/stable/common_pitfalls.html
- https://numpy.org/doc/stable/reference/random/generator.html
