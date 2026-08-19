# Input And Output Contracts

How to test whether a module's boundary actually constrains callers, and where each invariant belongs.

- A `validation` block is the only construct that rejects an invalid input before any plan work happens; a `description` and a README describe intent but enforce nothing, so an input whose invalid values break the module needs a validation rule rather than better prose.
- A type constraint and a validation rule answer different questions: the type rejects the wrong shape, the rule rejects the wrong value of the right shape. A `string` typed CIDR block is well-typed and still able to carry `not-a-cidr`.
- `nullable = false` makes a null input an error rather than a silent fallback to the default; leaving an optional input nullable means the module must handle null explicitly, and modules routinely do not, producing a failure deep inside a provider call rather than at the boundary.
- Marking an input `sensitive` suppresses it from plan and apply output. It does not encrypt it, does not remove it from state, and does not protect it at rest — the value is still written to state in the clear unless the backend or the engine encrypts state separately.
- An output that returns an entire resource object hands every caller a dependency on every attribute of that resource, including ones the module may need to change; narrowing the output to the attributes callers actually need is what keeps the implementation swappable.
- `precondition` and `postcondition` blocks assert around a resource and block the operation when they fail, while a `check` block runs as a continuous non-blocking assertion and reports without failing the run — an invariant that must stop a bad apply cannot be written as a `check` block.
- A change to a module's `count` or `for_each` key changes the resource addresses of every instance for every caller, which the engine sees as destroy-and-create rather than a rename unless `moved` blocks carry the old addresses forward.
