use wgpu::TextureSampleType;

pub trait TextureSampleTypeExt {
    fn float(filterable: bool) -> Self;
    fn depth() -> Self;
    fn sint() -> Self;
    fn uint() -> Self;
}

impl TextureSampleTypeExt for TextureSampleType {
    fn float(filterable: bool) -> Self {
        TextureSampleType::Float { filterable }
    }

    fn depth() -> Self {
        TextureSampleType::Depth
    }

    fn sint() -> Self {
        TextureSampleType::Sint
    }

    fn uint() -> Self {
        TextureSampleType::Uint
    }
}