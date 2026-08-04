use async_trait::async_trait;
use mongodb::bson::{doc, to_bson, Document};
use mongodb::options::UpdateModifications;
use mongodb::{Collection, Database};
use sync_dto::{EntityKind, FolderDto, NoteDto, TagDto};

use crate::domain::ports::{ReadModelRepository, RepositoryError};

fn backend_err(err: impl std::fmt::Display) -> RepositoryError {
  RepositoryError::Backend(err.to_string())
}

fn document_id(user_id: &str, entity_id: &str) -> String {
  format!("{user_id}:{entity_id}")
}

pub struct MongoReadModelRepository {
  notes: Collection<Document>,
  folders: Collection<Document>,
  tags: Collection<Document>,
}

impl MongoReadModelRepository {
  pub fn new(db: Database) -> Self {
    Self {
      notes: db.collection("notes"),
      folders: db.collection("folders"),
      tags: db.collection("tags"),
    }
  }

  fn collection_for(&self, entity: EntityKind) -> &Collection<Document> {
    match entity {
      EntityKind::Note => &self.notes,
      EntityKind::Folder => &self.folders,
      EntityKind::Tag => &self.tags,
    }
  }

  async fn upsert(
    collection: &Collection<Document>,
    user_id: &str,
    entity_id: &str,
    payload: &impl serde::Serialize,
    label: &str,
  ) -> Result<(), RepositoryError> {
    let mut document = to_bson(payload)
      .map_err(backend_err)?
      .as_document()
      .cloned()
      .ok_or_else(|| RepositoryError::Backend(format!("{label} did not serialize to a document")))?;

    let id = document_id(user_id, entity_id);
    document.insert("_id", id.clone());
    document.insert("user_id", user_id.to_string());

    collection
      .update_one(doc! { "_id": &id }, UpdateModifications::Document(doc! { "$set": document }))
      .upsert(true)
      .await
      .map_err(backend_err)?;

    Ok(())
  }

  async fn delete(collection: &Collection<Document>, user_id: &str, entity_id: &str) -> Result<(), RepositoryError> {
    collection
      .delete_one(doc! { "_id": document_id(user_id, entity_id) })
      .await
      .map_err(backend_err)?;

    Ok(())
  }
}

#[async_trait]
impl ReadModelRepository for MongoReadModelRepository {
  async fn existing_updated_at_ms(&self, user_id: &str, entity: EntityKind, entity_id: &str) -> Result<Option<i64>, RepositoryError> {
    let found = self
      .collection_for(entity)
      .find_one(doc! { "_id": document_id(user_id, entity_id) })
      .await
      .map_err(backend_err)?;

    Ok(found.and_then(|document| document.get_i64("updated_at_ms").ok()))
  }

  async fn upsert_note(&self, user_id: &str, note: &NoteDto) -> Result<(), RepositoryError> {
    Self::upsert(&self.notes, user_id, &note.id, note, "note").await
  }

  async fn upsert_folder(&self, user_id: &str, folder: &FolderDto) -> Result<(), RepositoryError> {
    Self::upsert(&self.folders, user_id, &folder.id, folder, "folder").await
  }

  async fn upsert_tag(&self, user_id: &str, tag: &TagDto) -> Result<(), RepositoryError> {
    Self::upsert(&self.tags, user_id, &tag.id, tag, "tag").await
  }

  async fn delete_note(&self, user_id: &str, id: &str) -> Result<(), RepositoryError> {
    Self::delete(&self.notes, user_id, id).await
  }

  async fn delete_folder(&self, user_id: &str, id: &str) -> Result<(), RepositoryError> {
    Self::delete(&self.folders, user_id, id).await
  }

  async fn delete_tag(&self, user_id: &str, id: &str) -> Result<(), RepositoryError> {
    Self::delete(&self.tags, user_id, id).await
  }
}
