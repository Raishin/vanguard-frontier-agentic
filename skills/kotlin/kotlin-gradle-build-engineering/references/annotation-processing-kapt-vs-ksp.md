# Annotation Processing: Kapt Vs KSP

Why KSP is preferred over kapt, and what incremental processing requires.

- KSP (Kotlin Symbol Processing) is documented as significantly faster than kapt because it analyzes Kotlin code directly instead of kapt's approach of generating Java stub sources for annotation processors to consume.
- Incremental annotation processing is opt-in: a processor must declare itself isolating (per-file) or aggregating (whole-compilation) for the build to incrementally recompile only affected files instead of a full recompilation on every change.
- Where both a kapt-based and a KSP-based version of the same processor exist, using kapt is a documented, avoidable performance cost for a Kotlin-targeted project.

## Sources

- https://kotlinlang.org/docs/ksp-overview.html
