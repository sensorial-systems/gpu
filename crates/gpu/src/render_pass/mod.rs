mod descriptor;
mod color_attachment;
mod operations;

pub use descriptor::*;
pub use color_attachment::*;
pub use operations::*;
use wgpu::{RenderPass, RenderPassDescriptor};

pub trait RenderPassExt<'a>: Sized {
    fn new(encoder: &'a mut wgpu::CommandEncoder, descriptor: &RenderPassDescriptor<'_>) -> Self;
}

impl<'a> RenderPassExt<'a> for RenderPass<'a> {
    fn new(encoder: &'a mut wgpu::CommandEncoder, descriptor: &RenderPassDescriptor<'_>) -> Self {
        encoder.begin_render_pass(descriptor)
    }
}
