use super::{Float, FloatDVector};
use rand::{
    distributions::{Distribution, Uniform},
    prelude::ThreadRng,
};

#[allow(non_snake_case)]
pub fn get_sparse_DVec_with(n: usize, k: usize, rng: &mut ThreadRng) -> FloatDVector {
    let v = get_sparse_vec(n, k, rng);

    FloatDVector::from_vec(v)
}

pub fn get_sparse_vec(n: usize, k: usize, rng: &mut ThreadRng) -> Vec<Float> {
    let mut v: Vec<Float> = vec![0; n];
    let dist = Uniform::from(0..n);
    let mut one_counter: usize = 0;

    while one_counter != k {
        let pos_to_one = dist.sample(rng);

        // here, we add one, as we suspect, we have got new `1` in our sparse vector
        one_counter += 1;

        // next, we subtract what was previously in the vec on our desired position.
        // If it already was `1`, we will end up with not increasing our `one_counter`
        one_counter -= v[pos_to_one] as usize;

        v[pos_to_one] = 1 as Float;
    }

    v
}
