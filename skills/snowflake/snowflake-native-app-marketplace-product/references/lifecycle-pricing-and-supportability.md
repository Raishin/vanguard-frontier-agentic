# Lifecycle, Pricing, and Supportability

The three product properties that decide whether an application is a business rather than a demo. Load when planning a release, a price, or a support model.

## Version, patch, and the withdrawal problem

- Design the release lifecycle before the first release: how versions and patches are published, how consumers move between them, and what a release directive controls.
- The asymmetry that shapes everything: a provider can stop offering a version, but a consumer who has already upgraded generally cannot be rolled back to the prior one by the provider. Forward-fix is therefore the realistic recovery path.
- That asymmetry makes the release gate the real control. A rollback plan that assumes consumers can be reverted is not a rollback plan; the plan is a fast, tested patch path plus a gate strict enough to make it rare.
- Consumers stranded on old versions are a support cost and a security exposure. State how many versions are supported, for how long, and what the communication path to a consumer on an unsupported version actually is.
- Every version change is a compatibility event for any consumer object bound through a reference. Enumerate what a version can and cannot change without breaking an installed application.

## Pricing that can actually be billed

- Choose the unit of value first, then check that the platform lets the provider observe it. A per-usage model needs a usage signal the provider is permitted to receive; a per-seat model needs a seat concept the consumer's install exposes.
- Metering that depends on telemetry depends on consent. If the consumer's governance team restricts the telemetry, the pricing model stops working — so the telemetry design and the pricing design are one decision.
- Model gross margin with the support cost included. Provider-side compute for the application, plus support hours per consumer, is what turns a headline price into a margin, and support is the term most often omitted.
- Check regional and cloud availability for the target market before the revenue plan depends on it. Availability is a fact to verify, not to assume, and it changes.

## Supportability is a scaling limit

- Ask what a consumer can diagnose alone: does the application surface its own state, its errors, and its prerequisites clearly enough that a competent consumer administrator can resolve the common cases?
- Every diagnostic that requires provider access to the consumer account is three costs at once: support hours, a security conversation, and a delay while the access is approved.
- Support incidents per consumer is the number that decides whether the product scales. A product with excellent margins at ten customers and two support hours each has no margin at two hundred.
- Design the failure messages as product surface. An installation that fails with an unexplained privilege error becomes a support ticket; one that names the missing privilege becomes a consumer action.
- Shareback and telemetry, designed well, are what let a provider diagnose without account access. That is their strongest product justification and the one that makes the consent conversation winnable.

## Sources

Primary sources for the claims above. Each line states what that page establishes — a URL with no claim attached is a bibliography, not a reference.

- https://docs.snowflake.com/en/developer-guide/native-apps/versioning — How versions, patches, and release directives control which version consumers receive
- https://other-docs.snowflake.com/en/collaboration/provider-listings-about — Listing requirements, regional availability, and the monetization options available to providers
