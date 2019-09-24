use specs::prelude::*;
pub use crate::comp::*;

pub struct Render;

impl<'a> System<'a> for Render {
  type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Velocity>);

  fn run(&mut self, (mut pos, vel): Self::SystemData) {
    for (pos, vel) in (&mut pos, &vel).join() {
      // pos.0 += vel.0;
      println!("Render System {}", pos.0);
    }
  }
}
