use std::io::Read;
use std::path::{Path, PathBuf};

use dioxus::logger::tracing::{error, info};
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{AssertSqlSafe, ConnectOptions, Connection};

use crate::db_key::{raw_key_literal, DbKey};

const SQLITE_MAGIC: &[u8; 16] = b"SQLite format 3\0";

fn sql_string(path: &Path) -> String {
  format!("'{}'", path.to_string_lossy().replace('\'', "''"))
}

fn sidecars(db_path: &Path) -> [PathBuf; 2] {
  ["-wal", "-shm"].map(|suffix| {
    let mut sidecar = db_path.as_os_str().to_os_string();
    sidecar.push(suffix);
    PathBuf::from(sidecar)
  })
}

pub fn is_plaintext(db_path: &Path) -> bool {
  let Ok(mut file) = std::fs::File::open(db_path) else {
    return false;
  };

  let mut header = [0u8; SQLITE_MAGIC.len()];
  match file.read_exact(&mut header) {
    Ok(()) => &header == SQLITE_MAGIC,
    Err(_) => false,
  }
}

pub async fn migrate_if_plaintext(db_path: &Path, key: &DbKey) -> bool {
  if !is_plaintext(db_path) {
    return true;
  }

  info!("migrating the plaintext local store at {} to sqlcipher", db_path.display());

  let encrypted_path = db_path.with_extension("sqlcipher-new");
  let _ = std::fs::remove_file(&encrypted_path);

  if let Err(err) = export_encrypted(db_path, &encrypted_path, key).await {
    error!("local store unavailable: cannot encrypt {}: {err}", db_path.display());
    let _ = std::fs::remove_file(&encrypted_path);
    return false;
  }

  if let Err(err) = verify(&encrypted_path, key).await {
    error!("local store unavailable: the encrypted copy of {} did not verify: {err}", db_path.display());
    let _ = std::fs::remove_file(&encrypted_path);
    return false;
  }

  if let Err(err) = std::fs::rename(&encrypted_path, db_path) {
    error!("local store unavailable: cannot replace {}: {err}", db_path.display());
    let _ = std::fs::remove_file(&encrypted_path);
    return false;
  }

  for sidecar in sidecars(db_path) {
    let _ = std::fs::remove_file(sidecar);
  }

  info!("migrated the local store at {} to sqlcipher", db_path.display());

  true
}

async fn export_encrypted(db_path: &Path, encrypted_path: &Path, key: &DbKey) -> Result<(), sqlx::Error> {
  let mut conn = SqliteConnectOptions::new()
    .filename(db_path)
    .create_if_missing(true)
    .disable_statement_logging()
    .connect()
    .await?;

  let user_version: i64 = sqlx::query_scalar("PRAGMA user_version").fetch_one(&mut conn).await?;

  sqlx::raw_sql("PRAGMA wal_checkpoint(TRUNCATE)").fetch_all(&mut conn).await?;

  let attach = format!("ATTACH DATABASE {} AS encrypted KEY {}", sql_string(encrypted_path), raw_key_literal(key));
  sqlx::raw_sql(AssertSqlSafe(attach)).execute(&mut conn).await?;

  let exported = sqlx::raw_sql("SELECT sqlcipher_export('encrypted')").fetch_all(&mut conn).await;
  let versioned = match exported {
    Ok(_) => {
      sqlx::raw_sql(AssertSqlSafe(format!("PRAGMA encrypted.user_version = {user_version}")))
        .execute(&mut conn)
        .await
        .map(|_| ())
    }
    Err(err) => Err(err),
  };

  sqlx::raw_sql("DETACH DATABASE encrypted").execute(&mut conn).await?;
  versioned?;

  conn.close().await
}

async fn verify(encrypted_path: &Path, key: &DbKey) -> Result<(), sqlx::Error> {
  let mut conn = SqliteConnectOptions::new()
    .filename(encrypted_path)
    .create_if_missing(false)
    .disable_statement_logging()
    .pragma("key", raw_key_literal(key))
    .connect()
    .await?;

  let _: i64 = sqlx::query_scalar("SELECT count(*) FROM sqlite_master").fetch_one(&mut conn).await?;

  conn.close().await
}

