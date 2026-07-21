# Component Exposure And Intents

Exported components, intent surfaces, and PendingIntent safety.

- On API 31+ any component declaring an intent-filter must set `android:exported` explicitly; an exported component with no permission is reachable by any app.
- An implicit intent (no target component) can be intercepted; a mutable PendingIntent without `FLAG_IMMUTABLE` on API 31+ can be tampered — require explicit targets and immutability unless mutation is required.
- A content provider is exported by default below API 17 and must set `exported="false"` or a signature permission when it holds app data.

## Sources

- https://developer.android.com/guide/topics/manifest/activity-element#exported
- https://developer.android.com/reference/android/app/PendingIntent
