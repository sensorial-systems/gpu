use wgpu::{TextureAspect, TextureFormat, TextureUsages, TextureViewDimension, TextureViewDescriptor};

pub trait TextureViewDescriptorExt<'a> {
    fn new() -> Self;

    fn with_format(self, format: Option<TextureFormat>) -> Self;

    fn with_dimension(self, dimension: Option<TextureViewDimension>) -> Self;

    fn with_usage(self, usage: Option<TextureUsages>) -> Self;

    fn with_aspect(self, aspect: TextureAspect) -> Self;

    fn with_base_mip_level(self, base_mip_level: u32) -> Self;

    fn with_mip_level_count(self, mip_level_count: Option<u32>) -> Self;

    fn with_base_array_layer(self, base_array_layer: u32) -> Self;

    fn with_array_layer_count(self, array_layer_count: Option<u32>) -> Self;
}

impl<'a> TextureViewDescriptorExt<'a> for TextureViewDescriptor<'a> {
    fn new() -> Self {
        Default::default()
    }

    fn with_format(mut self, format: Option<TextureFormat>) -> Self {
        self.format = format;
        self
    }

    fn with_dimension(mut self, dimension: Option<TextureViewDimension>) -> Self {
        self.dimension = dimension;
        self
    }

    fn with_usage(mut self, usage: Option<TextureUsages>) -> Self {
        self.usage = usage;
        self
    }

    fn with_aspect(mut self, aspect: TextureAspect) -> Self {
        self.aspect = aspect;
        self
    }

    fn with_base_mip_level(mut self, base_mip_level: u32) -> Self {
        self.base_mip_level = base_mip_level;
        self
    }

    fn with_mip_level_count(mut self, mip_level_count: Option<u32>) -> Self {
        self.mip_level_count = mip_level_count;
        self
    }

    fn with_base_array_layer(mut self, base_array_layer: u32) -> Self {
        self.base_array_layer = base_array_layer;
        self
    }

    fn with_array_layer_count(mut self, array_layer_count: Option<u32>) -> Self {
        self.array_layer_count = array_layer_count;
        self
    }
}

crate::impl_label!(TextureViewDescriptor<'a>);