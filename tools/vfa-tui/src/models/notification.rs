//! Notification model — toast messages displayed in the TUI overlay.

use std::time::{Duration, Instant};

/// A transient in-application notification rendered as a toast overlay.
///
/// Notifications auto-dismiss after their [`ttl`](Notification::ttl) has elapsed.
/// `created_at` uses [`Instant`] rather than a wall-clock timestamp so the
/// auto-dismiss logic can compare elapsed time without clock skew.
#[derive(Debug, Clone)]
pub struct Notification {
    /// Human-readable message to display.
    pub message: String,
    /// How urgently this notification should be presented.
    pub severity: NotificationSeverity,
    /// Monotonic timestamp of when the notification was created.
    pub created_at: Instant,
    /// Duration after `created_at` at which the notification should be dismissed.
    pub ttl: Duration,
}

impl Notification {
    /// Create a notification that will auto-dismiss after `ttl`.
    pub fn new(message: String, severity: NotificationSeverity, ttl: Duration) -> Self {
        Self {
            message,
            severity,
            created_at: Instant::now(),
            ttl,
        }
    }

    /// Returns `true` if the notification has lived past its TTL.
    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed() >= self.ttl
    }
}

/// Severity classification for [`Notification`] messages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotificationSeverity {
    Info,
    Warning,
    Error,
    Success,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn notification_severity_equality() {
        assert_eq!(NotificationSeverity::Info, NotificationSeverity::Info);
        assert_ne!(NotificationSeverity::Info, NotificationSeverity::Error);
        assert_eq!(NotificationSeverity::Success, NotificationSeverity::Success);
    }

    #[test]
    fn notification_new_sets_fields() {
        let msg = "Catalog reloaded".to_string();
        let ttl = Duration::from_secs(5);
        let n = Notification::new(msg.clone(), NotificationSeverity::Success, ttl);

        assert_eq!(n.message, msg);
        assert_eq!(n.severity, NotificationSeverity::Success);
        assert_eq!(n.ttl, ttl);
        // Should not be expired immediately after construction.
        assert!(!n.is_expired());
    }

    #[test]
    fn notification_is_expired_with_zero_ttl() {
        // A zero-TTL notification should be considered expired right away
        // (elapsed >= 0 is always true).
        let n = Notification::new(
            "gone".to_string(),
            NotificationSeverity::Info,
            Duration::ZERO,
        );
        assert!(n.is_expired());
    }

    #[test]
    fn notification_clone_preserves_severity() {
        let n = Notification::new(
            "test".to_string(),
            NotificationSeverity::Warning,
            Duration::from_secs(3),
        );
        let cloned = n.clone();
        assert_eq!(cloned.severity, NotificationSeverity::Warning);
        assert_eq!(cloned.message, "test");
    }

    #[test]
    fn notification_debug_does_not_panic() {
        let n = Notification::new(
            "debug test".to_string(),
            NotificationSeverity::Error,
            Duration::from_secs(10),
        );
        let _ = format!("{:?}", n);
    }
}
