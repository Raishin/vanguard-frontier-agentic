# Composite widget patterns

> Load this reference only when the component in scope is a composite/compound widget — combobox, tabs, dialog, listbox, accordion, menu — where deep prop or context threading may be an intentional API rather than an architectural defect.

## What people get wrong

The naive story is:

> Any prop passed through 3+ components is prop drilling; any component with a lot of internal cross-talk is doing too much.

Wrong for composite widgets. A compound-component API (e.g. `<Tabs><Tabs.List><Tabs.Tab/></Tabs.List><Tabs.Panels>...</Tabs.Panels></Tabs>`) intentionally threads shared state (active index, orientation, disabled state) across a family of subcomponents via context or cloned props, because the alternative — flattening everything into one component — would produce a worse, less composable, less accessible API. Flagging that threading as "drilling" without checking intent produces a false-positive finding that erodes reviewer credibility.

## How to tell intentional threading from accidental drilling

- **Naming convention**: subcomponents namespaced under a parent (`Tabs.Tab`, `Menu.Item`, `Dialog.Trigger`) signal a deliberate compound-component API, not incidental nesting.
- **Shared context scoped to the family**: a context provider created and consumed only within the widget's own file/module (not app-wide) is a scoping signal of intentional design, not overbroad context.
- **ARIA role relationships**: if the DOM structure maps to a documented ARIA Authoring Practices Guide (APG) pattern — combobox, listbox, tablist/tab/tabpanel, menu/menuitem — the parent-child prop/state relationship usually exists to satisfy the accessibility contract (e.g., `aria-activedescendant`, `aria-selected`, `aria-expanded` synchronization across the family), not because of poor decomposition.
- **Reuse boundary**: subcomponents of a compound API are not meant to be reused independently outside their parent; that is by design, not a coupling defect. Do not recommend "extracting" a `Tabs.Tab` to be reused standalone unless the user has an actual standalone-reuse requirement.

## When to still flag a composite widget

Even for compound APIs, still flag:

- Hook-rule violations inside any subcomponent (unconditional top-level rule always applies).
- A shared context value that changes on every keystroke/interaction and re-renders every sibling subcomponent when only one needs to update — recommend splitting state (e.g., "active index" context separate from "orientation" context) or memoizing consumers.
- Hardcoded ARIA attributes that contradict the actual interaction model (e.g., `role="listbox"` on a widget that behaves like a menu) — this is a defect regardless of component-architecture concerns and should be flagged as an accessibility finding, not folded into an architecture finding.
- A compound API that silently duplicates state between a controlled prop and internal state without a documented reconciliation strategy (uncontrolled/controlled conflict).

## Reference pattern shape

React's own composition guidance underlies compound components: children are passed through `props.children`, and shared behavior is coordinated via context scoped to the family, consistent with `Sharing State Between Components` (lift state to the nearest common owner) and `Passing Data Deeply with Context` (use context when passing props becomes genuinely inconvenient across the family, not as a first resort).

## When to push back

Push back if the user asks to:

- flatten a compound-component API into a single component "to reduce prop drilling" — that removes composability without removing coupling,
- extract a namespaced subcomponent for standalone reuse without an actual standalone-reuse requirement,
- remove the family-scoped context and thread every value as explicit props across 4+ compound subcomponents — that reintroduces the exact verbosity context was scoped to solve.

That is not simplification. It is a worse API with the same underlying coupling.
