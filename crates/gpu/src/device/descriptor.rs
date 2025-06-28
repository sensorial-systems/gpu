use wgpu::{DeviceDescriptor, Features, Limits, MemoryHints, Trace};

pub trait DeviceDescriptorExt<'label> {
    fn new() -> Self;
    fn with_required_features(self, required_features: Features) -> Self;
    fn with_required_limits(self, required_limits: Limits) -> Self;
    fn with_memory_hints(self, memory_hints: MemoryHints) -> Self;
    fn with_trace(self, trace: Trace) -> Self;
}

impl<'label> DeviceDescriptorExt<'label> for DeviceDescriptor<'label> {
    fn new() -> Self {
        Default::default()
    }

    fn with_required_features(mut self, required_features: Features) -> Self {
        self.required_features = required_features;
        self
    }

    fn with_required_limits(mut self, required_limits: Limits) -> Self {
        self.required_limits = required_limits;
        self
    }

    fn with_memory_hints(mut self, memory_hints: MemoryHints) -> Self {
        self.memory_hints = memory_hints;
        self
    }

    fn with_trace(mut self, trace: Trace) -> Self {
        self.trace = trace;
        self
    }
}

crate::impl_label!(DeviceDescriptor<'label>);
