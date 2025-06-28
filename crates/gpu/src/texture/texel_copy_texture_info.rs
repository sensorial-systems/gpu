use crate::prelude::*;

use wgpu::{Origin3d, TexelCopyTextureInfo, Texture, TextureAspect};

pub trait TexelCopyTextureInfoExt<'a> {
    fn new(texture: &'a Texture) -> Self;
    fn with_mip_level(self, mip_level: u32) -> Self;
    fn with_origin(self, origin: Origin3d) -> Self;
    fn with_aspect(self, aspect: TextureAspect) -> Self;
}

impl<'a> FromExt<&'a Texture> for TexelCopyTextureInfo<'a> {
    fn from_ext(texture: &'a Texture) -> Self {
        TexelCopyTextureInfo::new(texture)
    }
}

impl<'a> TexelCopyTextureInfoExt<'a> for TexelCopyTextureInfo<'a> {
    fn new(texture: &'a Texture) -> Self {
        TexelCopyTextureInfo {
            texture,
            mip_level: 0,
            origin: Origin3d::ZERO,
            aspect: TextureAspect::All,
        }
    }

    fn with_mip_level(mut self, mip_level: u32) -> Self {
        self.mip_level = mip_level;
        self
    }

    fn with_origin(mut self, origin: Origin3d) -> Self {
        self.origin = origin;
        self
    }

    fn with_aspect(mut self, aspect: TextureAspect) -> Self {
        self.aspect = aspect;
        self
    }
}