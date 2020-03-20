use crate::data_structures::{
    algorithm_configuration::AlgorithmConfiguration as Configuration, DynamicResult,
};

use crate::data_structures::{population::Population, Float};
use crate::loader;
use rand::{thread_rng, Rng};
use std::path::PathBuf;

pub fn evolve(config: &Configuration) -> DynamicResult<Vec<Float>> {
    let task = loader::read(
        config
            .input_file
            .as_ref()
            .unwrap_or(&PathBuf::from("tasks.csv")),
    )?;
    let mut population =
        Population::generate_initial_population(config.population_size, task.number_of_objects);
    let mut rand = thread_rng();

    let cutting_point =
        if config.cutting_point < task.number_of_objects || config.use_random_barrier {
            config.cutting_point
        } else if config.try_recover_from_barrier_overflow {
            rand.gen_range(0usize, task.number_of_objects)
        } else {
            // TODO: change to some rust style error
            return Err("crossover barrier is set to too big number".into());
        };

    // save crossover function before andy loop, so we don't have to branch many times
    // do not save the best individual genes, it is not needed
    let results: Vec<Float> = if config.use_random_barrier {
        (0..config.iterations)
            .map(|_| {
                population.evolve_generation_with_random_barrier(
                    &task,
                    config.tournament_size,
                    config.crossover_rate,
                    config.mutation_rate,
                    &mut rand,
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
                    &mut rand,
                )
            })
            .collect()
    };

    Ok(results)
}
