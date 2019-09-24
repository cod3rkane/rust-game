extern crate gl;
use specs::prelude::{VecStorage, Component};

#[derive(Debug)]
pub struct Renderable {
  pub shaderProgram: gl::types::GLuint,
  pub VAO: gl::types::GLuint,
}

impl Renderable {
  pub fn new() -> Renderable {
    Renderable {
      shaderProgram: 0,
      VAO: 0,
    }
  }
}

impl Component for Renderable {
  type Storage = VecStorage<Self>;
}
