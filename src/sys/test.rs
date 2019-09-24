use specs::prelude::*;
pub use crate::comp::*;

pub struct Test;

impl<'a> System<'a> for Test {
  type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Velocity>);

  fn run(&mut self, (mut pos, vel): Self::SystemData) {
    for (pos, vel) in (&mut pos, &vel).join() {
      pos.0 += vel.0;
      println!("Here {}", pos.0);
    }
  }
}
