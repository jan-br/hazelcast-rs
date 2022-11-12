pub struct DistributedObjectInfo {
  pub service_name: String,
  pub name: String,
}

impl DistributedObjectInfo {
  pub fn new(service_name: String, name: String) -> DistributedObjectInfo {
    DistributedObjectInfo {
      service_name,
      name,
    }
  }
}