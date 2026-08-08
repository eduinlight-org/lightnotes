use std::fmt::Write as _;
use std::path::Path;

use dioxus::logger::tracing::error;

pub const KEY_LEN: usize = 32;

pub type DbKey = [u8; KEY_LEN];

const SERVICE: &str = "lightnotes";
const ENTRY: &str = "db-key";

pub fn to_hex(key: &DbKey) -> String {
  let mut hex = String::with_capacity(KEY_LEN * 2);
  for byte in key {
    let _ = write!(hex, "{byte:02x}");
  }

  hex
}

pub fn raw_key_literal(key: &DbKey) -> String {
  format!("'x''{}'''", to_hex(key))
}

fn from_hex(text: &str) -> Option<DbKey> {
  let text = text.trim();
  if text.len() != KEY_LEN * 2 {
    return None;
  }

  let mut key = [0u8; KEY_LEN];
  for (index, slot) in key.iter_mut().enumerate() {
    *slot = u8::from_str_radix(text.get(index * 2..index * 2 + 2)?, 16).ok()?;
  }

  Some(key)
}

fn generate() -> Option<DbKey> {
  let mut key = [0u8; KEY_LEN];
  match getrandom::fill(&mut key) {
    Ok(()) => Some(key),
    Err(err) => {
      error!("local store unavailable: cannot generate a database key: {err}");
      None
    }
  }
}

#[cfg(not(target_os = "android"))]
pub fn resolve(_data_dir: &Path) -> Option<DbKey> {
  let entry = match keyring::Entry::new(SERVICE, ENTRY) {
    Ok(entry) => entry,
    Err(err) => {
      error!("local store unavailable: cannot reach the OS keychain: {err}");
      return None;
    }
  };

  match entry.get_password() {
    Ok(stored) => match from_hex(&stored) {
      Some(key) => Some(key),
      None => {
        error!("local store unavailable: the keychain entry for {SERVICE}/{ENTRY} is malformed");
        None
      }
    },
    Err(keyring::Error::NoEntry) => {
      let key = generate()?;
      if let Err(err) = entry.set_password(&to_hex(&key)) {
        error!("local store unavailable: cannot store the database key in the OS keychain: {err}");
        return None;
      }

      Some(key)
    }
    Err(err) => {
      error!("local store unavailable: cannot read the database key from the OS keychain: {err}");
      None
    }
  }
}

#[cfg(target_os = "android")]
pub fn resolve(data_dir: &Path) -> Option<DbKey> {
  use std::io::Write as _;
  use std::os::unix::fs::OpenOptionsExt;

  if let Err(err) = std::fs::create_dir_all(data_dir) {
    error!("local store unavailable: cannot create directory {}: {err}", data_dir.display());
    return None;
  }

  let key_path = data_dir.join("db-key");

  match std::fs::read_to_string(&key_path) {
    Ok(stored) => match from_hex(&stored) {
      Some(key) => return Some(key),
      None => {
        error!("local store unavailable: the key file at {} is malformed", key_path.display());
        return None;
      }
    },
    Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
    Err(err) => {
      error!("local store unavailable: cannot read the key file at {}: {err}", key_path.display());
      return None;
    }
  }

  let key = generate()?;
  let write = std::fs::OpenOptions::new()
    .write(true)
    .create_new(true)
    .mode(0o600)
    .open(&key_path)
    .and_then(|mut file| file.write_all(to_hex(&key).as_bytes()));

  if let Err(err) = write {
    error!("local store unavailable: cannot write the key file at {}: {err}", key_path.display());
    return None;
  }

  Some(key)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn hex_round_trips_a_key() {
    let key: DbKey = std::array::from_fn(|index| index as u8);

    assert_eq!(from_hex(&to_hex(&key)), Some(key));
  }

  #[test]
  fn hex_rejects_malformed_entries() {
    assert_eq!(from_hex(""), None);
    assert_eq!(from_hex("abcd"), None);
    assert_eq!(from_hex(&"z".repeat(KEY_LEN * 2)), None);
  }

  #[test]
  fn generated_keys_are_not_all_zeroes() {
    let key = generate().expect("generate");

    assert_ne!(key, [0u8; KEY_LEN]);
  }
}
