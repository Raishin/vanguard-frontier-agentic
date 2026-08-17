# Refactoring Addresses And Releasing Resources

Carrying address changes in configuration, and the difference between releasing a resource and destroying it.

- A `moved` block tells the engine that an object recorded at one address is the same object now described at another; the engine renames it in state instead of destroying and recreating it, which is what makes a refactor free rather than an outage.
- `moved` blocks select modules, resources, and resources inside child modules, so a restructure that pushes resources down into a submodule is expressible without touching state directly.
- A `moved` block is superior to `state mv` for the same reason a migration file is superior to a manual database edit: it is reviewed, versioned, applied identically in every workspace, and does not depend on each operator repeating the same command correctly.
- Any change to a `for_each` key or a `count` index changes instance addresses, and the engine cannot infer that the new address is the old object; without a `moved` block the plan is a genuine destroy-and-create rather than a cosmetic difference.
- Deleting a resource block destroys the infrastructure; a `removed` block stops managing it and leaves it running. These are opposite outcomes reached by similar-looking edits, and the intended one must be stated before the diff is written.
- A released resource becomes invisible to every future plan, so releasing without recording the release elsewhere produces exactly the same operational result as an accidental orphan — the only difference is whether anyone knows.
- `prevent_destroy` does not survive deletion of the resource block that carries it, so a guard intended to protect a resource through a refactor protects it only while the block still exists.
