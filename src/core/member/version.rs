use std::fmt::Display;

pub struct MemberVersion {
  pub major: u8,
  pub minor: u8,
  pub patch: u8,
}

impl MemberVersion {
  pub fn new(major: u8, minor: u8, patch: u8) -> Self {
    MemberVersion {
      major,
      minor,
      patch,
    }
  }

  pub fn equals(&self, other: MemberVersion, ignore_patch_version: bool) -> bool {
    if ignore_patch_version {
      self.major == other.major && self.minor == other.minor
    } else {
      self.major == other.major && self.minor == other.minor && self.patch == other.patch
    }
  }
}

impl Display for MemberVersion {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
  }
}