pub struct EventType;

impl EventType {
  pub const ADDED: i32 = 1 << 0;
  pub const REMOVED: i32 = 1 << 1;
  pub const UPDATED: i32 = 1 << 2;
  pub const EVICTED: i32 = 1 << 3;
  pub const EXPIRED: i32 = 1 << 4;
  pub const EVICT_ALL: i32 = 1 << 5;
  pub const CLEAR_ALL: i32 = 1 << 6;
  pub const MERGED: i32 = 1 << 7;
  pub const INVALIDATED: i32 = 1 << 8;
  pub const LOADED: i32 = 1 << 9;
  pub const ALL: i32 = Self::ADDED | Self::REMOVED | Self::UPDATED | Self::EVICTED | Self::EXPIRED | Self::EVICT_ALL | Self::CLEAR_ALL | Self::MERGED | Self::INVALIDATED | Self::LOADED;
}