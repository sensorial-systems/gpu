use wgpu::{PrimitiveTopology, FrontFace, Face, PolygonMode, PrimitiveState};

pub trait PrimitiveStateExt {
    fn new(topology: PrimitiveTopology) -> Self;
}

impl PrimitiveStateExt for PrimitiveState {
    fn new(topology: PrimitiveTopology) -> Self {
        Self {
            topology,
            strip_index_format: None,
            front_face: FrontFace::Ccw,
            cull_mode: Some(Face::Back),
            polygon_mode: PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        }
    }
}