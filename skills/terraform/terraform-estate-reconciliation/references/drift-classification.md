# Drift Classification And Disposition

The four kinds of drift, why the response differs, and how to observe drift without entangling it with a change.

- A `-refresh-only` plan reconciles state against remote objects without proposing configuration changes, which makes it the only way to see drift as a separate question from the change someone is trying to ship.
- Reviewing drift in a normal plan entangles two decisions, because the resulting apply resolves both the drift and the configuration change at once and afterwards no one can tell which change came from the repository and which from reality.
- Unauthorized change and authorized out-of-band fix look identical in a plan, and only the change record distinguishes them; reverting all drift by default silently rolls back the emergency fix someone made during an incident, usually at the worst possible moment.
- An externally owned attribute is not drift at all — it is a boundary the configuration failed to model, and the correct response is a scoped `ignore_changes` naming the owning system rather than a repeated reconciliation.
- Some apparent drift is a provider representation artifact: the remote object never changed, but the provider normalizes, reorders, or defaults an attribute differently than the configuration expresses it. Reverting these produces a permanent diff that never converges.
- `ignore_changes = all` converts a managed resource into one the configuration merely creates; anything it does afterwards is invisible, which is a legitimate arrangement only when another system is the declared owner.
- Unresolved drift is best measured by the age of the oldest unreconciled item, because a count drops when someone reverts everything without deciding and rises when detection improves — neither of which reflects whether the estate is under control.
