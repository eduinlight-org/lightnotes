use mongodb::{Client, Database};

pub async fn connect(uri: &str) -> Database {
  let client = Client::with_uri_str(uri).await.expect("failed to connect to MongoDB");
  client.default_database().expect("MONGODB_URI must include a default database name")
}
