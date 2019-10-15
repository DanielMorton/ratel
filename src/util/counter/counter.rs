use num_traits::Num;

pub trait Counter<T: Num> {
    fn counter(&self) -> &T;

    fn reset(&mut self) -> ();

    fn update(&mut self, n: T) -> ();
}
