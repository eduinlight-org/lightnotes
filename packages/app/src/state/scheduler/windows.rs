use dioxus::logger::tracing::warn;
use windows::core::HSTRING;
use windows::Data::Xml::Dom::XmlDocument;
use windows::Foundation::DateTime;
use windows::UI::Notifications::{ScheduledToastNotification, ToastNotificationManager, ToastNotifier};

use super::notify::delivery_permission;
use super::{Permission, ScheduleAction, ScheduledReminder, SchedulerSupport};
use crate::state::date_math::local_ms_to_utc_ms;

const AUMID: &str = "dev.lightnotes.desktop";
const GROUP: &str = "lightnotes-reminders";
const TICKS_PER_MS: i64 = 10_000;
const UNIX_EPOCH_IN_WINDOWS_TICKS: i64 = 116_444_736_000_000_000;

pub fn windows_ticks_from_unix_ms(unix_ms: i64) -> i64 {
  UNIX_EPOCH_IN_WINDOWS_TICKS + unix_ms * TICKS_PER_MS
}

pub fn escape_xml(text: &str) -> String {
  text
    .chars()
    .flat_map(|character| match character {
      '&' => "&amp;".chars().collect::<Vec<char>>(),
      '<' => "&lt;".chars().collect(),
      '>' => "&gt;".chars().collect(),
      '"' => "&quot;".chars().collect(),
      '\'' => "&apos;".chars().collect(),
      character => vec![character],
    })
    .collect()
}

pub fn toast_xml(title: &str, body: &str) -> String {
  format!(
    "<toast><visual><binding template=\"ToastGeneric\"><text>{}</text><text>{}</text></binding></visual></toast>",
    escape_xml(title),
    escape_xml(body)
  )
}

fn register_app_user_model_id() -> bool {
  let key = match windows_registry::CURRENT_USER.create(format!("Software\\Classes\\AppUserModelId\\{AUMID}")) {
    Ok(key) => key,
    Err(err) => {
      warn!("could not register the notification app id: {err}");
      return false;
    }
  };

  if let Err(err) = key.set_string("DisplayName", "LightNotes") {
    warn!("could not name the notification app id: {err}");
    return false;
  }

  true
}

fn notifier() -> Option<ToastNotifier> {
  match ToastNotificationManager::CreateToastNotifierWithId(&HSTRING::from(AUMID)) {
    Ok(notifier) => Some(notifier),
    Err(err) => {
      warn!("background reminders unavailable: no toast notifier for {AUMID}: {err}");
      None
    }
  }
}

fn remove_scheduled(notifier: &ToastNotifier, note_id: &str) {
  let Ok(scheduled) = notifier.GetScheduledToastNotifications() else {
    return;
  };

  let Ok(count) = scheduled.Size() else {
    return;
  };

  for index in 0..count {
    let Ok(item) = scheduled.GetAt(index) else {
      continue;
    };

    if item.Tag().map(|tag| tag.to_string_lossy() == note_id).unwrap_or(false) {
      if let Err(err) = notifier.RemoveFromSchedule(&item) {
        warn!("could not unschedule a reminder toast: {err}");
      }
    }
  }
}

fn schedule(notifier: &ToastNotifier, reminder: &ScheduledReminder) -> bool {
  let document = match XmlDocument::new() {
    Ok(document) => document,
    Err(err) => {
      warn!("could not create the toast document: {err}");
      return false;
    }
  };

  if let Err(err) = document.LoadXml(&HSTRING::from(toast_xml(&reminder.title, &reminder.body))) {
    warn!("could not load the toast payload: {err}");
    return false;
  }

  let delivery = DateTime {
    UniversalTime: windows_ticks_from_unix_ms(local_ms_to_utc_ms(reminder.fire_at_local_ms)),
  };

  let scheduled = match ScheduledToastNotification::CreateScheduledToastNotification(&document, delivery) {
    Ok(scheduled) => scheduled,
    Err(err) => {
      warn!("could not build a scheduled toast: {err}");
      return false;
    }
  };

  if scheduled.SetTag(&HSTRING::from(&reminder.note_id)).is_err() || scheduled.SetGroup(&HSTRING::from(GROUP)).is_err() {
    warn!("could not label a scheduled toast");
    return false;
  }

  remove_scheduled(notifier, &reminder.note_id);

  match notifier.AddToSchedule(&scheduled) {
    Ok(()) => true,
    Err(err) => {
      warn!("could not schedule a reminder toast: {err}");
      false
    }
  }
}

pub async fn support() -> SchedulerSupport {
  let permission = delivery_permission();

  if !register_app_user_model_id() {
    return SchedulerSupport { background: false, permission };
  }

  SchedulerSupport { background: notifier().is_some(), permission }
}

pub async fn request_permission() -> Permission {
  delivery_permission()
}

pub async fn apply(actions: Vec<ScheduleAction>) -> Vec<ScheduleAction> {
  let Some(notifier) = notifier() else {
    return Vec::new();
  };

  let mut applied = Vec::new();

  for action in actions {
    match &action {
      ScheduleAction::Set(reminder) => {
        if schedule(&notifier, reminder) {
          applied.push(action);
        }
      }
      ScheduleAction::Remove { note_id } => {
        remove_scheduled(&notifier, note_id);
        applied.push(action);
      }
    }
  }

  applied
}

pub async fn clear_all() {
  let Some(notifier) = notifier() else {
    return;
  };

  let Ok(scheduled) = notifier.GetScheduledToastNotifications() else {
    return;
  };

  let Ok(count) = scheduled.Size() else {
    return;
  };

  for index in (0..count).rev() {
    let Ok(item) = scheduled.GetAt(index) else {
      continue;
    };

    if item.Group().map(|group| group.to_string_lossy() == GROUP).unwrap_or(false) {
      notifier.RemoveFromSchedule(&item).ok();
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::state::date_math::MS_PER_HOUR;

  #[test]
  fn the_unix_epoch_lands_on_the_documented_windows_tick_count() {
    assert_eq!(windows_ticks_from_unix_ms(0), 116_444_736_000_000_000);
    assert_eq!(windows_ticks_from_unix_ms(1) - windows_ticks_from_unix_ms(0), 10_000);
    assert_eq!(
      windows_ticks_from_unix_ms(MS_PER_HOUR) - windows_ticks_from_unix_ms(0),
      36_000_000_000
    );
  }

  #[test]
  fn xml_escaping_covers_every_character_that_could_break_the_payload() {
    assert_eq!(escape_xml("a & b"), "a &amp; b");
    assert_eq!(escape_xml("<text>"), "&lt;text&gt;");
    assert_eq!(escape_xml("say \"hi\""), "say &quot;hi&quot;");
    assert_eq!(escape_xml("it's"), "it&apos;s");
  }

  #[test]
  fn a_hostile_title_cannot_inject_extra_toast_elements() {
    let xml = toast_xml("</text><text>injected", "Due");

    assert_eq!(xml.matches("<text>").count(), 2);
    assert!(xml.contains("&lt;/text&gt;&lt;text&gt;injected"));
  }
}
