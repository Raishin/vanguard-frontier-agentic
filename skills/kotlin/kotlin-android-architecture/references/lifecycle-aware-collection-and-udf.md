# Lifecycle-Aware Collection And Unidirectional Data Flow

How lifecycle-gated Flow collection and unidirectional data flow keep UI state consistent.

- repeatOnLifecycle(STARTED) suspends and cancels the block's coroutines when the lifecycle falls below STARTED and restarts them when it returns, so Flow collection pauses while the UI is backgrounded.
- collectAsStateWithLifecycle (Compose) and flowWithLifecycle wrap the same STARTED-gated behavior; collecting a Flow in a bare launch/collect with no lifecycle gate keeps the collector — and any upstream work it drives — running while backgrounded.
- Unidirectional data flow keeps a single source of truth: UI state flows down from the ViewModel, typically as a StateFlow, and user actions flow up as events, never as direct mutation of the ViewModel's state from the UI layer.

## Sources

- https://developer.android.com/topic/libraries/architecture/coroutines
- https://developer.android.com/topic/architecture
