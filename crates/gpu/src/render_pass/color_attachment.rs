use wgpu::{Operations, RenderPassColorAttachment, TextureView, Color};

pub trait ColorAttachmentExt<'a>: Sized {
    fn new(view: &'a TextureView) -> Self;
    fn with_resolve_target(self, resolve_target: Option<&'a TextureView>) -> Self;
    fn with_ops(self, ops: Operations<Color>) -> Self;
}

impl<'a> ColorAttachmentExt<'a> for RenderPassColorAttachment<'a> {
    fn new(view: &'a TextureView) -> Self {
        RenderPassColorAttachment {
            view,
            resolve_target: Default::default(),
            ops: Default::default(),
        }
    }

    fn with_resolve_target(mut self, resolve_target: Option<&'a TextureView>) -> Self {
        self.resolve_target = resolve_target;
        self
    }

    fn with_ops(mut self, ops: Operations<Color>) -> Self {
        self.ops = ops;
        self
    }
}

