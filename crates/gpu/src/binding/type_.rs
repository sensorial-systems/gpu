use wgpu::{BindingType, BufferBindingType, BufferSize, SamplerBindingType, StorageTextureAccess, TextureFormat, TextureSampleType, TextureViewDimension};


pub trait BindingTypeExt<'a> {
    fn sampler(ty: SamplerBindingType) -> Self;
    fn texture(sample_type: TextureSampleType, dimension: TextureViewDimension, multisampled: bool) -> Self;
    fn buffer(ty: BufferBindingType, has_dynamic_offset: bool, min_binding_size: Option<BufferSize>) -> Self;
    fn storage_texture(access: StorageTextureAccess, format: TextureFormat, view_dimension: TextureViewDimension) -> Self;
    fn acceleration_structure(vertex_return: bool) -> Self;
}

impl<'a> BindingTypeExt<'a> for BindingType {
    fn sampler(ty: SamplerBindingType) -> Self {
        BindingType::Sampler(ty)
    }

    fn texture(sample_type: TextureSampleType, dimension: TextureViewDimension, multisampled: bool) -> Self {
        BindingType::Texture {
            sample_type,
            view_dimension: dimension,
            multisampled,
        }
    }

    fn buffer(ty: BufferBindingType, has_dynamic_offset: bool, min_binding_size: Option<BufferSize>) -> Self {
        BindingType::Buffer {
            ty,
            has_dynamic_offset,
            min_binding_size,
        }
    }

    fn storage_texture(access: StorageTextureAccess, format: TextureFormat, view_dimension: TextureViewDimension) -> Self {
        BindingType::StorageTexture {
            access,
            format,
            view_dimension,
        }
    }

    fn acceleration_structure(vertex_return: bool) -> Self {
        BindingType::AccelerationStructure {
            vertex_return,
        }
    }
}
