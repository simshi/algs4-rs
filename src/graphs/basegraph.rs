pub trait BaseGraph<'a> {
    type Iter: Iterator<Item = &'a usize>;

    fn v_size(&self) -> usize;
    fn e_size(&self) -> usize;

    fn degree(&self, v: usize) -> usize;
    fn add_edge(&mut self, v: usize, w: usize);
    // `impl trait` not ready...
    // fn adj<'a>(&'a self, v: usize) -> impl Iterator<Item = &'a usize>;
    fn adj(&'a self, v: usize) -> Self::Iter;
}
