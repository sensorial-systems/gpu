use wgpu::{Operations, LoadOp, StoreOp};

pub trait OperationsExt<T> {
    fn new(load: LoadOp<T>, store: StoreOp) -> Self;
}

impl<T> OperationsExt<T> for Operations<T> {
    fn new(load: LoadOp<T>, store: StoreOp) -> Self {
        Operations { load, store }
    }
}
