use wgpu::MultisampleState;

pub trait MultisampleStateExt {
    fn new() -> Self;
    fn with_count(self, count: u32) -> Self;
    fn with_mask(self, mask: u64) -> Self;
    fn with_alpha_to_coverage_enabled(self, alpha_to_coverage_enabled: bool) -> Self;
}

impl MultisampleStateExt for MultisampleState {
    fn new() -> Self {
        wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        }
    }

    fn with_count(mut self, count: u32) -> Self {
        self.count = count;
        self
    }

    fn with_mask(mut self, mask: u64) -> Self {
        self.mask = mask;
        self
    }

    fn with_alpha_to_coverage_enabled(mut self, alpha_to_coverage_enabled: bool) -> Self {
        self.alpha_to_coverage_enabled = alpha_to_coverage_enabled;
        self
    }
}