use super::{utils::get_sparse_vec, Float, FloatDVector};
use nalgebra::base::dimension::{Dynamic, U1};
use nalgebra::SliceStorage;
use rand::{prelude::ThreadRng, Rng};

pub type Individual<'a> =
    nalgebra::Matrix<Float, U1, Dynamic, SliceStorage<'a, Float, U1, Dynamic, U1, Dynamic>>;

pub trait Mutate {
    type Output;
    fn mutate(&mut self, mutation_rate: f64, rng: &mut ThreadRng) -> Self::Output;
}

pub trait Crossover {
    type New;

    fn crossover(
        first_parent: Self,
        second_parent: Self,
        crossover_rate: f64,
        cutting_point: usize,
        rng: &mut ThreadRng,
    ) -> Self::New;

    fn random_crossover(
        first_parent: Self,
        second_parent: Self,
        crossover_rate: f64,
        rng: &mut ThreadRng,
    ) -> Self::New;
}

impl Mutate for FloatDVector {
    type Output = Vec<Float>;
    fn mutate(&mut self, mutation_rate: f64, rng: &mut ThreadRng) -> Self::Output {
        let n = self.nrows();
        // dbg!(n);
        let sparse_vec = get_sparse_vec(n, (n as f64 * mutation_rate) as usize, rng);

        let mutated = self
            .iter_mut()
            .zip(sparse_vec.into_iter())
            .map(|(first, second)| (*first + second) % 2)
            .collect();

        mutated
    }
}

impl<'a> Crossover for Individual<'a> {
    type New = FloatDVector;

    fn crossover(
        first_parent: Self,
        second_parent: Self,
        crossover_rate: f64,
        cutting_point: usize,
        rng: &mut ThreadRng,
    ) -> Self::New {
        if rng.gen_range(0f64, 1f64) > crossover_rate {
            return FloatDVector::from_vec(first_parent.iter().cloned().collect());
        }

        let first_parent: Vec<Float> = first_parent.into_iter().cloned().collect();
        let second_parent: Vec<Float> = second_parent.into_iter().cloned().collect();

        dbg!(&first_parent, &second_parent);

        let new_child = first_parent
            .into_iter()
            .take(cutting_point)
            .chain(second_parent.into_iter().skip(cutting_point))
            .collect();
        dbg!(&new_child);

        FloatDVector::from_vec(new_child)
    }

    fn random_crossover(
        first_parent: Self,
        second_parent: Self,
        crossover_rate: f64,
        rng: &mut ThreadRng,
    ) -> Self::New {
        if rng.gen_range(0f64, 1f64) > crossover_rate {
            return FloatDVector::from_vec(first_parent.into_iter().map(|i| *i).collect());
        }

        let parents_length = first_parent.ncols();
        // dbg!(parents_length);
        let first_parent: Vec<Float> = first_parent.into_iter().map(|i| *i).collect();
        let second_parent: Vec<Float> = second_parent.into_iter().map(|i| *i).collect();
        let cutting_point = rng.gen_range(0usize, parents_length);

        // dbg!(&first_parent, &second_parent);

        let new_child = first_parent
            .into_iter()
            .take(cutting_point)
            .chain(second_parent.into_iter().skip(cutting_point))
            .collect();
        // dbg!(&new_child);

        FloatDVector::from_vec(new_child)
    }
}
