pub use glfw::InitError;
pub use luminance::state::StateQueryError;

use std::error::Error;
use std::fmt;

/// Error that can be risen while creating a surface.
#[derive(Debug)]
pub enum GlfwSurfaceError {
  InitError(InitError),
  WindowCreationFailed,
  NoPrimaryMonitor,
  NoVideoMode,
  GraphicsStateError(StateQueryError),
}

impl fmt::Display for GlfwSurfaceError {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    f.write_str(self.description())
  }
}

impl Error for GlfwSurfaceError {
  fn description(&self) -> &str {
    match *self {
      GlfwSurfaceError::InitError(_) => "initialization error",
      GlfwSurfaceError::WindowCreationFailed => "failed to create window",
      GlfwSurfaceError::NoPrimaryMonitor => "no primary monitor",
      GlfwSurfaceError::NoVideoMode => "no video mode",
      GlfwSurfaceError::GraphicsStateError(_) => "failed to get graphics state",
    }
  }

  fn cause(&self) -> Option<&Error> {
    match *self {
      GlfwSurfaceError::InitError(ref e) => Some(e),
      GlfwSurfaceError::GraphicsStateError(ref e) => Some(e),
      _ => None
    }
  }
}

