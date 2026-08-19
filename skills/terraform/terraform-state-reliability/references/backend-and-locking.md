# Backend Selection And Locking

What a backend must provide to be safe, and how locking actually fails.

- State locking prevents two operations from writing the state file at the same time; without it, two concurrent applies produce a final state describing neither run, and the corruption surfaces on a later plan rather than at the time it happens.
- The S3 backend supports native locking through the `use_lockfile` argument, which stores a lock object alongside the state and requires object read, write, and delete permission on the lock path in addition to the state path.
- DynamoDB-based locking for the S3 backend is deprecated and documented for removal in a future minor version; both mechanisms may be configured at once during migration, but a configuration relying on DynamoDB alone carries scheduled breakage.
- A stuck lock and a held lock look identical from outside. The difference is whether the holding process is still alive, and that question is answered by the lock metadata and the run history, never by how long the caller has been waiting.
- `force-unlock` is safe only when the holder is confirmed dead, because the lock it removes is the mechanism that would otherwise prevent the concurrent write; breaking a live lock is the standard route to a corrupted state file.
- Backend durability, versioning, and access control are reliability properties of the state, not of the storage service: a backend whose objects can be overwritten without a recoverable prior version has no rollback path for a bad write, regardless of the service's advertised durability.
- Workspaces separate state within a single backend and a single access-control boundary; they are not an isolation mechanism between production and non-production, because anything able to reach the backend can reach every workspace in it.
