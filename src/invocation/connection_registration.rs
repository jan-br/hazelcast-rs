use uuid::Uuid;

pub struct ConnectionRegistration {
    server_registration_id: Uuid,
    correlation_id: u64
}

impl ConnectionRegistration {
    pub fn new(server_registration_id: Uuid, correlation_id: u64) -> Self {
        Self {
            server_registration_id,
            correlation_id
        }
    }
}