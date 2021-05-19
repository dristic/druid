
use crate::piet::{Piet, RenderContext};
use druid_shell::WindowHandle;

use crate::widget_host::WidgetState;

pub struct EventCtx<'a> {
    pub(crate) window: &'a WindowHandle,
    pub(crate) state: &'a mut WidgetState,
}

pub struct PaintCtx<'a, 'b> {
    state: &'a WidgetState,
    render_ctx: &'a mut Piet<'b>,
}

pub struct LayoutCtx<'a> {
    state: &'a WidgetState,
}

impl<'a> EventCtx<'a> {
    pub fn hovered(&self) -> bool {
        self.state.hovered
    }

    pub fn mouse_focused(&self) -> bool {
        self.state.mouse_focus
    }

    pub fn keyboard_focused(&self) -> bool {
        self.state.keyboard_focus
    }
}

impl<'c> std::ops::Deref for PaintCtx<'_, 'c> {
    type Target = Piet<'c>;

    fn deref(&self) -> &Self::Target {
        self.render_ctx
    }
}

impl<'c> std::ops::DerefMut for PaintCtx<'_, 'c> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.render_ctx
    }
}

impl PaintCtx<'_, '_> {
    pub fn hovered(&self) -> bool {
        self.state.hovered
    }

    pub fn mouse_focused(&self) -> bool {
        self.state.mouse_focus
    }

    pub fn keyboard_focused(&self) -> bool {
        self.state.keyboard_focus
    }

    pub fn with_save(&mut self, f: impl FnOnce(&mut PaintCtx)) {
        if let Err(e) = self.render_ctx.save() {
            eprintln!("Failed to save RenderContext: '{}'", e);
            return;
        }

        f(self);

        if let Err(e) = self.render_ctx.restore() {
            eprintln!("Failed to restore RenderContext: '{}'", e);
        }
    }
}