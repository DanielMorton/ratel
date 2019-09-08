use std::cmp::PartialOrd;

pub fn arg_max<N: PartialOrd>(vec: &[N]) -> usize {
    vec.iter().enumerate().max_by(|a, b| (a.1).partial_cmp(b.1).unwrap()).unwrap().0
}

pub fn arg_min<N: PartialOrd>(vec: &[N]) -> usize {
    vec.iter().enumerate().min_by(|a, b| (a.1).partial_cmp(b.1).unwrap()).unwrap().0
}

#[cfg(test)]
mod tests {
    use super::{arg_max, arg_min};

    lazy_static! {
        static ref VEC: Vec<i32> = vec![3, 1, 5, 2, 4];
    }

    #[test]
    fn test_arg_max() { assert_eq!(arg_max(&VEC), 2) }

    #[test]
    fn test_arg_min() { assert_eq!(arg_min(&VEC), 1) }
}