use std::num::NonZeroU32;

use crate::MultisampleStateExt;

use wgpu::{PipelineCache, DepthStencilState, FragmentState, MultisampleState, PipelineLayout, PrimitiveState, RenderPipelineDescriptor, VertexState};

pub trait RenderPipelineDescriptorExt<'label> {
    fn new(vertex_state: VertexState<'label>, primitive: PrimitiveState) -> Self;
    fn with_layout(self, layout: &'label PipelineLayout) -> Self;
    fn with_fragment(self, fragment: Option<FragmentState<'label>>) -> Self;
    fn with_depth_stencil(self, depth_stencil: Option<DepthStencilState>) -> Self;
    fn with_multisample(self, multisample: MultisampleState) -> Self;
    fn with_multiview(self, multiview: Option<NonZeroU32>) -> Self;
    fn with_cache(self, cache: Option<&'label PipelineCache>) -> Self;
}

impl<'label> RenderPipelineDescriptorExt<'label> for RenderPipelineDescriptor<'label> {
    fn new(vertex_state: VertexState<'label>, primitive: PrimitiveState) -> RenderPipelineDescriptor<'label> {
        RenderPipelineDescriptor {
            label: Default::default(),
            layout: Default::default(),
            vertex: vertex_state,
            fragment: Default::default(),
            primitive,
            depth_stencil: Default::default(),
            multisample: MultisampleState::new(),
            multiview: Default::default(),
            cache: Default::default(),
        }
    }

    fn with_layout(mut self, layout: &'label PipelineLayout) -> Self {
        self.layout = Some(layout);
        self
    }

    fn with_fragment(mut self, fragment: Option<FragmentState<'label>>) -> Self {
        self.fragment = fragment;
        self
    }

    fn with_depth_stencil(mut self, depth_stencil: Option<DepthStencilState>) -> Self {
        self.depth_stencil = depth_stencil;
        self
    }

    fn with_multisample(mut self, multisample: MultisampleState) -> Self {
        self.multisample = multisample;
        self
    }

    fn with_multiview(mut self, multiview: Option<NonZeroU32>) -> Self {
        self.multiview = multiview;
        self
    }

    fn with_cache(mut self, cache: Option<&'label PipelineCache>) -> Self {
        self.cache = cache;
        self
    }
}

crate::impl_label!(RenderPipelineDescriptor<'label>);