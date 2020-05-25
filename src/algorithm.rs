use crate::data_structures::{
    algorithm_configuration::AlgorithmConfiguration as Configuration, DynamicResult,
};

use crate::data_structures::{population::Population, Float};
use crate::loader;
use nalgebra::{DMatrix, Dim};
use rand::{thread_rng, Rng};
use std::path::PathBuf;

pub fn evolve(config: &Configuration) -> DynamicResult<Vec<Float>> {
    let task = loader::read(
        config
            .input_file
            .as_ref()
            .unwrap_or(&PathBuf::from("tasks.csv")),
    )?;
    let mut population = Population::generate_initial_population(
        config.population_size,
        task.number_of_objects,
        config.density,
    );
    let mut rand = thread_rng();

    let cutting_point =
        if config.cutting_point < task.number_of_objects || config.use_random_cutting_point {
            config.cutting_point
        } else if config.try_recover_from_barrier_overflow {
            rand.gen_range(0usize, task.number_of_objects)
        } else {
            // TODO: refactor error
            return Err("crossover barrier is set to too big number".into());
        };

    // create a new matrix that will be used as workbench for creating a new population.
    // this way, I can reduce the number of allocation of new vectors, what takes a lot of time,
    // it can be done, as we are not interested in collecting all the generated generations
    let mut workbench: DMatrix<Float> = unsafe {
        DMatrix::new_uninitialized_generic(
            Dim::from_usize(population.rows()),
            Dim::from_usize(population.cols()),
        )
    };

    let results: DynamicResult<Vec<Float>> = if config.use_random_cutting_point {
        (0..config.iterations)
            .map(|_| {
                population.evolve_generation_with_random_barrier(
                    &task,
                    config.tournament_size,
                    config.crossover_rate,
                    config.mutation_rate,
                    &mut workbench,
                )
            })
            .collect()
    } else {
        (0..config.iterations)
            .map(|_| {
                population.evolve_generation(
                    &task,
                    config.tournament_size,
                    config.crossover_rate,
                    cutting_point,
                    config.mutation_rate,
                    &mut workbench,
                )
            })
            .collect()
    };

    Ok(results?)
}
