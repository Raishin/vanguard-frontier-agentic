# Source Addresses And Registry Resolution

How a name becomes code, where the name can betray the author, and what a registry actually attests to.

- A provider source address names a registry host, a namespace, and a type, and the local name a configuration uses is only an alias — two different packages can be referenced by the same local name, and the configuration body reads identically either way.
- Terraform and OpenTofu resolve unqualified or legacy provider references to different default registries, so the same configuration text can install different packages depending on which engine runs it; an assessment that does not name the engine is incomplete.
- Namespace confusion is the practical attack: a package published under a namespace that resembles the intended vendor's is indistinguishable in the configuration and distinguishable only by checking the source address against the provider's own documentation.
- A registry attests to publication and, where signing metadata exists, to who published a package. It does not attest that the code is safe, maintained, or unmodified in intent, so registry presence is never a substitute for a review decision.
- Module sources are not tracked by the dependency lock file at all — it records provider dependencies only — so a module reference is re-resolved on every `init` unless the reference itself is immutable.
- A Git branch or a mutable tag as a module source means the code can change with no diff in the consuming repository and no signal in any review; only a commit reference is actually pinned.
- Trust is not transitive by default: a reviewed module that references another module from an unreviewed source silently widens the trust boundary, so the enumeration must follow the module tree rather than stopping at the diff.
