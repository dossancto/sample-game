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
    pub message: String,
    pub kind: ToastKind,
    pub duration: Duration,
}

impl ToastEvent {
    pub fn new(message: impl Into<String>, kind: ToastKind, duration: Duration) -> Self {
        Self {
            message: message.into(),
            kind,
            duration,
        }
    }

    pub fn info(message: impl Into<String>, duration: Duration) -> Self {
        Self::new(message, ToastKind::Info, duration)
    }

    pub fn warning(message: impl Into<String>, duration: Duration) -> Self {
        Self::new(message, ToastKind::Warning, duration)
    }

    pub fn error(message: impl Into<String>, duration: Duration) -> Self {
        Self::new(message, ToastKind::Error, duration)
    }
}
