extern crate daemonize;

trait Workflow {
  fn execute(&self) -> &str;
}

struct DownloadVideo;
impl Workflow for DownloadVideo {
  fn execute(&self) -> &str {
    println!("Downloading video...");
    return "Downloading video";
  }
}

struct Machine {
  workflows: Vec<Box<dyn Workflow>>,
}
impl Machine {
  fn new() -> Self {
    Self { workflows: vec![] }
  }

  fn add_workflow(&mut self, workflow: Box<dyn Workflow>) {
    self.workflows.push(workflow);
  }

  fn exe(&self) -> Vec<&str> {
    return self.workflows.iter().map(|wf| wf.execute()).collect();
  }
}
