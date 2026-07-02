# Stale Closures and Dependency Arrays

Use this reference only when a dependency-array omission or a stale-closure suspicion is present — a value is read inside an effect body but is missing from (or wrongly present in) the dependency array.

## What people get wrong

The common bad assumption is:

> "The effect's dependency array is a performance knob — trim it to reduce re-runs."

That is backwards. The dependency array is not a performance setting; it is a correctness contract. Every reactive value (props, state, and anything derived from them) that the effect body reads must be listed, or the effect closes over a stale value from the render in which it was created and keeps using that stale value until something else happens to cause a re-run. This is the documented root cause of "the effect uses an old value even though the state clearly updated."

## Officially grounded pattern: distinguishing stable from reactive values

Not every value used inside an effect is "reactive" in the sense that omitting it is a bug:

- **Provably stable, safe to omit:** the `setState` function returned by `useState`, the `dispatch` function returned by `useReducer`, and `ref.current` reads performed *inside* the effect body (refs themselves are not reactive; React guarantees `set` function identity is stable across renders).
- **Reactive, must be included if read:** any prop, any state variable, any value derived from props/state (including objects and functions recreated on every render, e.g., an inline object literal or arrow function passed as a prop).
- **Non-reactive but currently forces a re-run because it's a function/object identity that changes every render:** this is the actual problem `useEffectEvent` was designed to solve (see below) — the value is logically "the latest one," not something the effect should resynchronize over.

## Non-negotiable design rules

1. **Do not classify a missing dependency as "intentional" without evidence.** A dependency array that omits a value read in the effect body is a stale-closure finding by default. It is only safe when the omitted value is one of the provably-stable categories above, or the codebase has already isolated the non-reactive read into a mechanism designed for that purpose (see rule 3). "The author probably meant to do that" is not evidence.
2. **A dependency-array lint suppression (`// eslint-disable-next-line react-hooks/exhaustive-deps`) is itself a finding**, not a signal that the array is correct. Read the suppressed line and independently verify whether the omission is safe using rule 1's criteria.
3. **When the React version supports `useEffectEvent`** (stable as of React 19.2; confirm via `package.json` and Context7 before assuming availability), it is the documented mechanism for reading the *latest* value of a prop or state variable inside an effect without making that value reactive. Example from the docs — `onMessage` needs the current `isMuted` value without forcing the connection effect to re-run every time `isMuted` toggles:

   ```js
   function ChatRoom({ roomId }) {
     const [messages, setMessages] = useState([]);
     const [isMuted, setIsMuted] = useState(false);

     const onMessage = useEffectEvent(receivedMessage => {
       setMessages(msgs => [...msgs, receivedMessage]);
       if (!isMuted) {
         playSound();
       }
     });

     useEffect(() => {
       const connection = createConnection();
       connection.connect();
       connection.on('message', (receivedMessage) => {
         onMessage(receivedMessage);
       });
       return () => connection.disconnect();
     }, [roomId]); // ✅ All dependencies declared
   }
   ```

   The Effect Event itself (`onMessage`) is **not** reactive and must be omitted from the effect's dependency array — only `roomId` remains, because `roomId` is the value that should actually trigger resynchronization (reconnecting).
4. **On React versions without `useEffectEvent`**, do not recommend it. The pre-19.2 alternatives are: accept the value as a real dependency (and accept the effect re-running when it changes, if that is actually correct behavior), or store the latest value in a `ref` updated on every render and read `ref.current` inside the effect (a manual, documented workaround with the same intent but without the ergonomics or the "must be called during render" constraint of `useEffectEvent`). State which alternative applies and why the effect's actual resynchronization need (or lack thereof) supports it.
5. **A stale closure over a `setState` value called without the updater form is a separate, related finding.** `setCount(count + 1)` inside a callback that closes over an old `count` is a stale-closure bug even outside effects; inside an effect or an effect-scheduled callback, prefer the updater form `setCount(c => c + 1)` when the new value only depends on the previous value — this sidesteps the staleness question entirely rather than requiring the value to be a correct dependency.

## Adversarial checklist

Before accepting a dependency-array omission as safe, answer:

- Is the omitted value actually read inside the effect body, or only inside a nested function that is *not* called synchronously within the effect (e.g., only referenced in a comment or dead code)? If it's genuinely unread, there is no finding.
- Is the omitted value one of the provably-stable categories (setState/dispatch function, `ref.current`)? If not, is there a `useEffectEvent` wrapper already isolating it, and does the target React version actually support `useEffectEvent`?
- If the value were included as a real dependency, would the effect's re-run behavior actually be wrong (e.g., reconnecting a socket every time an unrelated piece of state changes)? If re-running is actually correct and just looks noisy, the fix is not to omit the dependency — it may be to reconsider whether that value belongs in the same effect at all.
- Does the effect call an inline object or array literal as a dependency (e.g., `[{ id: props.id }]`)? A new object identity is created every render, so this dependency is present but still causes the effect to re-run every render — a different but related finding: the fix is to depend on the primitive fields (`props.id`) instead of the object.

## Verification targets

- Confirm the installed React major (`package.json`) before citing `useEffectEvent` as available — it is a documented, comparatively recent addition (React 19.2). Repos on 18.x or earlier need the `ref`-based workaround instead.
- Cross-reference any stale-closure finding in an async effect against `effect-cleanup-and-race-conditions.md` — a missing request-parameter dependency (e.g., `page` omitted from a fetch effect's array) is often paired with a missing cancellation guard in the same effect, and both should be reported together as related findings, not two disconnected line items.

## When to push back

Push back if the user says:

- "just disable the lint rule" as the resolution for a stale-closure finding — the rule exists specifically to catch this class of bug; disabling it removes the signal without fixing the defect,
- "the value rarely changes so it's fine to omit" — "rarely" is not "never"; the bug still exists and will surface intermittently, which is the expensive-to-debug failure mode this skill exists to catch pre-merge,
- "add `// eslint-disable` everywhere this warns" as a blanket policy — each suppression must be independently verified per the adversarial checklist above, not applied wholesale.
