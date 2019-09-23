extern crate glfw;
use self::glfw::{Action, Context, Key};

extern crate gl;

use std::sync::mpsc::Receiver;

// Window Settings
const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub fn game() {
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

  while !window.should_close() {
    // events
    // -----
    process_events(&mut window, &events);

    // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
    // -------------------------------------------------------------------------------
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
