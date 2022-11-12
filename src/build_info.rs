pub struct BuildInfo;

impl BuildInfo {
  pub const UNKNOWN_VERSION_ID: i32 = -1;
  pub const MAJOR_VERSION_MULTIPLIER: i32 = 10000;
  pub const MINOR_VERSION_MULTIPLIER: i32 = 100;

  pub fn calculate_server_version_from_string(version_string: Option<String>) -> i32 {
    match version_string {
      None => {
        Self::UNKNOWN_VERSION_ID
      }
      Some(version_string) => {
        let main_parts = version_string.split("_").collect::<Vec<&str>>();
        let tokens = main_parts[0].split(".").collect::<Vec<&str>>();
        if tokens.len() < 2 {
          Self::UNKNOWN_VERSION_ID
        } else {
          let major = tokens[0].parse::<i32>().unwrap();
          let minor = tokens[1].parse::<i32>().unwrap();
          let patch = if tokens.len() == 2 {
            0
          } else {
            tokens[2].parse::<i32>().unwrap()
          };
          Self::calculate_server_version(major, minor, patch)
        }
      }
    }
  }

  pub fn calculate_server_version(major: i32, minor: i32, patch: i32) -> i32 {
    Self::MAJOR_VERSION_MULTIPLIER * major + Self::MINOR_VERSION_MULTIPLIER * minor + patch
  }
}