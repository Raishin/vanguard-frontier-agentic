# Optimization Economics

The contract every cost recommendation must satisfy, and the anti-patterns that produce reported savings which never appear on an invoice. Load before proposing any change.

## The seven questions

- **Expected saving** — in credits, with the calculation shown, not a percentage.
- **Reliability risk** — what becomes more likely to fail, and how it fails.
- **Performance risk** — which SLO moves, and by how much.
- **Engineering cost** — the work to implement and to operate afterwards.
- **Confidence** — and the evidence that would raise it.
- **How the saving will be measured** — the volume-normalized comparison and the sustain period, agreed before the change.
- **Rollback condition** — the observation that triggers reverting, defined in advance so it is not argued about later.
- A recommendation missing any of the seven is a suggestion. This board does not ship suggestions as recommendations.

## Anti-patterns that produce phantom savings

- **Demand-drop savings.** Credits fell because the workload shrank. Always compare volume before and after; without it, the claim is unfalsifiable.
- **Deferred cost.** A saving that moves work to a different month, a different warehouse, or a different team's budget is an accounting effect.
- **Undersizing.** Shrinking a warehouse can increase total credits when queries spill and run longer. Measure the credits per successful run, not the hourly rate.
- **Retention trading.** Reducing retention lowers storage and lowers recovery capability. That is a risk decision with a different owner.
- **Control removal.** Removing a row-access policy or a monitoring job to save credits transfers cost into the risk register.
- **Aggregate blindness.** A 20% warehouse reduction reported while serverless and AI spend grew 60% is a true statement about a shrinking share of the bill.
- **Unsustained savings.** A change measured for one week and reported as annualized is a forecast, not a result. State the sustain period in the claim.
