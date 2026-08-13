use dioxus::logger::tracing::warn;
use jni::objects::{JObject, JValue};
use jni::JNIEnv;

use super::{Notification, Permission, ScheduleAction, ScheduledReminder, SchedulerSupport};
use crate::state::date_math::local_ms_to_utc_ms;

const RECEIVER_CLASS: &str = "dev.lightnotes.mobile.ReminderReceiver";
const POST_NOTIFICATIONS: &str = "android.permission.POST_NOTIFICATIONS";
const EXTRA_TITLE: &str = "lightnotes.title";
const EXTRA_BODY: &str = "lightnotes.body";
const EXTRA_ID: &str = "lightnotes.id";

const RTC_WAKEUP: i32 = 0;
const FLAG_UPDATE_CURRENT: i32 = 0x0800_0000;
const FLAG_IMMUTABLE: i32 = 0x0400_0000;
const FLAG_NO_CREATE: i32 = 0x2000_0000;
const PERMISSION_GRANTED: i32 = 0;
const TIRAMISU: i32 = 33;

pub fn request_code(note_id: &str) -> i32 {
  let mut hash: u32 = 2_166_136_261;

  for byte in note_id.as_bytes() {
    hash ^= *byte as u32;
    hash = hash.wrapping_mul(16_777_619);
  }

  (hash & 0x7fff_ffff) as i32
}

fn with_env<R>(body: impl FnOnce(&mut JNIEnv, &JObject) -> jni::errors::Result<R>) -> Option<R> {
  let context = ndk_context::android_context();
  let vm = unsafe { jni::JavaVM::from_raw(context.vm().cast()) }.ok()?;
  let activity = unsafe { JObject::from_raw(context.context().cast()) };
  let mut env = vm.attach_current_thread().ok()?;

  match body(&mut env, &activity) {
    Ok(value) => Some(value),
    Err(err) => {
      let _ = env.exception_clear();
      warn!("android notification call failed: {err}");
      None
    }
  }
}

fn sdk_int(env: &mut JNIEnv) -> jni::errors::Result<i32> {
  env.get_static_field("android/os/Build$VERSION", "SDK_INT", "I")?.i()
}

fn service<'a>(env: &mut JNIEnv<'a>, activity: &JObject, name: &str) -> jni::errors::Result<JObject<'a>> {
  let name = env.new_string(name)?;

  env
    .call_method(
      activity,
      "getSystemService",
      "(Ljava/lang/String;)Ljava/lang/Object;",
      &[JValue::Object(&name)],
    )?
    .l()
}

fn put_string_extra(env: &mut JNIEnv, intent: &JObject, key: &str, value: &str) -> jni::errors::Result<()> {
  let key = env.new_string(key)?;
  let value = env.new_string(value)?;

  env.call_method(
    intent,
    "putExtra",
    "(Ljava/lang/String;Ljava/lang/String;)Landroid/content/Intent;",
    &[JValue::Object(&key), JValue::Object(&value)],
  )?;

  Ok(())
}

fn receiver_intent<'a>(env: &mut JNIEnv<'a>, activity: &JObject) -> jni::errors::Result<JObject<'a>> {
  let class = env.new_string(RECEIVER_CLASS)?;
  let receiver = env.call_static_method(
    "java/lang/Class",
    "forName",
    "(Ljava/lang/String;)Ljava/lang/Class;",
    &[JValue::Object(&class)],
  )?;

  env.new_object(
    "android/content/Intent",
    "(Landroid/content/Context;Ljava/lang/Class;)V",
    &[JValue::Object(activity), receiver.borrow()],
  )
}

fn pending_intent<'a>(
  env: &mut JNIEnv<'a>,
  activity: &JObject,
  reminder: Option<&ScheduledReminder>,
  note_id: &str,
  flags: i32,
) -> jni::errors::Result<JObject<'a>> {
  let intent = receiver_intent(env, activity)?;

  put_string_extra(env, &intent, EXTRA_ID, note_id)?;

  if let Some(reminder) = reminder {
    put_string_extra(env, &intent, EXTRA_TITLE, &reminder.title)?;
    put_string_extra(env, &intent, EXTRA_BODY, &reminder.body)?;
  }

  env
    .call_static_method(
      "android/app/PendingIntent",
      "getBroadcast",
      "(Landroid/content/Context;ILandroid/content/Intent;I)Landroid/app/PendingIntent;",
      &[
        JValue::Object(activity),
        JValue::Int(request_code(note_id)),
        JValue::Object(&intent),
        JValue::Int(flags),
      ],
    )?
    .l()
}

fn schedule(env: &mut JNIEnv, activity: &JObject, reminder: &ScheduledReminder) -> jni::errors::Result<()> {
  let manager = service(env, activity, "alarm")?;
  let pending = pending_intent(env, activity, Some(reminder), &reminder.note_id, FLAG_UPDATE_CURRENT | FLAG_IMMUTABLE)?;
  let fire_at_utc_ms = local_ms_to_utc_ms(reminder.fire_at_local_ms);

  env.call_method(
    &manager,
    "setExactAndAllowWhileIdle",
    "(IJLandroid/app/PendingIntent;)V",
    &[JValue::Int(RTC_WAKEUP), JValue::Long(fire_at_utc_ms), JValue::Object(&pending)],
  )?;

  Ok(())
}

fn cancel(env: &mut JNIEnv, activity: &JObject, note_id: &str) -> jni::errors::Result<()> {
  let manager = service(env, activity, "alarm")?;
  let pending = pending_intent(env, activity, None, note_id, FLAG_NO_CREATE | FLAG_IMMUTABLE)?;

  if pending.is_null() {
    return Ok(());
  }

  env.call_method(&manager, "cancel", "(Landroid/app/PendingIntent;)V", &[JValue::Object(&pending)])?;
  env.call_method(&pending, "cancel", "()V", &[])?;

  Ok(())
}

fn permission_state(env: &mut JNIEnv, activity: &JObject) -> jni::errors::Result<Permission> {
  if sdk_int(env)? < TIRAMISU {
    return Ok(Permission::Granted);
  }

  let name = env.new_string(POST_NOTIFICATIONS)?;
  let granted = env
    .call_method(activity, "checkSelfPermission", "(Ljava/lang/String;)I", &[JValue::Object(&name)])?
    .i()?;

  Ok(match granted == PERMISSION_GRANTED {
    true => Permission::Granted,
    false => Permission::Denied,
  })
}

const CHANNEL_ID: &str = "lightnotes-reminders";
const IMPORTANCE_DEFAULT: i32 = 3;
const OREO: i32 = 26;
const FALLBACK_ICON: i32 = 0x0108_00a4;

fn small_icon(env: &mut JNIEnv, activity: &JObject) -> jni::errors::Result<i32> {
  let info = env
    .call_method(activity, "getApplicationInfo", "()Landroid/content/pm/ApplicationInfo;", &[])?
    .l()?;
  let icon = env.get_field(&info, "icon", "I")?.i()?;

  Ok(match icon {
    0 => FALLBACK_ICON,
    icon => icon,
  })
}

fn ensure_channel(env: &mut JNIEnv, manager: &JObject) -> jni::errors::Result<()> {
  if sdk_int(env)? < OREO {
    return Ok(());
  }

  let id = env.new_string(CHANNEL_ID)?;
  let name = env.new_string("Reminders")?;
  let channel = env.new_object(
    "android/app/NotificationChannel",
    "(Ljava/lang/String;Ljava/lang/CharSequence;I)V",
    &[JValue::Object(&id), JValue::Object(&name), JValue::Int(IMPORTANCE_DEFAULT)],
  )?;

  env.call_method(
    manager,
    "createNotificationChannel",
    "(Landroid/app/NotificationChannel;)V",
    &[JValue::Object(&channel)],
  )?;

  Ok(())
}

fn post_notification(env: &mut JNIEnv, activity: &JObject, notification: &Notification) -> jni::errors::Result<()> {
  let manager = service(env, activity, "notification")?;
  ensure_channel(env, &manager)?;

  let builder = match sdk_int(env)? >= OREO {
    true => {
      let id = env.new_string(CHANNEL_ID)?;
      env.new_object(
        "android/app/Notification$Builder",
        "(Landroid/content/Context;Ljava/lang/String;)V",
        &[JValue::Object(activity), JValue::Object(&id)],
      )?
    }
    false => env.new_object(
      "android/app/Notification$Builder",
      "(Landroid/content/Context;)V",
      &[JValue::Object(activity)],
    )?,
  };

  let title = env.new_string(&notification.title)?;
  let body = env.new_string(&notification.body)?;
  let icon = small_icon(env, activity)?;

  env.call_method(
    &builder,
    "setContentTitle",
    "(Ljava/lang/CharSequence;)Landroid/app/Notification$Builder;",
    &[JValue::Object(&title)],
  )?;
  env.call_method(
    &builder,
    "setContentText",
    "(Ljava/lang/CharSequence;)Landroid/app/Notification$Builder;",
    &[JValue::Object(&body)],
  )?;
  env.call_method(&builder, "setSmallIcon", "(I)Landroid/app/Notification$Builder;", &[JValue::Int(icon)])?;
  env.call_method(&builder, "setAutoCancel", "(Z)Landroid/app/Notification$Builder;", &[JValue::Bool(1)])?;

  let built = env.call_method(&builder, "build", "()Landroid/app/Notification;", &[])?.l()?;

  env.call_method(
    &manager,
    "notify",
    "(ILandroid/app/Notification;)V",
    &[JValue::Int(request_code(&notification.title)), JValue::Object(&built)],
  )?;

  Ok(())
}

pub fn notify_now(notification: Notification) {
  with_env(|env, activity| post_notification(env, activity, &notification));
}

pub async fn support() -> SchedulerSupport {
  let permission = with_env(|env, activity| permission_state(env, activity)).unwrap_or(Permission::Unknown);

  SchedulerSupport {
    background: permission == Permission::Granted,
    permission,
  }
}

pub async fn request_permission() -> Permission {
  let requested = with_env(|env, activity| {
    if sdk_int(env)? < TIRAMISU {
      return Ok(true);
    }

    let name = env.new_string(POST_NOTIFICATIONS)?;
    let names = env.new_object_array(1, "java/lang/String", &name)?;

    env.call_method(
      activity,
      "requestPermissions",
      "([Ljava/lang/String;I)V",
      &[JValue::Object(&names), JValue::Int(0)],
    )?;

    Ok(true)
  });

  if requested.is_none() {
    return Permission::Unknown;
  }

  for _ in 0..150 {
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    match with_env(|env, activity| permission_state(env, activity)) {
      Some(Permission::Granted) => return Permission::Granted,
      Some(_) => continue,
      None => return Permission::Unknown,
    }
  }

  Permission::Denied
}

pub async fn apply(actions: Vec<ScheduleAction>) -> Vec<ScheduleAction> {
  let mut applied = Vec::new();

  for action in actions {
    let done = match &action {
      ScheduleAction::Set(reminder) => with_env(|env, activity| schedule(env, activity, reminder)).is_some(),
      ScheduleAction::Remove { note_id } => with_env(|env, activity| cancel(env, activity, note_id)).is_some(),
    };

    if done {
      applied.push(action);
    }
  }

  applied
}

pub async fn clear_all() {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn request_codes_are_stable_distinct_and_never_negative() {
    assert_eq!(request_code("note-1"), request_code("note-1"));
    assert_ne!(request_code("note-1"), request_code("note-2"));

    for id in ["", "note-1", "a-very-long-note-identifier-0123456789", "ünïcøde"] {
      assert!(request_code(id) >= 0);
    }
  }
}
