# Strangler Boundary Design

Use this reference only when the user needs the actual technical seam between legacy and modern code — a proxy/route table, a module federation boundary, or an in-page adapter layer — not for a high-level phase summary (that belongs in the main SKILL.md response).

## What people get wrong

The naive story is:

> "We'll put the new framework next to the old one and slowly move pages over."

That undersells the seam design problem. The seam is where almost all migration risk concentrates: shared routing, shared auth/session, shared global CSS/JS, and shared build tooling all leak across the boundary unless it is designed deliberately. A vague "next to" is not a seam; it is an accident waiting to surface in production.

Officially grounded seam patterns (Context7-verified where cited):

## 1. Route-table / reverse-proxy seam (cross-stack, coarse-grained)

The legacy app and the new app live at the same origin (or behind a shared edge proxy), and routing decides which stack serves which path. This is the seam Next.js documents directly: a `rewrites().fallback` config that proxies every unmatched route to the existing legacy site, so you migrate route-by-route while the legacy app keeps serving everything not yet migrated (Context7-verified: `vercel/next.js` docs, `rewrites.mdx`).

Design rules for this seam:
- Decide route ownership explicitly per URL prefix; do not let both stacks claim the same path under different conditions (e.g. query-param-based routing to two different apps at the same path is a maintenance trap).
- Shared navigation chrome (header/footer/nav) rendered by two different stacks will drift visually and behaviorally. Either keep it duplicated with a visual-regression gate on both, or extract it into a served fragment neither app owns independently.
- Session/auth cookies must be readable by both origins/apps identically, or you have silently created a security gate (see SKILL.md — this needs an explicit review, not an assumption that "cookies just work").

## 2. Directory/module coexistence seam (same-framework major upgrades, or App Router migrations)

Some frameworks explicitly support two routing/rendering models coexisting in the same codebase during migration — for example, Next.js `app/` and `pages/` directories running side by side, with official guidance to migrate in small incremental steps rather than all at once (Context7-verified: `vercel/next.js` docs, `app-router-migration.mdx`).

Design rules for this seam:
- Confirm the specific framework version actually supports this coexistence mode before planning around it — do not assume parity across major versions without checking current docs for that version.
- Treat each migrated unit (page, route segment, module) as independently revertible: moving a route from the old model to the new one should be revertible by moving it back, not by reverting a large multi-file commit.
- Watch for cross-cutting concerns that do not respect the module boundary: global CSS resets, shared state stores, and app-wide providers/interceptors often need to be dual-registered or bridged, and that bridging code is itself migration debt that must be tracked and later removed.

## 3. Adapter/wrapper seam (embedding new components inside legacy views, or vice versa)

A common fine-grained seam: mount a modern component (e.g. React) inside a specific DOM node that the legacy stack (e.g. jQuery/Backbone) still controls, via an explicit mount/unmount adapter. This is the right seam when there is no clean route-level boundary — e.g. a single complex widget inside an otherwise-legacy page.

Design rules for this seam:
- The adapter must own an explicit lifecycle contract: who calls mount, who calls unmount, and what happens on the legacy view's re-render (does it destroy and remount the modern component, or does it leave it alone?). An undefined lifecycle contract causes memory leaks or duplicate event listeners — this is the most common actual incident in this pattern.
- Do not share mutable state between the two stacks by direct object reference across the boundary. Pass data in, read data out via an explicit, versioned contract (props in, callback/event out), so either side can evolve independently.
- CSS leakage across the boundary (legacy global styles cascading into the modern component's markup, or vice versa) needs a stated containment strategy (scoped classes, shadow DOM, or a documented "no shared class names" rule) before this seam ships to production, not after a visual bug report.

## 4. Module federation / micro-frontend seam (large-scale, multi-team boundary)

When multiple teams own different areas of the surface, a build-time or runtime module federation boundary lets each area upgrade independently. This is a heavier seam — it introduces shared-dependency version negotiation (React/framework version skew between federated modules) as a first-class operational concern.

Design rules for this seam:
- Explicitly decide and document which shared dependencies (framework, design-system, state library) are singleton-shared vs. independently versioned per module. Undecided version skew is the top failure mode of federation-based strangling.
- Define a contract for cross-module navigation and shared auth/session state before any module ships, not as each new module is added ad hoc.

## Verification targets

- For any Next.js-specific claim above (rewrites/fallback config, app/pages coexistence), re-verify the exact config shape against the installed Next.js version's current docs before writing production config — minor/major versions have changed this surface across releases.
- For any other framework/bundler pairing not covered above, resolve the library via Context7 and query current docs for an "incremental adoption" or "migration guide" page before proposing a specific seam mechanism; do not assume feature parity with the Next.js/React examples above.

## When to push back

Push back if the user asks for:

- a seam with no explicit route/module ownership decision ("we'll figure out which app handles what dynamically")
- shared mutable state passed by direct reference across the legacy/modern boundary "to keep it simple"
- a module federation boundary with no shared-dependency version policy

Those are not simplifications. They are the exact places migrations go from "in progress" to "permanently half-broken."
