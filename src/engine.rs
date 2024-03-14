pub struct Engine {
  content: String
}

impl Engine {
  pub fn content(&self) -> String {
    self.content.clone()
  }
}
