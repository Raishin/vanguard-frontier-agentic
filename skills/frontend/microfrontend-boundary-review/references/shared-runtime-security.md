# Shared-runtime security and CSP boundary

Use this reference only when a remote in scope handles sensitive data and same-runtime (non-iframe) composition is proposed or already in place. It grounds the CSP trust-boundary collapse risk that makes "we used module federation" an insufficient answer to "is this isolated."

## What people get wrong

The naive story is:

> Module federation loads remote code into my app via a script tag, same as any other bundle-splitting technique. That's just how modern frontend works — it's not a security boundary question.

Wrong. A `<script>`-loaded remote, whether fetched via module federation, dynamic `import()`, or any other non-iframe mechanism, executes in the same JavaScript realm as the host: same `window`, same DOM, same Content-Security-Policy context, same cookies and storage the host can reach. There is no process boundary and no origin boundary unless one is deliberately introduced. "It's just code splitting" is true from a bundler's perspective and false from a trust-boundary perspective — those are different questions with different answers.

## Officially grounded shape

- CSP (per MDN) is enforced per browsing context / document, not per script origin within that document. A single CSP header applies to the entire page — host and every same-runtime remote share it. A remote that needs a looser policy (e.g., to run its own inline styles or connect to its own API) cannot get one without loosening the policy for the host and every sibling remote, unless that remote is isolated into its own document (an iframe with its own CSP).
- An iframe, by contrast, is a separate browsing context with its own document and can carry its own CSP (via the iframe's own response headers, or constrained further by the host's `Content-Security-Policy: frame-src`/`child-src` and the iframe's `sandbox` attribute). This is the mechanism that actually creates an enforceable boundary between host and remote, not module federation's build-time module resolution.
- React's official docs confirm `createRoot` supports mounting multiple independent applications into one page (see `references/../SKILL.md` Context7 Documentation Protocol), which is the mechanism most same-runtime micro-frontend compositions use — this confirms the *mounting* pattern is supported, but says nothing about *security isolation* between those roots, because there is none by default: all roots share one JS realm.

## Non-negotiable design rules

1. **Isolation level is a data-sensitivity decision, not a tooling decision.** Do not let the choice of module federation vs. iframes be driven by developer convenience or performance preference alone when a remote handles data (payment details, PII, credentials, admin actions) that should not be reachable by a compromised or buggy sibling remote. Convenience does not override trust-boundary requirements.

2. **A compromised or buggy same-runtime remote can read/write anything the host can.** This includes DOM access to sibling remotes' rendered output, cookies and localStorage/sessionStorage the host has access to, and any global state or event bus shared across the composition. Treat a same-runtime remote as having the same privilege as the host itself, not a scoped subset of it.

3. **Third-party-owned or externally-built remotes are a stronger case for iframe isolation**, independent of data sensitivity, because the supply-chain trust level of the remote's build pipeline is not controlled by the host team. A remote built and deployed by a team outside the organization's own release process should default to iframe isolation unless there is a specific, documented reason and compensating control (e.g., contractual code-review gate, pinned immutable artifact, subresource integrity) to trust it in the shared runtime.

4. **Shared-dependency version drift is itself a security surface**, not just a stability one. An outdated shared dependency pulled in by version-range negotiation (rather than pinning) can reintroduce a patched vulnerability across the entire composition, and the host team may have no visibility into which remote's dependency requirement caused the downgrade.

## High-risk assumptions to kill

- "Module federation isolates the remote because it's a separate bundle." — False; bundle separation is a build-time concept, not a runtime trust boundary.
- "The remote only touches its own DOM subtree, so it can't affect the rest of the page." — False by default; nothing prevents a same-runtime remote's script from reaching outside its mount point unless the host specifically constrains it (and few do).
- "We trust the other team, so isolation doesn't matter." — Trust in a team does not substitute for isolation against a supply-chain compromise of that team's dependencies or build pipeline.
- "CSP is already strict, so we're covered." — A strict host-level CSP still applies uniformly to every same-runtime remote; it does not give any one remote a tighter or differently-scoped policy than the rest of the page.

## Safe verification targets

- The response headers actually served for the host document — confirm the CSP header text, not a policy described in a design doc.
- Whether any remote requiring isolation is served inside an `<iframe>` with its own `sandbox` attribute and (where relevant) its own CSP header, versus mounted via same-runtime `createRoot`/module federation.
- The federation config's `shared` block or equivalent, to confirm whether shared dependencies are pinned/singleton-enforced or resolved via an unpinned version range.

## When to push back

Push back if the user asks for:

- same-runtime composition for a remote handling payment, auth, or other sensitive data with no isolation mitigation and no documented compensating control,
- trusting a third-party-owned remote in the shared runtime solely because "we trust that vendor," with no supply-chain control (pinning, integrity checks, review gate) backing that trust,
- treating an unpinned shared-dependency version range as acceptable because "it hasn't broken yet."

Those are not pragmatic trade-offs. They are unreviewed risk acceptance dressed up as an architecture decision.
