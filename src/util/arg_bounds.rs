use std::cmp::PartialOrd;

pub fn arg_max<N: PartialOrd>(vec: &[N]) -> usize {
    vec.iter()
        .enumerate()
        .max_by(|a, b| (a.1).partial_cmp(b.1).unwrap())
        .unwrap()
        .0
}

pub fn arg_min<N: PartialOrd>(vec: &[N]) -> usize {
    vec.iter()
        .enumerate()
        .min_by(|a, b| (a.1).partial_cmp(b.1).unwrap())
        .unwrap()
        .0
}

#[cfg(test)]
mod tests {
    use super::{arg_max, arg_min};

    lazy_static! {
        static ref VEC: Vec<i32> = vec![87, 26, 79, 82, 92];
        static ref FLOAT_VEC: Vec<f64> = vec![0.51, 0.8, 0.93, 0.68, 0.58];
    }

    #[test]
    fn test_arg_max() {
        assert_eq!(arg_max(&VEC), 4)
    }

    #[test]
    fn test_float_arg_max() {
        assert_eq!(arg_max(&FLOAT_VEC), 2)
    }

    #[test]
    fn test_arg_min() {
        assert_eq!(arg_min(&VEC), 1)
    }

    #[test]
    fn test_float_arg_min() {
        assert_eq!(arg_min(&FLOAT_VEC), 0)
    }
}
