use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MachineConfig {
  pub version: u8,
  pub api_key: String,
  pub working_dir: String,
}

impl ::std::default::Default for MachineConfig {
  fn default() -> Self {
    Self {
      version: 0,
      api_key: "".into(),
      working_dir: ".".into(),
    }
  }
}

pub fn load() -> MachineConfig {
  return confy::load("the-machine").unwrap();
}
