# Breaking Change Classification

What counts as breaking for a module's callers, and why the answer is rarely visible in the diff size.

- Removing a variable, narrowing a type, adding a required variable without a default, or tightening a validation rule are all breaking for existing callers, because each turns a previously valid call into an error.
- Renaming or removing an output is breaking even when nothing inside the module changed, since callers reference outputs by name and the engine gives no compatibility shim for a renamed one.
- Changing a resource address inside a module — by renaming a resource, introducing `for_each`, or restructuring into a submodule — is breaking for callers' state even when the module's inputs and outputs are untouched, and requires `moved` blocks to avoid a destroy-and-create.
- Widening a type, adding an optional variable with a default, and adding a new output are non-breaking, which makes them the preferred shape for satisfying a caller's new requirement without a major version.
- A module published to a registry is versioned by tag, and callers pin with a version constraint; shipping a breaking change under a patch tag defeats every caller's pin at once and is the single most common cause of an unexplained mass replacement across an estate.
- A default value change is breaking in effect even though it is not breaking in signature: every caller that relied on the old default gets different infrastructure on their next apply with no diff in their own repository.
