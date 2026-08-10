use std::path::PathBuf;
use std::process::Command;

use dioxus::logger::tracing::warn;

use super::notify::delivery_permission;
use super::{Permission, ScheduleAction, ScheduledReminder, SchedulerSupport};
use crate::state::date_math::date_ms_to_ymdhm;

const UNIT_PREFIX: &str = "lightnotes-reminder-";

pub fn unit_stem(note_id: &str) -> String {
  let sanitized: String = note_id
    .chars()
    .map(|character| match character.is_ascii_alphanumeric() {
      true => character,
      false => '-',
    })
    .collect();

  format!("{UNIT_PREFIX}{sanitized}")
}

pub fn on_calendar(fire_at_local_ms: i64) -> String {
  let (year, month, day, hour, minute) = date_ms_to_ymdhm(fire_at_local_ms);

  format!("{year:04}-{month:02}-{day:02} {hour:02}:{minute:02}:00")
}

pub fn escape_exec_argument(argument: &str) -> String {
  let escaped: String = argument
    .chars()
    .flat_map(|character| match character {
      '\\' | '"' => vec!['\\', character],
      '\n' | '\r' => vec![' '],
      character => vec![character],
    })
    .collect();

  format!("\"{escaped}\"")
}

pub fn timer_unit(fire_at_local_ms: i64, stem: &str) -> String {
  format!(
    "[Unit]\n\
     Description=LightNotes reminder\n\
     \n\
     [Timer]\n\
     OnCalendar={}\n\
     Persistent=true\n\
     AccuracySec=30s\n\
     Unit={stem}.service\n\
     \n\
     [Install]\n\
     WantedBy=timers.target\n",
    on_calendar(fire_at_local_ms)
  )
}

pub fn service_unit(executable: &str, title: &str, body: &str) -> String {
  format!(
    "[Unit]\n\
     Description=LightNotes reminder\n\
     \n\
     [Service]\n\
     Type=oneshot\n\
     ExecStart={} --notify-reminder {} {}\n",
    escape_exec_argument(executable),
    escape_exec_argument(title),
    escape_exec_argument(body)
  )
}

fn units_dir() -> Option<PathBuf> {
  let base = match std::env::var_os("XDG_CONFIG_HOME") {
    Some(config) if !config.is_empty() => PathBuf::from(config),
    _ => PathBuf::from(std::env::var_os("HOME")?).join(".config"),
  };

  Some(base.join("systemd").join("user"))
}

fn executable_path() -> Option<String> {
  let path = match std::env::var_os("APPIMAGE") {
    Some(appimage) if !appimage.is_empty() => PathBuf::from(appimage),
    _ => std::env::current_exe().ok()?,
  };

  Some(path.to_string_lossy().into_owned())
}

fn systemctl(arguments: &[&str]) -> bool {
  match Command::new("systemctl").arg("--user").args(arguments).output() {
    Ok(output) if output.status.success() => true,
    Ok(output) => {
      warn!("systemctl --user {:?} failed: {}", arguments, String::from_utf8_lossy(&output.stderr).trim());
      false
    }
    Err(err) => {
      warn!("systemctl --user {arguments:?} could not run: {err}");
      false
    }
  }
}

fn write_units(dir: &PathBuf, executable: &str, reminder: &ScheduledReminder) -> bool {
  let stem = unit_stem(&reminder.note_id);
  let service = service_unit(executable, &reminder.title, &reminder.body);
  let timer = timer_unit(reminder.fire_at_local_ms, &stem);

  if let Err(err) = std::fs::write(dir.join(format!("{stem}.service")), service) {
    warn!("could not write the reminder service unit: {err}");
    return false;
  }

  if let Err(err) = std::fs::write(dir.join(format!("{stem}.timer")), timer) {
    warn!("could not write the reminder timer unit: {err}");
    return false;
  }

  true
}

fn remove_units(dir: &PathBuf, note_id: &str) {
  let stem = unit_stem(note_id);

  systemctl(&["disable", "--now", &format!("{stem}.timer")]);
  std::fs::remove_file(dir.join(format!("{stem}.timer"))).ok();
  std::fs::remove_file(dir.join(format!("{stem}.service"))).ok();
}

pub async fn support() -> SchedulerSupport {
  let permission = delivery_permission();

  let Some(dir) = units_dir() else {
    return SchedulerSupport { background: false, permission };
  };

  if std::fs::create_dir_all(&dir).is_err() {
    warn!("background reminders unavailable: cannot create {}", dir.display());
    return SchedulerSupport { background: false, permission };
  }

  if !systemctl(&["--version"]) {
    warn!("background reminders unavailable: systemctl --user is not usable here");
    return SchedulerSupport { background: false, permission };
  }

  SchedulerSupport { background: true, permission }
}

pub async fn request_permission() -> Permission {
  delivery_permission()
}

pub async fn apply(actions: Vec<ScheduleAction>) -> Vec<ScheduleAction> {
  let (Some(dir), Some(executable)) = (units_dir(), executable_path()) else {
    return Vec::new();
  };

  if std::fs::create_dir_all(&dir).is_err() {
    return Vec::new();
  }

  let mut written = Vec::new();

  for action in &actions {
    match action {
      ScheduleAction::Set(reminder) => {
        if write_units(&dir, &executable, reminder) {
          written.push(action.clone());
        }
      }
      ScheduleAction::Remove { .. } => written.push(action.clone()),
    }
  }

  if written.is_empty() {
    return Vec::new();
  }

  systemctl(&["daemon-reload"]);

  let mut applied = Vec::new();

  for action in written {
    match &action {
      ScheduleAction::Set(reminder) => {
        let timer = format!("{}.timer", unit_stem(&reminder.note_id));
        if systemctl(&["enable", &timer]) && systemctl(&["restart", &timer]) {
          applied.push(action);
        }
      }
      ScheduleAction::Remove { note_id } => {
        remove_units(&dir, note_id);
        applied.push(action);
      }
    }
  }

  applied
}

pub async fn clear_all() {
  let Some(dir) = units_dir() else {
    return;
  };

  let Ok(entries) = std::fs::read_dir(&dir) else {
    return;
  };

  let stems: Vec<String> = entries
    .filter_map(|entry| entry.ok())
    .filter_map(|entry| entry.file_name().into_string().ok())
    .filter(|name| name.starts_with(UNIT_PREFIX) && name.ends_with(".timer"))
    .map(|name| name.trim_end_matches(".timer").to_string())
    .collect();

  for stem in stems {
    systemctl(&["disable", "--now", &format!("{stem}.timer")]);
    std::fs::remove_file(dir.join(format!("{stem}.timer"))).ok();
    std::fs::remove_file(dir.join(format!("{stem}.service"))).ok();
  }

  systemctl(&["daemon-reload"]);
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::state::date_math::ymdhm_to_date_ms;

  #[test]
  fn on_calendar_is_zero_padded_local_civil_time() {
    assert_eq!(on_calendar(ymdhm_to_date_ms(2026, 8, 9, 9, 5)), "2026-08-09 09:05:00");
    assert_eq!(on_calendar(ymdhm_to_date_ms(2026, 12, 31, 23, 59)), "2026-12-31 23:59:00");
  }

  #[test]
  fn unit_stems_never_leave_the_prefix_or_admit_path_separators() {
    assert_eq!(unit_stem("2f9a-41c7"), "lightnotes-reminder-2f9a-41c7");

    for hostile in ["../../etc/passwd", "a/b c.d", "..", "with\nnewline"] {
      let stem = unit_stem(hostile);

      assert!(stem.starts_with(UNIT_PREFIX));
      assert!(!stem.contains(['/', '\\', ' ', '.', '\n']));
    }
  }

  #[test]
  fn exec_arguments_survive_quotes_and_backslashes() {
    assert_eq!(escape_exec_argument("plain"), "\"plain\"");
    assert_eq!(escape_exec_argument("say \"hi\""), "\"say \\\"hi\\\"\"");
    assert_eq!(escape_exec_argument("back\\slash"), "\"back\\\\slash\"");
    assert_eq!(escape_exec_argument("two\nlines"), "\"two lines\"");
  }

  #[test]
  fn a_timer_unit_points_at_its_service_and_survives_downtime() {
    let unit = timer_unit(ymdhm_to_date_ms(2026, 8, 14, 9, 0), "lightnotes-reminder-abc");

    assert!(unit.contains("OnCalendar=2026-08-14 09:00:00"));
    assert!(unit.contains("Unit=lightnotes-reminder-abc.service"));
    assert!(unit.contains("Persistent=true"));
    assert!(unit.contains("WantedBy=timers.target"));
  }

  #[test]
  fn a_service_unit_passes_the_payload_as_separate_quoted_arguments() {
    let unit = service_unit("/usr/bin/light-notes", "Buy milk", "Due 14 Aug");

    assert!(unit.contains("Type=oneshot"));
    assert!(unit.contains("ExecStart=\"/usr/bin/light-notes\" --notify-reminder \"Buy milk\" \"Due 14 Aug\""));
  }

  #[test]
  fn a_title_containing_a_newline_cannot_forge_a_unit_directive() {
    let unit = service_unit("/usr/bin/light-notes", "evil\nExecStart=/bin/sh -c rm", "Due");
    let directives: Vec<&str> = unit.lines().filter(|line| line.starts_with("ExecStart=")).collect();

    assert_eq!(directives.len(), 1);
    assert!(directives[0].contains("\"evil ExecStart=/bin/sh -c rm\""));
  }
}
