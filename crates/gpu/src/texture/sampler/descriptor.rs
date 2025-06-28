use std::ops::Range;

use wgpu::{AddressMode, CompareFunction, FilterMode, SamplerBorderColor, SamplerDescriptor};

pub trait SamplerDescriptorExt<'a> {
    fn new() -> Self;

    fn with_address_mode(self, address_mode: (AddressMode, AddressMode, AddressMode)) -> Self;
    fn with_address_mode_u(self, address_mode_u: AddressMode) -> Self;
    fn with_address_mode_v(self, address_mode_v: AddressMode) -> Self;
    fn with_address_mode_w(self, address_mode_w: AddressMode) -> Self;

    fn with_filter(self, filter: (FilterMode, FilterMode, FilterMode)) -> Self;
    fn with_mag_filter(self, mag_filter: FilterMode) -> Self;
    fn with_min_filter(self, min_filter: FilterMode) -> Self;
    fn with_mipmap_filter(self, mipmap_filter: FilterMode) -> Self;

    fn with_lod_clamp(self, lod_clamp: Range<f32>) -> Self;
    fn with_lod_min_clamp(self, lod_min_clamp: f32) -> Self;
    fn with_lod_max_clamp(self, lod_max_clamp: f32) -> Self;

    fn with_compare(self, compare: Option<CompareFunction>) -> Self;

    fn with_anisotropy_clamp(self, anisotropy_clamp: u16) -> Self;

    fn with_border_color(self, border_color: Option<SamplerBorderColor>) -> Self;
}

impl<'a> SamplerDescriptorExt<'a> for SamplerDescriptor<'a> {
    fn new() -> Self {
        Self {
            address_mode_u: AddressMode::Repeat,
            address_mode_v: AddressMode::Repeat,
            address_mode_w: AddressMode::Repeat,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Linear,
            ..Default::default()
        }
    }

    fn with_address_mode(self, address_mode: (AddressMode, AddressMode, AddressMode)) -> Self {
        self.with_address_mode_u(address_mode.0)
            .with_address_mode_v(address_mode.1)
            .with_address_mode_w(address_mode.2)
    }

    fn with_address_mode_u(mut self, address_mode_u: AddressMode) -> Self {
        self.address_mode_u = address_mode_u;
        self
    }

    fn with_address_mode_v(mut self, address_mode_v: AddressMode) -> Self {
        self.address_mode_v = address_mode_v;
        self
    }

    fn with_address_mode_w(mut self, address_mode_w: AddressMode) -> Self {
        self.address_mode_w = address_mode_w;
        self
    }

    fn with_filter(self, filter: (FilterMode, FilterMode, FilterMode)) -> Self {
        self.with_mag_filter(filter.0)
            .with_min_filter(filter.1)
            .with_mipmap_filter(filter.2)
    }

    fn with_mag_filter(mut self, mag_filter: FilterMode) -> Self {
        self.mag_filter = mag_filter;
        self
    }

    fn with_min_filter(mut self, min_filter: FilterMode) -> Self {
        self.min_filter = min_filter;
        self
    }

    fn with_mipmap_filter(mut self, mipmap_filter: FilterMode) -> Self {
        self.mipmap_filter = mipmap_filter;
        self
    }

    fn with_lod_clamp(self, lod_clamp: Range<f32>) -> Self {
        self.with_lod_min_clamp(lod_clamp.start)
            .with_lod_max_clamp(lod_clamp.end)
    }

    fn with_lod_min_clamp(mut self, lod_min_clamp: f32) -> Self {
        self.lod_min_clamp = lod_min_clamp;
        self
    }

    fn with_lod_max_clamp(mut self, lod_max_clamp: f32) -> Self {
        self.lod_max_clamp = lod_max_clamp;
        self
    }

    fn with_compare(mut self, compare: Option<CompareFunction>) -> Self {
        self.compare = compare;
        self
    }

    fn with_anisotropy_clamp(mut self, anisotropy_clamp: u16) -> Self {
        self.anisotropy_clamp = anisotropy_clamp;
        self
    }

    fn with_border_color(mut self, border_color: Option<SamplerBorderColor>) -> Self {
        self.border_color = border_color;
        self
    }
}