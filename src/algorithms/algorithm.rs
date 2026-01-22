use std::sync::Arc;

pub trait Algorithm<Backend> {
    fn next(&mut self, backends: &[Arc<Backend>]) -> Option<Arc<Backend>>;
}
