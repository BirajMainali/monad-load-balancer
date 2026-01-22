pub struct Balancer<A, B> {
    backends: Vec<Arc<B>>,
    algorithm: A,
}

impl<A, B> Balancer<A, B>
where
    A: Algorithm<B>,
{
    pub fn new(backends: Vec<Arc<B>>, algorithm: A) -> Self {
        Self {
            backends,
            algorithm,
        }
    }

    pub fn next_backend(&mut self) -> Option<Arc<B>> {
        self.algorithm.next(&self.backends)
    }
}
