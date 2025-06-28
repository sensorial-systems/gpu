use crate::prelude::*;

mod descriptor;
mod sampler;
mod view;
mod texel_copy_texture_info;
mod texel_copy_buffer_layout;
mod sample_type;

pub use descriptor::*;
pub use view::*;
pub use sampler::*;
pub use texel_copy_texture_info::*;
pub use texel_copy_buffer_layout::*;
pub use sample_type::*;

use wgpu::{Device, Extent3d, Queue, TexelCopyBufferLayout, TexelCopyTextureInfo, Texture, TextureDescriptor};

pub trait TextureExt {
    fn new(device: &Device, descriptor: &TextureDescriptor) -> Self;
    fn write<'a>(&self, queue: &Queue, texture: impl IntoExt<TexelCopyTextureInfo<'a>>, data: &[u8], data_layout: TexelCopyBufferLayout, size: impl IntoExt<Extent3d>);
}

impl TextureExt for Texture {
    fn new(device: &Device, descriptor: &TextureDescriptor) -> Self {
        device.create_texture(&descriptor)
    }

    fn write<'a>(&self, queue: &Queue, texture: impl IntoExt<TexelCopyTextureInfo<'a>>, data: &[u8], data_layout: TexelCopyBufferLayout, size: impl IntoExt<Extent3d>) {
        queue.write_texture(
            texture.into_ext(),
            data,
            data_layout,
            size.into_ext()
        );
    }
}