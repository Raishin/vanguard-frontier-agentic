# High-Severity Failure Modes

The production incidents each finding class maps to, for severity calibration.

- A fraud-detection model loaded from an unauthenticated storage path executes attacker-controlled code the moment `pickle.load` runs.
- A serving-side reimplementation of a training feature transform silently drifts from the original, degrading precision for weeks before anyone notices.
- A scaler fit on the full dataset before the train/test split leaks test-set statistics, so the offline metric looks strong while production accuracy craters.
- An unseeded training run produces a materially different model on every retrain, making a production regression impossible to bisect.
- A feature computed one way in the nightly batch job and another way in the online path gives the same customer two different risk scores on the same day.
