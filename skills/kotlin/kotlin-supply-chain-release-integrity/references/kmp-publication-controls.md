# KMP/Maven Publication Controls

What a KMP/Maven publication must carry for a consumer to trust and resolve it.

- Kotlin Multiplatform library publication documents the expected publication structure — per-target artifacts, POM coordinates, and Gradle Module Metadata — that a consumer's build tool relies on to resolve the correct target artifact.
- A publication missing source or documentation artifacts, or incomplete module metadata, does not prevent resolution but reduces what a consuming build (or a human reviewer) can verify about the published artifact without a separate signing/provenance step, which sits outside this skill's scope.

## Sources

- https://kotlinlang.org/docs/multiplatform-publish-lib.html
