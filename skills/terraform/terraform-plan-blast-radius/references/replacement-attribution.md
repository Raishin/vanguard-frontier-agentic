# Replacement Attribution

Every reason the engine replaces a resource, and how to tell them apart in a plan.

- A plan's summary counts are not a risk measure: one destroy of a stateful resource outweighs a hundred additions, so a verdict issued from `N to add, N to change, N to destroy` is a verdict about arithmetic rather than about risk.
- A forced replacement is caused by a change to an attribute the provider cannot update in place; the plan names that attribute, and a replacement whose named attribute nobody intended to change usually means a provider default moved, a module default changed, or an upstream data source returned something new.
- `replace_triggered_by` causes a resource to be replaced because a different resource changed, so the replaced resource's own diff shows no cause at all — an unexplained replacement should be checked for a trigger before the provider is blamed.
- Address churn is the most common cause of a mass replacement: converting `count` to `for_each`, reordering a list that `count` indexes, changing a `for_each` key, renaming a resource, or moving resources into a submodule all change instance addresses, and the engine reads a changed address as destroy-and-create rather than as a rename.
- `moved` blocks are what tell the engine a changed address is the same object; without them the state has no way to connect the old address to the new one, and the plan is a genuine destroy-and-create rather than a display artifact.
- `ignore_changes` suppresses a difference between configuration and remote state rather than resolving it, so an attribute listed there is unmanaged in practice; `ignore_changes = all` means the resource is created by the configuration and thereafter owned by something else.
- A plan is only evidence about the provider versions that produced it. The same configuration planned under a different provider version can produce a different set of forced replacements, so a plan supplied without its provider versions supports inference, not confirmation.
