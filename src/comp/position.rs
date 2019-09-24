use specs::prelude::{VecStorage, Component};

#[derive(Debug)]
pub struct Position(pub f32);

impl Position {
  pub fn new(x: f32) -> Position {
    Position(x)
  }
}

impl Component for Position {
  type Storage = VecStorage<Self>;
}
