use std::cell::RefCell;
use std::ptr::NonNull;

use block2::RcBlock;
use dioxus::logger::tracing::warn;
use futures_channel::oneshot;
use objc2::rc::Retained;
use objc2::runtime::Bool;
use objc2_foundation::{NSArray, NSBundle, NSDateComponents, NSError, NSString};
use objc2_user_notifications::{
  UNAuthorizationOptions, UNAuthorizationStatus, UNCalendarNotificationTrigger, UNMutableNotificationContent, UNNotificationRequest,
  UNNotificationSettings, UNUserNotificationCenter,
};

use super::{Permission, ScheduleAction, ScheduledReminder, SchedulerSupport};
use crate::state::date_math::date_ms_to_ymdhm;

fn center() -> Option<Retained<UNUserNotificationCenter>> {
  NSBundle::mainBundle().bundleIdentifier()?;

  let center = UNUserNotificationCenter::currentNotificationCenter();

  #[cfg(target_os = "ios")]
  foreground::install(&center);

  Some(center)
}

async fn authorization_status() -> Option<UNAuthorizationStatus> {
  let center = center()?;
  let (tx, rx) = oneshot::channel();
  let tx = RefCell::new(Some(tx));

  let handler = RcBlock::new(move |settings: NonNull<UNNotificationSettings>| {
    let status = unsafe { settings.as_ref() }.authorizationStatus();
    if let Some(tx) = tx.borrow_mut().take() {
      let _ = tx.send(status);
    }
  });

  center.getNotificationSettingsWithCompletionHandler(&handler);

  rx.await.ok()
}

fn permission_of(status: UNAuthorizationStatus) -> Permission {
  match status {
    UNAuthorizationStatus::Authorized | UNAuthorizationStatus::Provisional => Permission::Granted,
    UNAuthorizationStatus::Denied => Permission::Denied,
    _ => Permission::Unknown,
  }
}

#[cfg(target_os = "macos")]
fn unbundled_permission() -> Permission {
  super::notify::delivery_permission()
}

#[cfg(target_os = "ios")]
fn unbundled_permission() -> Permission {
  Permission::Unsupported
}

pub async fn support() -> SchedulerSupport {
  let Some(status) = authorization_status().await else {
    warn!("background reminders unavailable: not running from an application bundle");
    return SchedulerSupport { background: false, permission: unbundled_permission() };
  };

  let permission = permission_of(status);

  SchedulerSupport { background: permission == Permission::Granted, permission }
}

pub async fn request_permission() -> Permission {
  let Some(center) = center() else {
    return Permission::Unsupported;
  };

  let (tx, rx) = oneshot::channel();
  let tx = RefCell::new(Some(tx));

  let handler = RcBlock::new(move |granted: Bool, error: *mut NSError| {
    if !error.is_null() {
      warn!("notification authorization failed: {:?}", unsafe { &*error });
    }
    if let Some(tx) = tx.borrow_mut().take() {
      let _ = tx.send(granted.as_bool());
    }
  });

  let options = UNAuthorizationOptions::Alert | UNAuthorizationOptions::Sound;
  center.requestAuthorizationWithOptions_completionHandler(options, &handler);

  match rx.await {
    Ok(true) => Permission::Granted,
    Ok(false) => Permission::Denied,
    Err(_) => Permission::Unknown,
  }
}

fn trigger_for(fire_at_local_ms: i64) -> Retained<UNCalendarNotificationTrigger> {
  let (year, month, day, hour, minute) = date_ms_to_ymdhm(fire_at_local_ms);
  let components = NSDateComponents::new();

  components.setYear(year as isize);
  components.setMonth(month as isize);
  components.setDay(day as isize);
  components.setHour(hour as isize);
  components.setMinute(minute as isize);
  components.setSecond(0);

  UNCalendarNotificationTrigger::triggerWithDateMatchingComponents_repeats(&components, false)
}

fn schedule(center: &UNUserNotificationCenter, reminder: &ScheduledReminder) {
  let content = UNMutableNotificationContent::new();
  content.setTitle(&NSString::from_str(&reminder.title));
  content.setBody(&NSString::from_str(&reminder.body));

  let request = UNNotificationRequest::requestWithIdentifier_content_trigger(
    &NSString::from_str(&reminder.note_id),
    &content,
    Some(&trigger_for(reminder.fire_at_local_ms)),
  );

  center.addNotificationRequest_withCompletionHandler(&request, None);
}

fn cancel(center: &UNUserNotificationCenter, note_id: &str) {
  let identifiers = NSArray::from_retained_slice(&[NSString::from_str(note_id)]);

  center.removePendingNotificationRequestsWithIdentifiers(&identifiers);
}

pub async fn apply(actions: Vec<ScheduleAction>) -> Vec<ScheduleAction> {
  let Some(center) = center() else {
    return Vec::new();
  };

  for action in &actions {
    match action {
      ScheduleAction::Set(reminder) => schedule(&center, reminder),
      ScheduleAction::Remove { note_id } => cancel(&center, note_id),
    }
  }

  actions
}

#[cfg(target_os = "ios")]
pub fn notify_now(notification: super::Notification) {
  let Some(center) = center() else {
    return;
  };

  let content = UNMutableNotificationContent::new();
  content.setTitle(&NSString::from_str(&notification.title));
  content.setBody(&NSString::from_str(&notification.body));

  let identifier = NSString::from_str(&format!("immediate-{}", notification.title));
  let request = UNNotificationRequest::requestWithIdentifier_content_trigger(&identifier, &content, None);

  center.addNotificationRequest_withCompletionHandler(&request, None);
}

pub async fn clear_all() {
  let Some(center) = center() else {
    return;
  };

  center.removeAllPendingNotificationRequests();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn an_unbundled_process_never_reaches_the_notification_center() {
    assert!(center().is_none());
  }
}

#[cfg(target_os = "ios")]
mod foreground {
  use std::sync::atomic::{AtomicBool, Ordering};

  use block2::DynBlock;
  use objc2::rc::Retained;
  use objc2::runtime::{NSObject, ProtocolObject};
  use objc2::{define_class, msg_send, MainThreadMarker, MainThreadOnly};
  use objc2_foundation::NSObjectProtocol;
  use objc2_user_notifications::{
    UNNotification, UNNotificationPresentationOptions, UNUserNotificationCenter, UNUserNotificationCenterDelegate,
  };

  define_class!(
    #[unsafe(super(NSObject))]
    #[name = "LightNotesNotificationDelegate"]
    #[thread_kind = MainThreadOnly]
    struct Delegate;

    unsafe impl NSObjectProtocol for Delegate {}

    unsafe impl UNUserNotificationCenterDelegate for Delegate {
      #[unsafe(method(userNotificationCenter:willPresentNotification:withCompletionHandler:))]
      fn will_present(
        &self,
        _center: &UNUserNotificationCenter,
        _notification: &UNNotification,
        completion: &DynBlock<dyn Fn(UNNotificationPresentationOptions)>,
      ) {
        completion.call((UNNotificationPresentationOptions::Banner | UNNotificationPresentationOptions::Sound,));
      }
    }
  );

  static INSTALLED: AtomicBool = AtomicBool::new(false);

  pub fn install(center: &UNUserNotificationCenter) {
    let Some(mtm) = MainThreadMarker::new() else {
      return;
    };

    if INSTALLED.swap(true, Ordering::Relaxed) {
      return;
    }

    let delegate: Retained<Delegate> = unsafe { msg_send![Delegate::alloc(mtm), init] };
    center.setDelegate(Some(ProtocolObject::from_ref(&*delegate)));

    std::mem::forget(delegate);
  }
}
