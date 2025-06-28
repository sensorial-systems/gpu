pub trait Label<'label> {
    fn with_label(self, label: Option<&'label str>) -> Self;
}

#[macro_export]
macro_rules! impl_label {
    ($($type:ident)::+ < $label:lifetime >) => {
        impl<$label> $crate::Label<$label> for $($type)::+<$label> {
            fn with_label(mut self, label: Option<&$label str>) -> Self {
                self.label = label;
                self
            }
        }
    };
}