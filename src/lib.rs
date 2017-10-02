extern crate gl;
extern crate glfw;
extern crate luminance_windowing;

use glfw::{Context, CursorMode, SwapInterval, Window, WindowMode};
pub use glfw::{Action, InitError, Key, MouseButton, WindowEvent};
pub use luminance_windowing::{Device, WindowDim, WindowOpt};
use std::os::raw::c_void;
use std::error::Error;
use std::fmt;
use std::sync::mpsc::Receiver;

/// Error that can be risen while creating a `Device` object.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GLFWDeviceError {
  InitError(InitError),
  WindowCreationFailed,
  NoPrimaryMonitor,
  NoVideoMode
}

impl fmt::Display for GLFWDeviceError {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    f.write_str(self.description())
  }
}

impl Error for GLFWDeviceError {
  fn description(&self) -> &str {
    match *self {
      GLFWDeviceError::InitError(_) => "initialization error",
      GLFWDeviceError::WindowCreationFailed => "failed to create window",
      GLFWDeviceError::NoPrimaryMonitor => "no primary monitor",
      GLFWDeviceError::NoVideoMode => "no video mode"
    }
  }

  fn cause(&self) -> Option<&Error> {
    match *self {
      GLFWDeviceError::InitError(ref e) => Some(e),
      _ => None
    }
  }
}

/// Device object.
///
/// Upon window and context creation, this type is used to add interaction and context handling.
pub struct GLFWDevice {
  /// Window.
  window: Window,
  /// Window events queue.
  events: Receiver<(f64, WindowEvent)>
}

impl Device for GLFWDevice {
  type Event = WindowEvent;
  type Error = GLFWDeviceError;

  fn new(dim: WindowDim, title: &str, win_opt: WindowOpt) -> Result<Self, Self::Error> {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).map_err(GLFWDeviceError::InitError)?;

    // OpenGL hints
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));

    // open a window in windowed or fullscreen mode
    let (mut window, events) = match dim {
      WindowDim::Windowed(w, h) => {
        glfw.create_window(w,
                           h,
                           title,
                           WindowMode::Windowed).ok_or(GLFWDeviceError::WindowCreationFailed)?
      },
      WindowDim::Fullscreen => {
        glfw.with_primary_monitor(|glfw, monitor| {
          let monitor = monitor.ok_or(GLFWDeviceError::NoPrimaryMonitor)?;
          let vmode = monitor.get_video_mode().ok_or(GLFWDeviceError::NoVideoMode)?;
          let (w, h) = (vmode.width, vmode.height);

          Ok(glfw.create_window(
              w,
              h,
              title,
              WindowMode::FullScreen(monitor)
              ).ok_or(GLFWDeviceError::WindowCreationFailed)?)
        })?
      },
      WindowDim::FullscreenRestricted(w, h) => {
        glfw.with_primary_monitor(|glfw, monitor| {
          let monitor = monitor.ok_or(GLFWDeviceError::NoPrimaryMonitor)?;

          Ok(glfw.create_window(
              w,
              h,
              title,
              WindowMode::FullScreen(monitor)
              ).ok_or(GLFWDeviceError::WindowCreationFailed)?)
        })?
      }
    };

    window.make_current();

    if win_opt.is_cursor_hidden() {
      window.set_cursor_mode(CursorMode::Disabled);
    }

    window.set_all_polling(true);
    glfw.set_swap_interval(SwapInterval::Sync(1));

    // init OpenGL
    gl::load_with(|s| window.get_proc_address(s) as *const c_void);

    Ok(GLFWDevice {
      window: window,
      events: events
    })
  }

  fn size(&self) -> [u32; 2] {
    let (w, h) = self.window.get_framebuffer_size();
    [w as u32, h as u32]
  }

  fn events<'a>(&'a mut self) -> Box<Iterator<Item = Self::Event> + 'a> {
    self.window.glfw.poll_events();
    Box::new(glfw::flush_messages(&self.events).map(|(_, e)| e))
  }

  fn draw<F>(&mut self, f: F) where F: FnOnce() {
    f();
    self.window.swap_buffers();
  }
}
