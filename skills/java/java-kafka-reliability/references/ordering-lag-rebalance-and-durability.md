> Static review only. Scope: JVM Kafka clients (Apache Kafka Java client, Spring Kafka `@KafkaListener`/`ConcurrentKafkaListenerContainerFactory`). Ordering, lag, and rebalance findings need the consumer/producer configuration *and* a description of the processing loop (per-record time, external calls) as evidence — a stall or reordering claim without the processing-loop shape is `inference (partial source)`. Sources: Apache Kafka documentation (`kafka.apache.org/documentation/`, consumer/producer configuration references), Spring for Apache Kafka reference (`docs.spring.io/spring-kafka/reference/`, error-handling and container chapters). Numeric client defaults (poll/session timeouts, in-flight-request limits) have shifted across Kafka releases — cite the documentation section by name and instruct the user to confirm against the version in use rather than treating a remembered number as ground truth.

## Ordering: max.in.flight.requests.per.connection and idempotence

Kafka guarantees per-partition ordering only under specific producer conditions. The producer setting `max.in.flight.requests.per.connection` controls how many unacknowledged produce requests can be outstanding on one connection at once:

- With `enable.idempotence=false` and `max.in.flight.requests.per.connection > 1`, a retried batch can be written to the broker *after* a later batch that succeeded on the first attempt, reordering records within the partition. This is a `HIGH` finding whenever the reviewed design assumes per-key ordering (partitioning by a business key specifically to get ordering) but does not also guarantee in-order delivery.
- With `enable.idempotence=true`, the broker uses per-partition sequence numbers to reject out-of-order or duplicate batches, which restores ordering even with multiple in-flight requests (Kafka documents idempotent producers as safe up to `max.in.flight.requests.per.connection<=5`).
- The safe-without-idempotence fallback is `max.in.flight.requests.per.connection=1`, which serializes requests and preserves ordering at a throughput cost.

Flag a design that both (a) relies on partition-key ordering for correctness (e.g., per-entity event sequencing, state-machine transitions) and (b) either leaves idempotence off with `max.in.flight.requests.per.connection > 1`, or does not state which of the two ordering-safe configurations is in use.

## Consumer lag as the SLA signal

Consumer lag — the gap between a partition's latest produced offset (or log-end offset) and a consumer group's last committed offset — is the primary operational signal for a Kafka pipeline, and its **absence** from a design under review is itself a finding, not a non-issue:

- Rising lag with steady throughput indicates the consumer is slower than the producer (undersized consumer group, slow per-record processing, an external call in the hot path) — a capacity problem.
- Lag that jumps and then plateaus at a rebalance boundary indicates a stuck or repeatedly-rebalancing consumer, not a throughput problem — see the rebalance-stall section below; treating every lag alert as "add consumers" without checking for rebalance churn misdiagnoses the second case.
- Flag a design/runbook that has no lag-based alert (per-partition or per-group, whichever the consumer's scaling model needs) as `HIGH`: without it, a stalled or slow consumer is invisible until a downstream symptom appears (stale data, an SLA breach reported by a user), and by the time that happens the backlog has already grown.
- This agent reviews whether the code and design *produce and act on* a lag signal (metrics exposed, alert thresholds referenced in code/config/runbook text provided); it does not read live broker or monitoring-system lag values — that would require a live connection, which is out of static-review tier. If the review needs a current lag reading to render a verdict, refuse to assert it and record it as an open question for whoever operates the cluster to supply.

## max.poll.interval.ms and rebalance stalls

`max.poll.interval.ms` bounds the time between successive calls to `poll()` on a given consumer thread. If the processing done between two `poll()` calls — `max.poll.records` records at the observed per-record processing time, including any blocking external call — exceeds this interval, the consumer group coordinator considers the consumer dead, evicts it, and triggers a rebalance, even though the consumer process is still alive and working.

Consequences worth naming explicitly when this pattern is found:
- The in-flight batch's work is likely partially or fully redone by whichever consumer picks up the reassigned partition (redelivery, not loss, assuming correct commit ordering) — but repeated stalls produce a **rebalance storm**: the group never stabilizes because every consumer's batch keeps exceeding the interval, throughput collapses, and lag grows during the storm itself.
- This interacts with `max.poll.records` (batch size per poll) and any synchronous external call inside the per-record processing loop (a blocking HTTP call, a slow DB write per record) — the fix is not always "raise `max.poll.interval.ms`"; raising it without justification just delays detection of a genuinely stuck consumer.

Flag a processing loop where `max.poll.records × observed-or-estimated per-record time` is not bounded well under `max.poll.interval.ms`, with no compensating strategy (lower `max.poll.records`, move slow work off the poll thread with `pause()`/`resume()` and a bounded async handoff, or a documented and justified interval increase). This is a `HIGH` finding: it is both a correctness risk (rebalance storms) and a lag-signal confounder (see above). Note that judging whether the loop is *actually* stalling in production would require live telemetry (observed p99 processing time, rebalance-event counts) — those are not available to static review; when the per-record time is only estimated, label the finding `inference` and ask for the measured distribution rather than asserting a stall.

## Dead-letter / retry-topic design

A consumer needs a defined path for a message that cannot be processed — a malformed payload, a downstream dependency that is down, a business-rule rejection. Two shapes are both defects:

- **Unbounded retry-and-block**: retrying the same message indefinitely in place stalls the partition (nothing after it in the partition is processed either) and, combined with the `max.poll.interval.ms` mechanics above, can trigger a rebalance loop centered on the poison message.
- **Silent catch-and-continue**: swallowing the processing exception and committing past the message anyway loses it with no record and no operator visibility.

The documented, supported Spring Kafka shape is bounded retry (a `DefaultErrorHandler`/retry-template configuration with a backoff and a max-attempts bound) followed by routing to a dead-letter topic (`DeadLetterPublishingRecoverer` or an equivalent explicit publish) on exhaustion, with retryable vs. non-retryable exception classification so a permanently-malformed payload does not exhaust retries pointlessly before landing in the DLQ. Flag the absence of a DLQ/retry-topic path as `MEDIUM` (it is a resilience gap, not a delivery-semantics violation on its own) unless the missing path is what is actually causing the partition stall under review, in which case raise it alongside the rebalance finding as `HIGH`.

## Durability: acks and min.insync.replicas

- `acks=1` (leader-only acknowledgment) acknowledges the write once the partition leader has it in its local log, before followers replicate — an unclean leader failover immediately after can lose the record even though the producer received a successful acknowledgment. Flag `acks=1` (or the value left unstated) on any payload the reviewed material describes as durable, critical, or the system of record.
- `acks=all` (`acks=-1`) requires the write to be replicated to every in-sync replica (ISR) before acknowledgment — but this guarantee is only as strong as the current ISR set. With the broker default `min.insync.replicas=1`, `acks=all` can still acknowledge a write held by a single replica if the ISR has shrunk to one member, silently reducing to `acks=1` semantics during a partial outage.
- Flag `acks=all` combined with `min.insync.replicas` left at its default (1) — or not stated — as a durability gap on critical topics: recommend `min.insync.replicas>=2` with `replication.factor>=3` so the topic tolerates one broker failure without either losing acknowledged writes or losing availability (the standard trade-off documented in the Kafka replication/producer documentation).

## Known uncertainty

- The exact default for `min.insync.replicas`, `max.in.flight.requests.per.connection` under idempotence, and `max.poll.interval.ms` should be confirmed against the specific broker/client version in the reviewed material rather than assumed from this reference — Kafka has changed producer defaults across major releases.
- Lag-based alerting thresholds ("how much lag is too much") are workload-specific and are not asserted here as a number; state them as an open question for the user to supply if the review needs to judge whether an existing threshold is adequate.
