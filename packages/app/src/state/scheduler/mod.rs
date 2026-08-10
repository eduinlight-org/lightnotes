#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, PartialEq)]
pub struct Notification {
  pub title: String,
  pub body: String,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, PartialEq)]
pub struct ScheduledReminder {
  pub note_id: String,
  pub fire_at_local_ms: i64,
  pub title: String,
  pub body: String,
  pub payload_hash: String,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, PartialEq)]
pub enum ScheduleAction {
  Set(ScheduledReminder),
  Remove { note_id: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Permission {
  #[default]
  Unknown,
  Granted,
  Denied,
  Unsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SchedulerSupport {
  pub background: bool,
  pub permission: Permission,
}

#[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
mod notify;
#[cfg(any(target_os = "macos", target_os = "windows", target_os = "linux"))]
pub use notify::{deliver_blocking, notify_now};

#[cfg(any(target_os = "macos", target_os = "ios"))]
mod apple;
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub use apple::{apply, clear_all, request_permission, support};
#[cfg(target_os = "ios")]
pub use apple::notify_now;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::{apply, clear_all, request_permission, support};

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::{apply, clear_all, request_permission, support};

#[cfg(target_os = "android")]
mod android;
#[cfg(target_os = "android")]
pub use android::{apply, clear_all, notify_now, request_permission, support};

#[cfg(not(any(target_os = "macos", target_os = "ios", target_os = "windows", target_os = "linux", target_os = "android")))]
mod unsupported;
#[cfg(all(not(any(target_os = "macos", target_os = "ios", target_os = "windows", target_os = "linux", target_os = "android")), not(target_arch = "wasm32")))]
pub use unsupported::{apply, clear_all, notify_now};
#[cfg(not(any(target_os = "macos", target_os = "ios", target_os = "windows", target_os = "linux", target_os = "android")))]
pub use unsupported::{request_permission, support};
