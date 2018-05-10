use gl;
use glfw::{Context, RenderContext};

pub struct Context {
  ctx: RenderContext,
  gfx_state: Rc<RefCell<GraphicsState>>,
}

impl GraphicsContext {
  pub(crate) fn new(window: &mut Window) -> Self {
    let ctx = window.render_context();

    ctx.make_current()

    gl::load_with(|s| window.get_proc_address(s) as *const c_void);
    GraphicsContext { ctx }
  }
//
//  pub fn draw<F>(&mut self, f: F) where F: FnOnce() {
//    self.ctx.make_current();
//    f();
//    self.ctx.swap_buffers();
//  }
}
