# Leakage, Skew, and Reproducibility

The three defects that make a good model metric meaningless, and how to check for each. Load before any model is promoted.

## Leakage, by name

- **Future information.** A feature computed from data that would not exist at prediction time — an aggregate over a window that includes the prediction moment, a status field updated after the outcome, a join to a table refreshed nightly and read as of today.
- **Target-derived features.** Anything computed from the label, however indirectly: a downstream field that is only populated for positive cases, a bucket assigned by a process that already knew the answer.
- **Split contamination.** The same entity in train and test; overlapping time periods in a temporal problem; duplicated rows across splits. All three inflate the metric and none raises an error.
- **Preprocessing before splitting.** Scalers, encoders, and imputers fitted on the full dataset leak test-set statistics into training.
- **Time-travel leakage in a warehouse specifically.** Snowflake makes it very easy to build a training set by joining current-state dimension tables to historical facts. Unless the dimension is queried as of the fact's timestamp, every training row carries information from the future.
- The diagnostic signature is a validation metric noticeably better than any comparable production system, and a feature-importance ranking dominated by one field. Treat both as leakage hypotheses until refuted.

## Training/serving skew

- Skew arises whenever the training feature and the serving feature are two implementations. It is not a possibility; it is what happens over time.
- The structural fix is one definition consumed by both paths — a shared feature definition rather than parallel SQL and Python.
- Where two paths are unavoidable, measure the skew: compute both for the same entities and compare distributions and per-row differences. Report the measurement, not the intention.
- Skew also arrives through timing: a feature refreshed hourly at training time and daily at serving time is a different feature even with identical logic.
- Missing-value handling is the most common silent skew: training imputes, serving passes null, and the model sees a value it never trained on.

## Reproducibility as a checklist

- **Data snapshot** — an addressable, immutable reference to exactly the rows used. A query with `CURRENT_DATE` in it is not reproducible; a query pinned to a timestamp, a snapshot table, or a versioned dataset is.
- **Code** — a committed revision, not a notebook state.
- **Dependencies** — pinned versions for every library. Floating versions re-run differently and the difference is not attributable.
- **Seeds and hyperparameters** — recorded with the run, not reconstructed from memory.
- **Environment** — the compute and runtime the training ran on, where results depend on it.
- Report each element as present or missing individually. 'Mostly reproducible' means not reproducible, and the missing element is the finding.
