> Static review only. Scope: JVM Kafka clients (Apache Kafka Java client, Spring Kafka). Every conclusion needs both the producer configuration/call sequence and the consumer configuration/call sequence as evidence; a delivery-semantics claim resting on only one side is `inference (partial source)` or `assumption (source absent)`. Sources: Apache Kafka documentation (`kafka.apache.org/documentation/`, semantics section), Spring for Apache Kafka reference (`docs.spring.io/spring-kafka/reference/`). Kafka client defaults have changed across releases (notably the idempotent-producer default flip in a Kafka 3.0-era release) — verify the deployed client/broker version against the official documentation rather than assuming a default; this reference states the *mechanism*, not a version-pinned default.

## The core conflation this agent exists to catch

"We have exactly-once" is the single most common false claim in Kafka pipeline design, and it almost always traces to one misunderstanding: `enable.idempotence=true` (or its implied form once `acks=all` is set) deduplicates **producer retries within a single producer session**, identified by a broker-assigned producer ID (PID) and per-partition sequence number. It stops the classic "network timeout, producer retries, broker had already written it, you get a duplicate" failure. It does **not**:
- Survive a producer process restart (a new PID is assigned; the broker has no memory of the old session's sequence numbers) — unless a `transactional.id` is also configured, in which case Kafka uses the transactional ID to recover and fence the previous instance's PID.
- Make the read → process → write cycle atomic. A consumer can read a message, produce a downstream side effect (write to a database, call another service, produce a Kafka message), and crash before committing its consumer offset — that read-process-write triple is not covered by producer idempotence at all, because idempotence only concerns the producer's own retries.

Flag any code comment, design doc, or architecture claim in the reviewed material that equates `enable.idempotence=true` with "exactly-once" or "EOS" without the transactional machinery below. This is a `CRITICAL`-severity finding because it is a correctness claim the system does not actually satisfy — downstream consumers of the claim (on-call runbooks, incident response, capacity/consistency assumptions) will be wrong.

## True exactly-once (read-process-write EOS): the complete checklist

Kafka's actual exactly-once guarantee is scoped to a **read-process-write** cycle where both the read offset commit and the write are part of one atomic transaction. All of the following must be present; any one missing breaks the guarantee (it does not degrade gracefully to "mostly exactly-once"):

1. **`transactional.id`** set on the producer — a stable, unique identifier per logical producer instance (not shared across concurrently running instances of the same logical producer; see the fencing note below). This is what lets Kafka recover producer identity across restarts and fence zombies.
2. **`producer.initTransactions()`** called once at producer startup, before any transactional work. This registers the transactional ID with the transaction coordinator and fences any previous producer instance using the same ID.
3. Each unit of work wrapped in **`producer.beginTransaction()` … `producer.commitTransaction()`**, with **`producer.abortTransaction()`** on any failure path. A transaction left neither committed nor aborted (e.g., an uncaught exception that skips the abort) will eventually time out via `transaction.timeout.ms`, but the correct pattern aborts explicitly.
4. Consumer offsets committed via **`producer.sendOffsetsToTransaction(offsets, consumerGroupMetadata)`** as part of the *same* transaction as the produced records — not via the consumer's own `commitSync()`/`commitAsync()`. Committing offsets outside the transaction breaks atomicity: the produce can commit while the offset commit is lost (or vice versa), reintroducing exactly the read-process-write gap EOS exists to close.
5. Downstream consumers configured **`isolation.level=read_committed`**. The default, `read_uncommitted`, returns every record in offset order regardless of whether its transaction ultimately committed or aborted — a consumer on `read_uncommitted` reading a produced-but-later-aborted record is a correctness bug hiding behind a producer that looks transactional. `read_committed` uses the transaction control (commit/abort) markers to withhold uncommitted/aborted records until resolution, at the cost of visibility latency (records only become visible after the transaction resolves).

Treat any subset of the above — e.g., `transactional.id` and `initTransactions()` present but the consumer left on default `isolation.level`, or offsets committed via the consumer API instead of `sendOffsetsToTransaction` — as **broken EOS**, not "partial" or "mostly correct" EOS. State explicitly which element is missing.

```java
// Minimal correct shape for the producer side of read-process-write EOS
producer.initTransactions();
try {
    ConsumerRecords<K, V> records = consumer.poll(pollTimeout);
    producer.beginTransaction();
    for (ConsumerRecord<K, V> record : records) {
        producer.send(toOutputRecord(record));
    }
    Map<TopicPartition, OffsetAndMetadata> offsets = currentOffsets(records);
    producer.sendOffsetsToTransaction(offsets, consumer.groupMetadata());
    producer.commitTransaction();
} catch (KafkaException e) {
    producer.abortTransaction();
}
```

## At-least-once + idempotent consumer: the pragmatic alternative

Most services do not need transactional EOS — its throughput and latency cost (transaction coordinator round trips, `read_committed` visibility delay, `transactional.id`-per-instance operational overhead) is often not worth it when a simpler pattern gets the same observable correctness. **At-least-once delivery plus an idempotent consumer** is that pattern, and it is this agent's own call (absorbed here, not deferred to a separate consumer-idempotency specialist):

- The producer need not be transactional; a plain idempotent producer (or even a non-idempotent one, with the caveat that its own retries can duplicate) is sufficient because the consumer is designed to tolerate redelivery.
- The consumer commits its offset **only after** the side effect (DB write, downstream call, produced message) has succeeded — see the commit-ordering rules below.
- The side effect itself must be idempotent under redelivery: a natural or synthetic dedup key (message key + offset, an event ID in the payload, an idempotency token) checked against a uniqueness constraint or an upsert (`INSERT … ON CONFLICT`, a versioned/last-write-wins update) rather than an unconditional `INSERT`/side-effecting call.

Flag an at-least-once design (manual commit after processing, no transactional producer) that has **no** dedup key, no upsert semantics, and no idempotency table/unique constraint on the write side as a `HIGH` finding: at-least-once guarantees redelivery on rebalance, retry, or crash-and-restart, and without consumer-side dedup that redelivery becomes a duplicate side effect (double charge, double email, double order) rather than a harmless replay.

## Commit ordering: the two ways to lose or duplicate messages

- **Message loss** — `enable.auto.commit=true` (the client default) commits the offset on a fixed timer (`auto.commit.interval.ms`) regardless of whether the polled batch was actually, successfully processed. If the process crashes (or a downstream call fails) after the timer fires but before processing completes, the offset is already advanced and the message is gone on the next poll. The same defect appears with **manual** commit issued *before* processing completes ("commit-then-process") — it is not specific to auto-commit, just the same ordering mistake made explicitly. Flag either shape as `HIGH`: message loss is silent and usually discovered downstream (a missing record, a support ticket), not at the point of failure.
- **Message duplication** — commit issued *after* successful processing is the correct order for at-least-once, but on its own it still permits duplicates (a crash between successful processing and the commit redelivers the same message). This is expected and acceptable **only if** the consumer is idempotent per the section above; without idempotency it is the same `HIGH` finding as the at-least-once-without-dedup case.

## Escalation / boundary conditions

- The correctness question is about the Kafka transactional API's interaction with a *non*-Kafka database transaction (e.g., dual-write outbox pattern, `@Transactional` wrapping a JPA write and a Kafka publish) rather than `sendOffsetsToTransaction` itself → the Kafka-transaction wiring stays here; the surrounding JDBC/JPA transaction boundary and isolation correctness is the transaction-and-consistency agent's call.
- The payload itself uses a deserialization path with known RCE surface (Java native deserialization, unguarded Jackson polymorphic typing, SnakeYAML) → hand to `java-deserialization-and-parser-security-agent`; do not render a delivery-semantics verdict that also tries to cover payload-deserialization safety.
- The user asks to actually run a producer/consumer against a live broker to observe behavior → out of scope for static review; describe what to instrument and who runs it.

## Known uncertainty

- Whether `enable.idempotence` defaults to `true` (and what that implies for `acks`/`max.in.flight.requests.per.connection` defaults) depends on the Kafka client version in use; this changed in a Kafka 3.0-era release. Verify the exact behavior for the version under review against `kafka.apache.org/documentation/` rather than assuming a version-independent default.
- Spring Kafka's `KafkaTransactionManager` / `ChainedKafkaTransactionManager` wiring for "synchronize a JPA transaction with a Kafka transaction" has version-specific capabilities (and known limitations — it does not make a JDBC write and a Kafka write atomic against each other, only sequences their commit/rollback). Treat any claim of true dual-system atomicity via these managers as `inference` requiring the Spring Kafka reference version in use, and default to recommending the outbox pattern for genuine dual-write atomicity.
