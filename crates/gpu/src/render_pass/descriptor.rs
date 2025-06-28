use wgpu::{QuerySet, RenderPassColorAttachment, RenderPassDescriptor, RenderPassDepthStencilAttachment, RenderPassTimestampWrites};

pub trait RenderPassDescriptorExt<'label>: Sized {
    fn new() -> Self;
    fn with_color_attachments(self, attachments: &'label [Option<RenderPassColorAttachment<'label>>]) -> Self;
    fn with_depth_stencil_attachment(self, attachment: Option<RenderPassDepthStencilAttachment<'label>>) -> Self;
    fn with_timestamp_writes(self, timestamp_writes: Option<RenderPassTimestampWrites<'label>>) -> Self;
    fn with_occlusion_query_set(self, occlusion_query_set: Option<&'label QuerySet>) -> Self;
}

impl<'label> RenderPassDescriptorExt<'label> for RenderPassDescriptor<'label> {
    fn new() -> Self {
        Default::default()
    }

    fn with_color_attachments(mut self, attachments: &'label [Option<RenderPassColorAttachment<'label>>]) -> Self {
        self.color_attachments = attachments;
        self
    }

    fn with_depth_stencil_attachment(mut self, attachment: Option<RenderPassDepthStencilAttachment<'label>>) -> Self {
        self.depth_stencil_attachment = attachment;
        self
    }

    fn with_timestamp_writes(mut self, timestamp_writes: Option<RenderPassTimestampWrites<'label>>) -> Self {
        self.timestamp_writes = timestamp_writes;
        self
    }

    fn with_occlusion_query_set(mut self, occlusion_query_set: Option<&'label QuerySet>) -> Self {
        self.occlusion_query_set = occlusion_query_set;
        self
    }
}

crate::impl_label!(RenderPassDescriptor<'label>);