#[repr(u8)]
pub enum AuthenticationStatus {
  Authenticated = 0,
  CredentialsFailed = 1,
  SerializationVersionMismatch = 2,
  NotAllowedInCluster = 3,
}