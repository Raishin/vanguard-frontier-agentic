# Event-Loop Ordering Trace Patterns

Use this reference only when a review requires tracing the actual resolution order of a specific microtask/macrotask/`await` sequence — not for general async-code review; use `rejection-audit.md` or `race-condition-patterns.md` for those.

## What people get wrong

The common bad assumption is:

> "`async`/`await` reads top-to-bottom, so it executes top-to-bottom relative to everything else."

That is wrong. `async`/`await` is sugar over Promises. Every `await` suspends the async function and schedules its continuation as a **microtask**; it does not pause the rest of the program. Code after the async function call keeps running synchronously until the call stack empties, and only then does the microtask queue drain — completely, including any new microtasks enqueued during that drain — before the next macrotask (`setTimeout`, `setInterval`, I/O callback, UI paint) runs.

## Officially grounded ordering rules (MDN)

Two queue classes, not one:

- **Task queue (macrotasks):** initial script execution, `setTimeout`/`setInterval` callbacks, event dispatch, I/O. When a new event-loop iteration begins, the runtime executes exactly the next task from the task queue. Tasks queued during that iteration wait for the *next* iteration.
- **Microtask queue:** Promise `.then`/`.catch`/`.finally` callbacks, `async`/`await` continuations, `queueMicrotask()`. Whenever a task exits and the call stack is empty, **all** microtasks run in turn — including ones newly enqueued by currently-running microtasks — until the microtask queue is fully empty. Only then does the next macrotask run.

Canonical worked example (MDN, `Guide/Using_promises`):

```js
const wait = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

wait(0).then(() => console.log(4));
Promise.resolve()
  .then(() => console.log(2))
  .then(() => console.log(3));
console.log(1);
// Output: 1, 2, 3, 4
```

`wait(0)` still goes through `setTimeout`, so its `.then` callback is a macrotask-queued continuation — it runs *after* all currently-queryable microtasks, even though its delay is `0` and even though it was scheduled first in source order. Do not accept "it's `setTimeout(fn, 0)` so it's basically synchronous" as a review claim; it is not.

Second canonical example, showing microtasks interleaving with `await` continuations (MDN, `Reference/Operators/await`):

```js
let i = 0;
queueMicrotask(function test() {
  i++;
  console.log("microtask", i);
  if (i < 3) queueMicrotask(test);
});

(async () => {
  console.log("async function start");
  for (let i = 1; i < 3; i++) {
    await null;
    console.log("async function resume", i);
  }
  await null;
  console.log("async function end");
})();

queueMicrotask(() => console.log("queueMicrotask() after calling async function"));
console.log("script sync part end");

// Logs, in order:
// async function start
// script sync part end
// microtask 1
// async function resume 1
// queueMicrotask() after calling async function
// microtask 2
// async function resume 2
// microtask 3
// async function end
```

The load-bearing detail: each `await null` inside the async function re-enters the *back* of the microtask queue on resume, so it interleaves with other pending microtasks rather than running immediately after the previous line. A review claim that "the loop finishes before the other microtasks run" is wrong given this trace — verify against the actual queue order, don't assert it.

## Non-negotiable design rules

1. **Never assert an ordering claim without naming which queue each operation lands on.** "This runs first" is not a finding; "this is a microtask (Promise `.then`) so it runs before that macrotask (`setTimeout`), regardless of the `setTimeout` delay value" is.
2. **`setTimeout(fn, 0)` (or any small delay) is not "basically synchronous" and is not "basically a microtask."** It is a macrotask. All pending microtasks — including ones enqueued while draining the current batch — run before it, no matter how small the delay.
3. **A `for`/`while` loop containing multiple `await` points interleaves with other queued microtasks on every resume**, not just at the start and end of the loop. Do not assume the loop runs to completion "in one go" once started.
4. **`Promise.resolve().then(...)` chains queue one microtask per `.then` link.** A five-link `.then` chain takes five microtask-queue drains to fully resolve, and other pending microtasks interleave between each link if they were already queued.
5. **Rendering/painting is a macrotask-adjacent checkpoint, not a microtask checkpoint.** Layout/paint work happens after the microtask queue drains and before the next macrotask in browsers implementing the HTML spec's rendering opportunity model — do not assume a DOM mutation made inside a microtask is guaranteed visible to the user before the *next* microtask runs; it is not guaranteed until a rendering opportunity occurs.

## Verification targets

When repo evidence is available, verify a disputed ordering claim by:

- identifying every `Promise`-returning call, `.then`/`.catch`/`.finally`, `await`, and `queueMicrotask` in the sequence and labeling each a microtask source,
- identifying every `setTimeout`/`setInterval`/event-dispatch/I-O callback and labeling each a macrotask source,
- walking the sequence in source order, applying "drain all microtasks (including newly enqueued ones) before the next macrotask" at each synchronous-execution boundary,
- if the trace is non-obvious or contested, recommend the reviewer actually run the snippet (`node` REPL or browser console) rather than settle the dispute by further reasoning alone — label the resulting claim `needs live-runtime verification` until that's done.

## When to push back

Push back if the user asks you to:

- assert an ordering claim "because `async`/`await` looks synchronous" without tracing the actual queue mechanics — that is exactly the intuition that produces production race conditions,
- treat `setTimeout(fn, 0)` as a synchronization primitive to "make sure the DOM update happened" — it is not deterministic relative to other macrotasks/microtasks queued by other code and is not a documented ordering guarantee,
- skip tracing a "probably fine" reordering in a diff without walking the actual microtask/macrotask sequence — "probably fine" is not evidence for a scheduling claim.
