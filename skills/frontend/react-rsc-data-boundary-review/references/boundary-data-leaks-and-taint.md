# Boundary Data Leaks and the Taint API

Use this reference only when the review scope includes a prop passed from a Server Component to a Client Component, a `server-only` guard question, or a value that should have been tainted before crossing the boundary.

## What people get wrong

The naive assumption is:

> "It's a Server Component doing the fetching, so anything it computes and hands down as props is automatically safe."

Wrong. A Server Component running on the server can read anything the server process can reach — environment variables, database credentials, internal API tokens — but the moment any of that value is passed as a prop to a Client Component, React serializes it into the payload the browser receives. "Computed on the server" is not the same as "safe to send to the client." The recurring real failure mode is not a developer typing `password={rawSecret}` on purpose; it is a Server Component fetching a config or session object for its own server-side use, then forwarding the *entire* object unnarrowed to a Client Component because the object also happens to contain a few fields the client legitimately needs.

## Officially grounded rules

React's own reference documentation states the anti-pattern directly, using exactly this shape:

```js
export async function Dashboard(props) {
  // DO NOT DO THIS
  return <Overview password={process.env.API_PASSWORD} />;
}
```

(`documentation-based`, React reference: `experimental_taintUniqueValue`.)

React's documented mitigation is `experimental_taintUniqueValue`, called on the server before the value can ever reach a serialization boundary:

```js
import "server-only";
import { experimental_taintUniqueValue } from 'react';

experimental_taintUniqueValue(
  'Do not pass the API token password to the client. ' +
    'Instead do all fetches on the server.',
  process,
  process.env.API_PASSWORD
);
```

Once tainted, React throws if that exact value is later passed to a Client Component or a Server Function — turning an accidental leak into a hard error instead of a silent shipment to the browser. This API is experimental and only available inside Server Components (`documentation-based`, React reference).

The equally valid, non-experimental mitigation — and the one to recommend when the codebase cannot yet depend on an experimental API — is narrowing: return only the specific non-sensitive fields a Client Component needs, never the full object that also carries the sensitive ones (e.g. return `config.SERVICE_API_VERSION`, not `config`).

Next.js's own documentation independently reinforces the server/client environment-variable split this rests on: server-only values (e.g. `process.env.DATABASE_URL`) are read directly in Server Components, while values intended for client code must be exposed through the `NEXT_PUBLIC_` prefix convention — there is no other sanctioned path for a raw secret to reach client code (`documentation-based`, Next.js docs).

## Non-negotiable design rules

### 1. Trace every prop's value to its origin before judging it

Do not evaluate `<ClientComponent someProp={value} />` in isolation. Is `value` a literal? A prop passed through unchanged from further up? A field read from `process.env`? A field on a database row or API response? The finding depends on where that trace terminates, not on the prop name alone — though a prop named `password`, `apiKey`, `secret`, or `token` bound directly to an env-derived value is close to a self-evident finding.

### 2. Whole-object forwarding is a distinct, easy-to-miss failure mode

`<ClientComponent config={config} />`, where `config` is the full object returned by a server-side fetch, forwards every field the object contains — including any the developer never explicitly intended to expose. This does not require a developer to type a dangerous-looking field name; it only requires forgetting that the object holds more than what the Client Component actually renders. Always ask: does this Client Component receive the *exact* fields it needs, or the whole object those fields came from?

### 3. `server-only` guards the module, not the specific call site

`import 'server-only'` at the top of a module causes a build-time error if any Client Component (or a file in the client bundle graph) ever imports that module — but it only protects the module it is declared in. A sibling data-access file with the same kind of `process.env` read and no `server-only` import of its own is not covered by a guard that exists elsewhere in the codebase.

### 4. An unused prop still serialized is still a leak

If a Client Component receives a sensitive prop but never reads it in its render output, the value has still been serialized into the payload sent to the browser and is visible in dev tools, network inspection, or the page source. "It's not rendered anywhere" does not clear a `boundary-data-leak` finding.

### 5. Taint and narrowing are complementary, not alternatives to trace around

Do not accept "this codebase uses `experimental_taintUniqueValue` elsewhere" as clearing a specific untraced prop pass. Confirm the taint call (or the narrowing) is on the exact path from the sensitive origin to this specific Client Component boundary.

## Minimal safe implementation pattern

```tsx
// Safe: narrow to a specific non-sensitive field.
import { getSystemConfig } from './config' // has `import 'server-only'` at its top

export async function Dashboard() {
  const config = await getSystemConfig()
  return <ClientDashboard version={config.SERVICE_API_VERSION} />
}
```

Anti-pattern (do not approve):

```tsx
export async function Dashboard(props) {
  // DO NOT DO THIS
  return <Overview password={process.env.API_PASSWORD} />;
}
```

```tsx
// Also unsafe: whole object forwarded, even with no field literally named "password".
export async function Dashboard() {
  const config = await getSystemConfig()
  return <ClientDashboard config={config} />
}
```

## Adversarial checklist

Before clearing a Server Component → Client Component prop boundary as safe:

- What is the literal origin of every prop value — a literal, a narrowed field, or a full object/response?
- If a full object is forwarded, does it contain any field that reads `process.env`, a credential, or a token anywhere upstream in its construction?
- Is there a `server-only` import at the top of the specific module that reads the sensitive value, or only somewhere else in the codebase?
- Is there an `experimental_taintUniqueValue` call on the exact path from the sensitive value's origin to this boundary, or is narrowing the only control in place?
- Would a future code change (a new field added to the forwarded object, or a refactor that removes the narrowing) reach the client without re-triggering a security review?

If any answer is unclear or reveals a gap, the finding is HIGH — do not soften it to "worth double-checking."

## Verification targets

- Grep for JSX usages that pass a prop from a Server Component to an imported Client Component (a component whose file has `'use client'` at its top), and enumerate every prop value.
- Grep for `password=`, `apiKey=`, `secret=`, `token=` (and case variants) bound to any non-literal expression in JSX.
- Grep for a prop name identical to the bound variable name (`config={config}`-shaped whole-object forwarding).
- Grep every data-access module for `process.env.` reads and confirm `import 'server-only'` is present as the first statement in that same file.
- Grep for `experimental_taintUniqueValue(` calls and confirm which specific values they cover.
