pub trait ArgMax<N> {
    fn arg_max(&self) -> usize;
}

impl<N> ArgMax<N> for Vec<N> {
    fn arg_max(&self) -> usize {
        let am = |am: (usize, N), x: &(usize, N)| if am.1 > x.1 {am} else {x};
        self.iter().enumerate().fold((0, vec[0]), am).0
    }
}