# What Each Check Actually Proves

A lookup from verification artifact to the claim it supports, so a suite is not mistaken for coverage.

- `validate` checks that a configuration is syntactically valid and internally consistent without contacting any provider or reading state; it proves the configuration parses and nothing about whether it would create correct infrastructure.
- A plan evaluates the configuration against current state and the installed provider versions, so it proves what the engine intends right now — and a plan produced under different provider versions is evidence about a different plan.
- A `run` block with `command = plan` executes the planning stage only, which makes it a unit test for module logic: it proves the module resolves to the intended resource arguments without creating anything.
- A `run` block with `command = apply` creates real infrastructure and runs assertions against it, which is the only mode that proves the provider actually accepts the configuration; the resources are destroyed when the test run completes.
- Mock providers supply the data a provider would return so a plan-mode test can run without credentials; they prove the module's logic under assumed provider behaviour and cannot prove the real provider agrees.
- An `assert` block's value comes entirely from what it checks: an assertion on a name, a tag, or a count passes for a resource that is also publicly readable, so coverage counted in assertions is not coverage of risk.
- A `validation` block on a variable rejects an invalid input in every caller's plan, which is strictly broader than a test proving the module rejects it in one covered case.
- A `check` block asserts continuously after apply without blocking, which suits a property that can become false through drift after a change that was correct when it was made.
- The test framework's surface must be confirmed against the engine actually in use, since a feature verified on one engine is not thereby available on the other.
