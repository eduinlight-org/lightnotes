use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityKind {
  Note,
  Folder,
  Tag,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeOp {
  Create,
  Update,
  Delete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FolderIconDto {
  Inbox,
  Briefcase,
  User,
  BookOpen,
  Notebook,
  Archive,
  House,
  Star,
  Heart,
  Settings,
  Calendar,
  Camera,
  Music,
  Code,
  Palette,
  Gift,
  Globe,
  Lock,
  Rocket,
  Bookmark,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NoteDto {
  pub id: String,
  pub title: String,
  pub content: String,
  pub folder_id: Option<String>,
  pub tag_ids: Vec<String>,
  pub pinned: bool,
  pub starred: bool,
  pub updated_at_ms: i64,
  pub order: i64,
  #[serde(default)]
  pub date_ms: i64,
  #[serde(default)]
  pub remind_before_hours: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FolderDto {
  pub id: String,
  pub name: String,
  pub icon: FolderIconDto,
  pub updated_at_ms: i64,
  pub order: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TagDto {
  pub id: String,
  pub name: String,
  pub updated_at_ms: i64,
  pub order: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChangePayload {
  Note(NoteDto),
  Folder(FolderDto),
  Tag(TagDto),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueuedChange {
  pub change_id: String,
  pub device_id: String,
  pub entity: EntityKind,
  pub entity_id: String,
  pub op: ChangeOp,
  pub payload: Option<ChangePayload>,
  pub client_updated_at_ms: i64,
  pub enqueued_at_ms: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PushChangesRequest {
  pub changes: Vec<QueuedChange>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AcceptedChange {
  pub change_id: String,
  pub seq: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RejectedChange {
  pub change_id: String,
  pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PushChangesResponse {
  pub accepted: Vec<AcceptedChange>,
  pub rejected: Vec<RejectedChange>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerChange {
  pub seq: i64,
  pub change_id: String,
  pub device_id: String,
  pub entity: EntityKind,
  pub entity_id: String,
  pub op: ChangeOp,
  pub payload: Option<ChangePayload>,
  pub server_applied_at_ms: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PullChangesResponse {
  pub changes: Vec<ServerChange>,
  pub cursor: i64,
}

pub fn is_newer_or_equal(existing_updated_at_ms: Option<i64>, incoming_updated_at_ms: i64) -> bool {
  existing_updated_at_ms.is_none_or(|existing| incoming_updated_at_ms >= existing)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserDto {
  pub id: String,
  pub email: String,
  pub name: Option<String>,
  pub picture: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthTokensDto {
  pub access_token: String,
  pub refresh_token: String,
  pub expires_in_secs: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthConfigResponse {
  pub google_client_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GoogleSignInRequest {
  pub id_token: String,
  pub device_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthSessionResponse {
  pub user: UserDto,
  pub tokens: AuthTokensDto,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RefreshRequest {
  pub refresh_token: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RefreshResponse {
  pub tokens: AuthTokensDto,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignOutRequest {
  pub refresh_token: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrentUserResponse {
  pub user: UserDto,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NativeAuthStartResponse {
  pub ticket: String,
  pub authorize_url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NativeAuthPollRequest {
  pub ticket: String,
}
