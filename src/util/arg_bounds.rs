fn arg_max<N>(vec: Vec<N>) -> usize {
    let am = |am: (usize, N), x: &(usize, N)| if am.1 > x.1 { am } else { x };
    vec.iter().enumerate().fold((0, vec[0]), am).0
}

fn arg_min<N>(vec: Vec<N>) -> usize {
    let am = |am: (usize, N), x: &(usize, N)| if am.1 < x.1 { am } else { x };
    vec.iter().enumerate().fold((0, vec[0]), am).0
}