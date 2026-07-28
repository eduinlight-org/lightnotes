use async_trait::async_trait;
use mongodb::bson::{doc, to_bson, Document};
use mongodb::options::UpdateModifications;
use mongodb::{Collection, Database};
use sync_dto::{EntityKind, FolderDto, NoteDto, TagDto};

use crate::domain::ports::{ReadModelRepository, RepositoryError};

fn backend_err(err: impl std::fmt::Display) -> RepositoryError {
  RepositoryError::Backend(err.to_string())
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
}

#[async_trait]
impl ReadModelRepository for MongoReadModelRepository {
  async fn existing_updated_at_ms(&self, entity: EntityKind, entity_id: &str) -> Result<Option<i64>, RepositoryError> {
    let found = self
      .collection_for(entity)
      .find_one(doc! { "_id": entity_id })
      .await
      .map_err(backend_err)?;

    Ok(found.and_then(|document| document.get_i64("updated_at_ms").ok()))
  }

  async fn upsert_note(&self, note: &NoteDto) -> Result<(), RepositoryError> {
    let mut document = to_bson(note).map_err(backend_err)?.as_document().cloned().ok_or_else(|| RepositoryError::Backend("note did not serialize to a document".into()))?;
    document.insert("_id", note.id.clone());

    self
      .notes
      .update_one(doc! { "_id": &note.id }, UpdateModifications::Document(doc! { "$set": document }))
      .upsert(true)
      .await
      .map_err(backend_err)?;

    Ok(())
  }

  async fn upsert_folder(&self, folder: &FolderDto) -> Result<(), RepositoryError> {
    let mut document = to_bson(folder).map_err(backend_err)?.as_document().cloned().ok_or_else(|| RepositoryError::Backend("folder did not serialize to a document".into()))?;
    document.insert("_id", folder.id.clone());

    self
      .folders
      .update_one(doc! { "_id": &folder.id }, UpdateModifications::Document(doc! { "$set": document }))
      .upsert(true)
      .await
      .map_err(backend_err)?;

    Ok(())
  }

  async fn upsert_tag(&self, tag: &TagDto) -> Result<(), RepositoryError> {
    let mut document = to_bson(tag).map_err(backend_err)?.as_document().cloned().ok_or_else(|| RepositoryError::Backend("tag did not serialize to a document".into()))?;
    document.insert("_id", tag.id.clone());

    self
      .tags
      .update_one(doc! { "_id": &tag.id }, UpdateModifications::Document(doc! { "$set": document }))
      .upsert(true)
      .await
      .map_err(backend_err)?;

    Ok(())
  }

  async fn delete_note(&self, id: &str) -> Result<(), RepositoryError> {
    self.notes.delete_one(doc! { "_id": id }).await.map_err(backend_err)?;
    Ok(())
  }

  async fn delete_folder(&self, id: &str) -> Result<(), RepositoryError> {
    self.folders.delete_one(doc! { "_id": id }).await.map_err(backend_err)?;
    Ok(())
  }

  async fn delete_tag(&self, id: &str) -> Result<(), RepositoryError> {
    self.tags.delete_one(doc! { "_id": id }).await.map_err(backend_err)?;
    Ok(())
  }
}
