extern crate glfw;
extern crate gl;

use self::glfw::{Action, Context, Key};
use std::sync::mpsc::Receiver;
use specs::prelude::*;
use crate::comp::*;
use crate::sys::*;

// Window Settings
const SCR_WIDTH: u32 = 1280;
const SCR_HEIGHT: u32 = 720;

pub fn game() {
  let mut world = World::new();
  world.register::<Velocity>();
  world.register::<Position>();
  world.register::<Renderable>();

  world.create_entity().with(Velocity::new(1.0)).with(Position::new(1.0)).build();
  world.create_entity().with(Velocity::new(2.0)).with(Position::new(2.0)).build();
  world.create_entity().with(Velocity::new(3.0)).with(Position::new(3.0)).build();

  world.create_entity().with(Position::new(4.0)).build();

  world.create_entity().with(Renderable::new()).build();

  let mut pre_sys_dispatcher = DispatcherBuilder::new().with(PreRender, "sys_pre_render", &[]).build();
  let mut loop_sys_dispatcher = DispatcherBuilder::new().with(Render, "sys_render", &[]).build();

  let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
  glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
  glfw.window_hint(glfw::WindowHint::OpenGlProfile(
    glfw::OpenGlProfileHint::Core,
  ));
  #[cfg(target_os = "macos")]
  glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

  let (mut window, events) = glfw
    .create_window(
      SCR_WIDTH,
      SCR_HEIGHT,
      "Survivor Economic City Builder - Alpha v0.0.1",
      glfw::WindowMode::Windowed,
    )
    .expect("Failed to create GLFW window");

  window.make_current();
  window.set_key_polling(true);
  window.set_framebuffer_size_polling(true);

  gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

  pre_sys_dispatcher.setup(&mut world);
  pre_sys_dispatcher.dispatch(&mut world);

  while !window.should_close() {
    // events
    // -----
    process_events(&mut window, &events);
    loop_sys_dispatcher.setup(&mut world);
    loop_sys_dispatcher.dispatch(&mut world);

    // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
    // -------------------------------------------------------------------------------
    unsafe {
      gl::ClearColor(1.0, 1.0, 1.0, 1.0);
      gl::Clear(gl::COLOR_BUFFER_BIT);
    }

    window.swap_buffers();
    glfw.poll_events();
  }
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
  for (_, event) in glfw::flush_messages(events) {
    match event {
      glfw::WindowEvent::FramebufferSize(width, height) => {
        // make sure the viewport matches the new window dimensions; note that width and
        // height will be significantly larger than specified on retina displays.
        unsafe { gl::Viewport(0, 0, width, height) }
      }
      glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
      _ => {}
    }
  }
}
