use super::{utils::get_sparse_vec, Float};
use nalgebra::base::dimension::{Dynamic, U1};
use nalgebra::{SliceStorage, SliceStorageMut};
use rand::{prelude::ThreadRng, Rng};

pub type Individual<'a> =
    nalgebra::Matrix<Float, U1, Dynamic, SliceStorage<'a, Float, U1, Dynamic, U1, Dynamic>>;

pub type IndividualMut<'a> =
    nalgebra::Matrix<Float, U1, Dynamic, SliceStorageMut<'a, Float, U1, Dynamic, U1, Dynamic>>;

pub trait Mutate {
    type Output;

    fn mutate(&mut self, mutation_rate: f64, rng: &mut ThreadRng);
}

pub trait Crossover {
    type Parent;

    fn crossover(
        &mut self,
        first_parent: Self::Parent,
        second_parent: Self::Parent,
        crossover_rate: f64,
        cutting_point: usize,
        rng: &mut ThreadRng,
    );

    fn random_crossover(
        &mut self,
        first_parent: Self::Parent,
        second_parent: Self::Parent,
        crossover_rate: f64,
        rng: &mut ThreadRng,
    );
}

impl<'a> Mutate for IndividualMut<'a> {
    type Output = ();

    fn mutate(&mut self, mutation_rate: f64, rng: &mut ThreadRng) {
        let sparse_vector = get_sparse_vec(
            self.iter().len(),
            (mutation_rate * self.iter().len() as f64) as usize,
            rng,
        );

        self.iter_mut()
            .zip(sparse_vector.iter())
            .for_each(|(gene, to_change)| *gene = (*gene + to_change) % 2)
    }
}

impl<'a> Crossover for IndividualMut<'a> {
    // parents do not have to be references to the Individual type ( check above ),
    // because it is a SliceStorage, which is a reference to matrix or vector
    type Parent = Individual<'a>;

    fn crossover(
        &mut self,
        first_parent: Self::Parent,
        second_parent: Self::Parent,
        crossover_rate: f64,
        cutting_point: usize,
        rng: &mut ThreadRng,
    ) {
        let first_parent_iter = first_parent.iter().cloned();

        if rng.gen_range(0f64, 1f64) > crossover_rate {
            self.iter_mut()
                .zip(first_parent_iter)
                .for_each(|(c, p)| *c = p);

            return;
        }

        let second_parent_iter = second_parent.iter().cloned();

        let parents_iter = first_parent_iter
            .take(cutting_point)
            .chain(second_parent_iter.skip(cutting_point));

        self.iter_mut().zip(parents_iter).for_each(|(c, p)| *c = p);
    }

    fn random_crossover(
        &mut self,
        first_parent: Self::Parent,
        second_parent: Self::Parent,
        crossover_rate: f64,
        rng: &mut ThreadRng,
    ) {
        let first_parent_iter = first_parent.iter().cloned();

        if rng.gen_range(0f64, 1f64) > crossover_rate {
            self.iter_mut()
                .zip(first_parent_iter)
                .for_each(|(c, p)| *c = p);

            return;
        }

        let second_parent_iter = second_parent.iter().cloned();

        let cutting_point = rng.gen_range(0usize, first_parent.iter().len());
        let parents_iter = first_parent_iter
            .take(cutting_point)
            .chain(second_parent_iter.skip(cutting_point));

        self.iter_mut().zip(parents_iter).for_each(|(c, p)| *c = p);
    }
}
