# Type-Contract Review Checklist

The per-concern checklist applied to every typing review.

- Boundaries: no public function accepts or returns `Any` (explicit or implicit); `# type: ignore` is scoped with an error code and rationale.
- Runtime validation: every trust boundary validates input at runtime, not via annotations alone.
- Variance: mutable containers are invariant; `TypeVar` bounds/constraints constrain the intended set.
- Overloads: `@overload` signatures are consistent and the implementation satisfies each.
- Structured data: TypedDict required/optional keys are respected; no mutable default on a dataclass field or default argument.
- API stability: public signature/return changes are treated as versioned breaking changes.
