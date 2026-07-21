# Flow, State Sharing, And Backpressure

Cold vs hot Flow, StateFlow/SharedFlow replay/buffer, and delivery guarantees.

- A cold Flow re-runs its producer for each collector; a hot StateFlow conflates and replays only the latest value; a SharedFlow buffers a configurable replay/extraBuffer.
- `StateFlow` drops intermediate values, so it is unsafe as an event bus where every event must be delivered — use a SharedFlow with explicit replay/buffer or a Channel.
- `buffer()` decouples producer and consumer; `conflate()` keeps only the latest; an unbounded or DROP overflow strategy silently loses events under load and must match the claimed delivery guarantee.

## Sources

- https://kotlinlang.org/docs/flow.html
- https://kotlinlang.org/docs/shared-flow.html
