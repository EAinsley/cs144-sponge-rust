use std::rc::Rc;
#[derive(Clone)]
pub struct Buffer {
  storage: Rc<String>,
  starting_offset: usize,
}
impl Buffer {
  pub fn new() -> Buffer {
    Buffer {
      storage: Rc::new(String::new()),
      starting_offset: 0,
    }
  }
  pub fn from_string(s: String) -> Buffer {
    Buffer {
      storage: Rc::new(s),
      starting_offset: 0,
    }
  }
  pub fn size(&self) -> usize {
    todo!()
  }
  pub fn at(&self, _n: usize) -> u8 {
    todo!();
  }
  pub fn remove_prefix(&self, _n: usize) {
    todo!();
  }
  pub fn copy(&self) -> String {
    (*self.storage).clone()
  }
  pub fn str(&self) -> &String {
    // &self.storage.str
    todo!()
  }
}
