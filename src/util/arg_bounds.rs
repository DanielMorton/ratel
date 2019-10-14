use std::cmp::PartialOrd;

pub trait ArgBounds<N: PartialOrd> {
    fn arg_max(&self) -> usize;
    fn arg_min(&self) -> usize;

    fn val_max(&self) -> N;
    fn val_min(&self) -> N;
}

impl<N: PartialOrd + Copy> ArgBounds<N> for Vec<N> {
    fn arg_max(&self) -> usize {
        self.iter()
            .enumerate()
            .max_by(|a, b| (a.1).partial_cmp(b.1).unwrap())
            .unwrap()
            .0
    }

    fn arg_min(&self) -> usize {
        self.iter()
            .enumerate()
            .min_by(|a, b| (a.1).partial_cmp(b.1).unwrap())
            .unwrap()
            .0
    }

    fn val_max(&self) -> N {
        self.iter()
            .max_by(|a, b| (*a).partial_cmp(*b).unwrap())
            .map(|x| x.to_owned())
            .unwrap()
    }

    fn val_min(&self) -> N {
        self.iter()
            .min_by(|a, b| (*a).partial_cmp(*b).unwrap())
            .map(|x| x.to_owned())
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::ArgBounds;

    lazy_static! {
        static ref VEC: Vec<i32> = vec![87, 26, 79, 82, 92];
        static ref FLOAT_VEC: Vec<f64> = vec![0.51, 0.8, 0.93, 0.68, 0.58];
    }

    #[test]
    fn test_arg_max() {
        assert_eq!(VEC.arg_max(), 4)
    }

    #[test]
    fn test_float_arg_max() {
        assert_eq!(FLOAT_VEC.arg_max(), 2)
    }

    #[test]
    fn test_arg_min() {
        assert_eq!(VEC.arg_min(), 1)
    }

    #[test]
    fn test_float_arg_min() {
        assert_eq!(FLOAT_VEC.arg_min(), 0)
    }

    #[test]
    fn test_val_max() {
        assert_eq!(VEC.val_max(), 92)
    }

    #[test]
    fn test_float_val_max() {
        assert_eq!(FLOAT_VEC.val_max(), 0.93)
    }

    #[test]
    fn test_val_min() {
        assert_eq!(VEC.val_min(), 26)
    }

    #[test]
    fn test_float_val_min() {
        assert_eq!(FLOAT_VEC.val_min(), 0.51)
    }
}
