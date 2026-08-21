# Platform Fragmentation And Golden Paths

How to judge whether a proposed module should exist, and when a platform ticket should become an input.

- Most requests for a new module are configuration requests wearing an architecture costume: when the difference between the proposed module and an existing platform module is a set of values rather than a set of resources, the correct answer is a new input on the existing module, not a fork.
- A wrapper module earns its existence only by adding something the wrapped module does not have — input validation, narrowed outputs, policy defaults, or an opinionated composition. A wrapper that only passes variables through adds an upgrade hop for every future change and provides nothing in return.
- Every fork of a platform module multiplies the cost of every future provider upgrade, policy change, and security fix by the number of forks, and that cost is paid by the platform team rather than by the team that forked.
- Module depth is a cost: each nesting level makes plan output harder to attribute, makes `moved` blocks harder to author, and makes a caller's upgrade depend on a chain of version bumps rather than one.
- A recurring platform ticket is a design signal, not a support load: the same request arriving repeatedly means the golden path is missing an input or an option, and the durable fix is to widen the module rather than to answer the ticket faster.
- Adoption is the only honest measure of a golden path. A platform module nobody uses is not a standard; counting published modules rather than callers per module reports the opposite of what is happening.
