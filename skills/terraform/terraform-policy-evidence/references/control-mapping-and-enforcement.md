# Control Mapping And Enforcement Reality

The difference between a control that exists and a control that enforces, and where policies actually run.

- Enforcement level is the property that decides whether a control is real: a policy that warns produces exactly the same infrastructure as no policy at all, and reporting a warned control as satisfied is the most common way an evidence pack becomes misleading.
- A policy evaluating a plan sees resolved values — what will actually be created — while a static scan of source text sees only literals; any value arriving from a variable, a data source, or a module default is invisible to source scanning, so a control enforced only that way covers only the literal cases.
- Post-apply state evaluation detects a violation after the infrastructure exists, which makes it a detective control rather than a preventive one; classifying it as preventive misrepresents when the exposure window closes.
- Findings must be mapped to the control an auditor asks about rather than to the scanner rule that produced them, because the translation from rule identifier to control is otherwise performed months later by whoever is least equipped to do it.
- An in-language `validation` block prevents an invalid value from entering a plan at all, which is a stronger control than a policy that rejects the plan afterwards, and it fails at the module boundary where the author can act on it.
- `precondition` and `postcondition` blocks block an operation when they fail, while a `check` block is a continuous non-blocking assertion; implementing a must-block control as a `check` block produces a control that reports and never stops anything.
- OPA policy is portable across engines and runners, while Sentinel is coupled to its platform and licence; treating them as interchangeable understates the cost of a later engine or platform change.
- A policy suite with a high false-positive rate degrades every control it contains, because routine overriding trains reviewers to dismiss output without reading it — and the override record then contains no signal about the cases that mattered.
