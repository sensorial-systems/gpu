use crate::prelude::*;

use wgpu::{Extent3d, Queue, TexelCopyBufferLayout, TexelCopyTextureInfo};

pub trait QueueExt {
    fn write_texture_ext<'a>(&self, texture: impl IntoExt<TexelCopyTextureInfo<'a>>, data: &[u8], data_layout: impl IntoExt<TexelCopyBufferLayout>, size: impl IntoExt<Extent3d>);
    fn write_texture_from_image<'a>(&self, texture: impl IntoExt<TexelCopyTextureInfo<'a>>, image: &impl ImageLayout);
}

impl QueueExt for Queue {
    fn write_texture_ext<'a>(&self, texture: impl IntoExt<TexelCopyTextureInfo<'a>>, data: &[u8], data_layout: impl IntoExt<TexelCopyBufferLayout>, size: impl IntoExt<Extent3d>) {
        self.write_texture(texture.into_ext(), data, data_layout.into_ext(), size.into_ext());
    }

    fn write_texture_from_image<'a>(&self, texture: impl IntoExt<TexelCopyTextureInfo<'a>>, image: &impl ImageLayout) {
        let dimensions = image.dimensions();
        self.write_texture(texture.into_ext(), image.data(), TexelCopyBufferLayout::new(Some(image.bytes_per_pixel() * dimensions.0), Some(dimensions.1)), dimensions.into_ext());
    }
}