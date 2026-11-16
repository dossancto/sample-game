use bevy::prelude::Event;
use std::time::Duration;

/// Visual importance of a toast notification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastKind {
    Info,
    Warning,
    Error,
}

/// Requests the menus plugin to display a toast notification.
#[derive(Event, Debug, Clone, PartialEq, Eq)]
pub struct ToastEvent {
    pub id: String,
    pub message: String,
    pub kind: ToastKind,
    pub duration: Duration,
}

impl ToastEvent {
    pub fn new(
        id: impl Into<String>,
        message: impl Into<String>,
        kind: ToastKind,
        duration: Duration,
    ) -> Self {
        Self {
            id: id.into(),
            message: message.into(),
            kind,
            duration,
        }
    }

    pub fn info(id: impl Into<String>, message: impl Into<String>, duration: Duration) -> Self {
        Self::new(id, message, ToastKind::Info, duration)
    }

    pub fn warning(id: impl Into<String>, message: impl Into<String>, duration: Duration) -> Self {
        Self::new(id, message, ToastKind::Warning, duration)
    }

    pub fn error(id: impl Into<String>, message: impl Into<String>, duration: Duration) -> Self {
        Self::new(id, message, ToastKind::Error, duration)
    }
}
