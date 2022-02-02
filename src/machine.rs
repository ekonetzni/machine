pub struct Machine {
  pub generation: String,
}
impl Machine {
  pub fn new() -> Self {
    Self {
      generation: "gen 1".into(),
    }
  }
  pub fn download(mut self) -> Self {
    println!("downloading");
    return self;
  }

  pub fn read(mut self) -> Self {
    println!("reading");
    return self;
  }
}
