// TODO: Probably move it to render_target.rs

pub use wgpu::{TextureFormat, BlendState, ColorWrites, ColorTargetState};

pub trait ColorTargetStateExt {
    fn new(format: TextureFormat) -> Self;
    fn with_blend(self, blend: BlendState) -> Self;
    fn with_write_mask(self, write_mask: ColorWrites) -> Self;
}

impl ColorTargetStateExt for ColorTargetState {
    fn new(format: TextureFormat) -> Self {
        Self {
            format,
            blend: Some(BlendState::REPLACE),
            write_mask: ColorWrites::ALL,
        }
    }

    fn with_blend(mut self, blend: BlendState) -> Self {
        self.blend = Some(blend);
        self
    }

    fn with_write_mask(mut self, write_mask: ColorWrites) -> Self {
        self.write_mask = write_mask;
        self
    }
}