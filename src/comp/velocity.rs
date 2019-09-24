use specs::prelude::{VecStorage, Component};

#[derive(Debug)]
pub struct Velocity(pub f32);

impl Velocity {
  pub fn new(vel: f32) -> Velocity {
    Velocity(vel)
  }
}

impl Component for Velocity {
  type Storage = VecStorage<Self>;
}
