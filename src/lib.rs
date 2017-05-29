#![feature(conservative_impl_trait)]

extern crate gl;
extern crate glfw;
extern crate luminance;

use glfw::{Context, CursorMode, SwapInterval, Window, WindowMode};
pub use glfw::{Action, InitError, Key, MouseButton, WindowEvent};
use std::os::raw::c_void;
use std::sync::mpsc::Receiver;

/// Error that can be risen while creating a `Device` object.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeviceError {
  InitError(InitError),
  WindowCreationFailed,
  NoPrimaryMonitor,
  NoVideoMode
}

/// Dimension of the window to create.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WindowDim {
  Windowed(u32, u32),
  Fullscreen,
  FullscreenRestricted(u32, u32)
}

/// Device object.
///
/// Upon window and context creation, this type is used to add interaction and context handling.
pub struct Device {
  /// Window.
  window: Window,
  /// Window events queue.
  events: Receiver<(f64, WindowEvent)>
}

impl Device {
  /// Give the size of the framebuffer attached to the window.
  pub fn size(&self) -> [u32; 2] {
    let (w, h) = self.window.get_framebuffer_size();
    [w as u32, h as u32]
  }

  /// Retrieve an iterator over any pulled events.
  pub fn events<'a>(&'a mut self) -> impl Iterator<Item = (f64, WindowEvent)> + 'a {
    self.window.glfw.poll_events();
    glfw::flush_messages(&self.events)
  }

  /// Perform a draw. You should recall that function each time you want to draw a single frame to
  /// the screen.
  pub fn draw<F>(&mut self, f: F) where F: FnOnce() {
    f();
    self.window.swap_buffers();
  }

  /// Create a device and bootstrap a luminance environment that lives as long as thdevice
  /// lives.
  pub fn new(dim: WindowDim, title: &str, win_opt: WindowOpt) -> Result<Self, DeviceError> {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).map_err(DeviceError::InitError)?;

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
                           WindowMode::Windowed).ok_or(DeviceError::WindowCreationFailed)?
      },
      WindowDim::Fullscreen => {
        glfw.with_primary_monitor(|glfw, monitor| {
          let monitor = monitor.ok_or(DeviceError::NoPrimaryMonitor)?;
          let vmode = monitor.get_video_mode().ok_or(DeviceError::NoVideoMode)?;
          let (w, h) = (vmode.width, vmode.height);

          Ok(glfw.create_window(w,
                                h,
                                title,
                                WindowMode::FullScreen(monitor)).ok_or(DeviceError::WindowCreationFailed)?)
        })?
      },
      WindowDim::FullscreenRestricted(w, h) => {
        glfw.with_primary_monitor(|glfw, monitor| {
          let monitor = monitor.ok_or(DeviceError::NoPrimaryMonitor)?;

          Ok(glfw.create_window(w,
                                h,
                                title,
                                WindowMode::FullScreen(monitor)).ok_or(DeviceError::WindowCreationFailed)?)
        })?
      }
    };

    window.make_current();

    if win_opt.hide_cursor {
      window.set_cursor_mode(CursorMode::Disabled);
    }

    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_mouse_button_polling(true);
    window.set_scroll_polling(true);
    window.set_framebuffer_size_polling(true);
    glfw.set_swap_interval(SwapInterval::Sync(1));

    // init OpenGL
    gl::load_with(|s| window.get_proc_address(s) as *const c_void);

    Ok(Device {
      window: window,
      events: events
    })
  }
}

/// Different window options.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WindowOpt {
  hide_cursor: bool
}

impl Default for WindowOpt {
  fn default() -> Self {
    WindowOpt {
      hide_cursor: false
    }
  }
}

impl WindowOpt {
  /// Hide or unhide the cursor. Default to `false`.
  #[inline]
  pub fn hide_cursor(self, hide: bool) -> Self {
    WindowOpt { hide_cursor: hide, ..self }
  }

  #[inline]
  pub fn is_cursor_hidden(&self) -> bool {
    self.hide_cursor
  }
}
