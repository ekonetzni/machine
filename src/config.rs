use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MachineConfig {
  pub version: u8,
  pub api_key: String,
  pub daemon: bool,
}

impl ::std::default::Default for MachineConfig {
  fn default() -> Self {
    Self {
      version: 0,
      api_key: "".into(),
      daemon: true,
    }
  }
}

pub fn load() -> MachineConfig {
  return confy::load("the-machine").unwrap();
}
