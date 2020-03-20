use super::{
    individual::{Crossover, Individual, Mutate},
    task::Task,
    utils::get_sparse_DVec_with,
    Float, FloatDVector, PopulationMatrix,
};
use crossbeam::thread;
use nalgebra::base::dimension::Dim;
use rand::{
    distributions::{Distribution, Uniform},
    prelude::ThreadRng,
    thread_rng,
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Debug)]
pub struct Population {
    population: PopulationMatrix,
}

impl Population {
    #[inline(always)]
    pub fn generate_initial_population(population_size: usize, number_of_elements: usize) -> Self {
        let dist = Uniform::from(0..15 as Float);

        let matrix: Vec<Float> = (0..number_of_elements * population_size)
            .into_par_iter()
            .map_init(|| thread_rng(), |rng, _| (dist.sample(rng) == 1) as Float)
            .collect();

        let matrix = PopulationMatrix::from_vec_generic(
            Dim::from_usize(population_size),
            Dim::from_usize(number_of_elements),
            matrix,
        );

        Self { population: matrix }
    }

    pub fn evaluate(&self, task: &Task) -> FloatDVector {
        thread::scope(|s| {
            let sum_weights_thread = s.spawn(|_| {
                Self::get_evaluated_vec(&self.population, task.get_weights(), task.max_weight)
            });
            let sum_sizes_thread = s.spawn(|_| {
                Self::get_evaluated_vec(&self.population, task.get_sizes(), task.max_size)
            });
            let sum_costs_thread =
                s.spawn(|_| Self::get_summed_vec(&self.population, task.get_costs()));

            // TODO: come up with some more useful panic messages!
            let mut costs_vec = sum_costs_thread.join().expect("sum_costs_thread panicked");

            costs_vec
                .component_mul_assign(&sum_sizes_thread.join().expect("sum_sizes_thread panicked"));
            costs_vec.component_mul_assign(
                &sum_weights_thread
                    .join()
                    .expect("sum_weights_thread panicked"),
            );

            costs_vec
        })
        .unwrap()
    }

    pub fn evolve_generation(
        &mut self,
        task: &Task,
        tournament_size: usize,
        crossover_rate: f64,
        cutting_point: usize,
        mutation_rate: f64,
        rand: &mut ThreadRng,
    ) -> Float {
        let evaluation = self.evaluate(&task);

        let (_, best) = evaluation.argmax();

        self.population = Self::get_new_generation(
            &self.population,
            &evaluation,
            tournament_size,
            crossover_rate,
            cutting_point,
            mutation_rate,
            rand,
        );

        best
    }

    pub fn evolve_generation_with_random_barrier(
        &mut self,
        task: &Task,
        tournament_size: usize,
        crossover_rate: f64,
        mutation_rate: f64,
        rand: &mut ThreadRng,
    ) -> Float {
        let evaluation = self.evaluate(&task);

        let (_, best) = evaluation.argmax();

        self.population = Self::get_new_generation_with_random_barrier(
            &self.population,
            &evaluation,
            tournament_size,
            crossover_rate,
            mutation_rate,
            rand,
        );

        best
    }

    fn get_new_generation(
        population: &PopulationMatrix,
        evaluation: &FloatDVector,
        tournament_size: usize,
        crossover_rate: f64,
        cutting_point: usize,
        mutation_rate: f64,
        rand: &mut ThreadRng,
    ) -> PopulationMatrix {
        let new_generation: Vec<Float> = (0..population.nrows())
            .map(|_| {
                let first_parent = tournament_selection(tournament_size, &evaluation, rand);
                let second_parent = tournament_selection(tournament_size, &evaluation, rand);
                let mut individual = Individual::crossover(
                    population.row(first_parent),
                    population.row(second_parent),
                    crossover_rate,
                    cutting_point,
                    rand,
                );

                individual.mutate(mutation_rate, rand)
            })
            .flatten()
            .collect();

        PopulationMatrix::from_vec(population.nrows(), population.ncols(), new_generation)
    }

    fn get_new_generation_with_random_barrier(
        population: &PopulationMatrix,
        evaluation: &FloatDVector,
        tournament_size: usize,
        crossover_rate: f64,
        mutation_rate: f64,
        rand: &mut ThreadRng,
    ) -> PopulationMatrix {
        let new_generation: Vec<Float> = (0..population.nrows())
            .map(|_| {
                let first_parent = tournament_selection(tournament_size, &evaluation, rand);
                let second_parent = tournament_selection(tournament_size, &evaluation, rand);
                let mut individual = Individual::random_crossover(
                    population.row(first_parent),
                    population.row(second_parent),
                    crossover_rate,
                    rand,
                );

                individual.mutate(mutation_rate, rand)
            })
            .flatten()
            .collect();

        PopulationMatrix::from_vec(population.nrows(), population.ncols(), new_generation)
    }

    fn get_evaluated_vec(
        population: &PopulationMatrix,
        vec: &FloatDVector,
        max_sum: usize,
    ) -> FloatDVector {
        let mut summed_vec = Self::get_summed_vec(&population, vec);
        let data_vec = unsafe { summed_vec.data.as_vec_mut() };
        data_vec.into_par_iter().for_each(|elem| {
            // println!("{} | {} :: {}", &elem, &max_sum, (*elem > max_sum as Float));
            *elem = (*elem <= max_sum as Float) as u8 as Float
        });

        summed_vec
    }

    fn get_summed_vec(population: &PopulationMatrix, vec: &FloatDVector) -> FloatDVector {
        population * vec
    }
}

pub fn tournament_selection(
    tournament_size: usize,
    evaluation: &FloatDVector,
    rng: &mut ThreadRng,
) -> usize {
    let mut sparse_tournament = get_sparse_DVec_with(evaluation.nrows(), tournament_size, rng);
    sparse_tournament.component_mul_assign(evaluation);

    let (winner_pos, _) = sparse_tournament.argmax();
    // dbg!(winner_pos, winner_score);

    winner_pos
}

impl From<PopulationMatrix> for Population {
    fn from(matrix: PopulationMatrix) -> Self {
        Self { population: matrix }
    }
}
