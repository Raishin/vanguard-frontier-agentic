# Ordering, Destroy Guards, And Decommissioning

How replacement ordering decides whether a replacement is an outage, and where destroy guards stop working.

- The default replacement order is destroy-then-create, which means the resource is absent for the duration; `create_before_destroy` inverts that so the replacement exists before the original is removed, and it is the difference between a rolling replacement and an outage.
- `create_before_destroy` propagates transitively: when a resource with it set depends on another resource, the engine enables the same behaviour on that dependency and records it in state, and a dependent resource cannot override it back to false because doing so would create a dependency cycle.
- A resource containing a destroy-time provisioner will not run that provisioner when `create_before_destroy` is enabled, so a teardown step implemented as a destroy provisioner is silently skipped by an ordering change made for availability reasons.
- `prevent_destroy` rejects a plan that would destroy the resource, but it does not prevent destruction caused by removing the resource block from configuration — deleting the block deletes the guard along with it, and no error is raised.
- Removing a resource from configuration and removing it from state are different operations with opposite outcomes: the first destroys the real infrastructure, the second leaves it running and unmanaged. Choosing between them is the central decision in any decommissioning plan.
- A destroy plan reverses dependency order, so the resources that fail first in a teardown are usually the ones nothing depended on during creation; reviewing a teardown by reading the creation order backwards is unreliable.
- Anything already removed from state before a destroy is not destroyed — it survives as an orphan that no configuration manages and no plan will ever show again. Orphans must be listed explicitly because no future plan will surface them.
