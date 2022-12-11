use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::codec::client_fetch_schema_codec::ClientFetchSchemaCodec;
use crate::connection::registry::ConnectionRegistry;
use crate::invocation::Invocation;
use crate::invocation::service::InvocationService;
use crate::serialization::schema::Schema;

pub struct SchemaService {
  schemas: RwLock<HashMap<i64, Arc<Schema>>>,
  connection_registry: Arc<ConnectionRegistry>,
}

impl SchemaService {
  pub fn new(connection_registry: Arc<ConnectionRegistry>) -> Self {
    SchemaService {
      schemas: RwLock::new(HashMap::new()),
      connection_registry,
    }
  }

  pub async fn fetch_schemas(&self, schema_id: i64) {
    let invocation_service = self.get_invocation_service();
    let mut invocation = Invocation::new(invocation_service.clone(), ClientFetchSchemaCodec::encode_request(&schema_id).await);
    invocation.handler = Some(Box::pin(|mut client_message| Box::pin(async move {
      Box::new(Box::new(ClientFetchSchemaCodec::decode_response(&mut client_message).await))
    })));
    let schema = invocation_service.invoke(&self.connection_registry, invocation).await;
    if let Some(schema) = schema {
      self.put_if_absent(&schema);
      //todo: Add logging
    } else {
      todo!()
    }
  }

  pub async fn put_if_absent(&self, schema: &Schema) {
    let schema_id = schema.schema_id;
    let mut schemas = self.schemas.write().await;
    match schemas.get(&schema_id) {
      None => {
        //todo: Add log
        schemas.insert(schema_id, Arc::new(schema.clone()));
      }
      Some(schema) => {
        todo!("Schema already exists: {}", schema.type_name)
      }
    }
  }

  pub fn get(&self, schema_id: i64) -> Option<Arc<Schema>> {
    todo!()
  }

  pub fn get_invocation_service(&self) -> Arc<InvocationService> {
    todo!()
  }
}