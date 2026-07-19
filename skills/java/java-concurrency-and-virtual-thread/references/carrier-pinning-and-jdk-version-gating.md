> Scope: verified directly against the primary JEP text for JEP 444 (openjdk.org/jeps/444 — Release 21, Status Closed/Delivered) and JEP 491 (openjdk.org/jeps/491 — Release 24, Status Closed/Delivered), and against JEP 506 (openjdk.org/jeps/506 — Scoped Values, Release 25, Status Closed/Delivered). The structured-concurrency preview-status note below reflects JEP 533 ("Structured Concurrency (Seventh Preview)", targeted at Release 27) as the most recent iteration found at the time this file was verified; structured concurrency has iterated through JEPs 428, 437, 453, 462, 480, 499, 505, 525, and 533 across releases and may finalize under a still-later JEP number. Check the JEP index (openjdk.org/jeps/0) for the current status before citing a specific JEP number as final — do not trust this file's JEP number for structured concurrency as permanent.

## The two things that pin a carrier (JEP 444, JDK 21+)

Per JEP 444's own text, a virtual thread cannot unmount from its carrier platform thread — and therefore blocks the carrier for the duration of any blocking operation performed while pinned — in exactly two situations:

1. It executes code inside a `synchronized` block or method.
2. It executes a native method or a foreign function (Foreign Function & Memory API).

Pinning does not make an application incorrect; it degrades scalability by capturing a carrier that could otherwise run a different virtual thread while the pinned thread blocks. JEP 444 is explicit that a rarely-executed synchronized block (e.g., only at startup) or one that only guards in-memory work is not a meaningful risk. The risk is a *frequently executed* synchronized construct that also performs a blocking operation — I/O, `BlockingQueue.take()`, and similar — while holding the monitor, under high virtual-thread fan-out. JEP 444 itself recommends replacing such hot, blocking-while-held `synchronized` usage with `java.util.concurrent.locks.ReentrantLock`.

## What changed in JDK 24+ (JEP 491)

JEP 491 changed the JVM's implementation of `synchronized` so a virtual thread can unmount while blocked acquiring a monitor, and while blocked in `Object.wait()`/its timed variants, releasing its carrier back to the scheduler in both cases. Per the JEP's own Summary, this change is intended to "eliminate nearly all cases of virtual threads being pinned" — but that claim is scoped specifically to `synchronized` constructs. JEP 491's Description section is explicit that the `jdk.VirtualThreadPinned` JFR event is *retained after this change specifically for native-method and Foreign Function & Memory pinning*, which JEP 491 does not address. In other words: JEP 491 does not eliminate pinning in general, it eliminates the `synchronized`-specific case.

**Practical gating rule:**
- JDK 21–23: a frequently-hit `synchronized` block or method wrapping a blocking operation is a real, version-appropriate pinning finding.
- JDK 24+: the same `synchronized` code is no longer a pinning risk on its own — but if that code (or a library it transitively calls) reaches native code or the Foreign Function & Memory API under load, pinning risk remains, on every JDK version including 24+.

Never apply JDK-21-era pinning advice to JDK-24+ code without re-checking whether the risk is actually the `synchronized` case (now fixed) or the native/FFM case (still live).

## Evidence requirement — do not assert pinning without JFR

Source alone shows *risk*, not *fact*. To confirm actual pinning in a running system, require one of:

- A JDK Flight Recorder capture showing `jdk.VirtualThreadPinned` events (with duration and stack trace) for the code path in question.
- `-Djdk.tracePinnedThreads=full` (or `=short`) output captured during representative load, per JEP 444's documented diagnostics.

Absent this evidence, label a pinning finding `inference (partial source)` when source shows a `synchronized` construct on a plausibly hot, blocking path, or `assumption (source absent)` when there is no evidence the path is hot or blocking at all — never `confirmed`. Ask for the JFR or trace evidence before raising a pinning finding above medium severity.

## ThreadLocal cost at virtual-thread scale

There is no single authoritative "N threads is too many" number to cite here, and none should be invented — this is a qualitative, structural risk, not a benchmark claim. `ThreadLocal` (and `InheritableThreadLocal`) storage sized and reasoned about for a platform-thread pool of, say, tens of threads does not have the same footprint when the same code runs per-task across a population that can reach into the millions with virtual threads: each live virtual thread that has touched a given `ThreadLocal` retains its own copy until the thread terminates or the value is explicitly removed. Flag `ThreadLocal`-backed caches, buffers, or session-like state that is carried unmodified from a platform-thread-pool design into a virtual-thread-per-task model. Prefer `java.lang.ScopedValue` (finalized and GA as of JEP 506, JDK 25 — confirm the JDK version in scope is 25 or later before recommending it as available; on earlier versions it is preview or incubator) for immutable per-task/per-call-tree sharing, or explicit task-scoped parameters where `ScopedValue` is not available.

## Structured concurrency is a preview feature — verify before relying on it

`StructuredTaskScope` has iterated through incubator status (JDK 19–20) and multiple preview iterations from JDK 21 onward (JEPs 453, 462, 480, 499, 505, 525, 533 across successive releases), and had not reached General Availability as of the most recent primary-source check performed for this file — JEP 533, "Structured Concurrency (Seventh Preview)," was the latest iteration found, targeted at a still-unreleased JDK release. Never present `StructuredTaskScope` as stable API surface. Confirm `--enable-preview` is set and identify the exact preview iteration/JDK release the reviewed code targets before evaluating its usage, since the API shape (method names, scope-joiner types) has changed across preview iterations and code written against one preview may not compile against another.

## Known uncertainty

- Structured concurrency's JEP number and preview iteration will continue to change release over release until finalization; re-check the JEP index (openjdk.org/jeps/0) rather than trusting this file's JEP number as current.
- `ScopedValue`'s GA status (JEP 506, JDK 25) applies from JDK 25 onward only; confirm the JDK version in scope rather than assuming it is available.
