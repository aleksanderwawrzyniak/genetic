#[macro_use]
extern crate structopt_derive;

use crate::data_structures::Float;
use structopt::StructOpt;

mod algorithm;
mod data_structures;
mod generator;
mod loader;
mod opt;

use data_structures::population::Population;
use data_structures::{task::Task, DynamicResult};
use opt::Opt;
use std::rc::Rc;
use std::{fs::File, io::Write, time::Instant};

fn main() -> DynamicResult<()> {
    let args = Opt::from_args();
    println!("{:?}", &args);

    match args {
        Opt::Generate(config) => match generator::generate(config) {
            Ok(_) => {}
            Err(e) => println!("An error ocurred during tasks generation: {}", e),
        },
        Opt::Read(read) => match loader::read(read.file_name.unwrap_or("tasks.csv".to_string())) {
            Ok(task) => {
                let tasks: Rc<Task> = Rc::new(task);
                println!("{:?}", tasks.clone())
            }
            Err(e) => eprintln!("Error: {}", e),
        },
        Opt::Init(config) => {
            let start = Instant::now();

            match loader::read(config.file_name.unwrap_or("tasks.csv".to_string())) {
                Ok(task) => {
                    // println!("{:?}", &task);
                    let time = start.elapsed().as_secs_f32();
                    println!("{}", time);
                    let population = Population::generate_initial_population(
                        config.size,
                        task.number_of_objects,
                    );
                    let time = start.elapsed().as_secs_f32();

                    // println!(
                    //     "population size: {} x {} ",
                    //     config.size, task.number_of_objects
                    // );
                    println!("{}", time);

                    let _ = population.evaluate(&task);
                    // println!("{}", v);
                }
                Err(e) => println!("{}", e),
            }
        }
        Opt::Evolve(config) => match algorithm::evolve(&config) {
            Ok(results) => {
                println!("{:?}", &results.iter().map(|&x| x).collect::<Vec<Float>>());
                let mut output_file = File::create(&config.output_file).unwrap();
                results
                    .iter()
                    .for_each(|&x| output_file.write_fmt(format_args!("{}\n", x)).unwrap());
                println!(
                    "best individual: {}",
                    &results
                        .iter()
                        .max()
                        .expect("no populations has been raised")
                )
            }
            Err(err) => eprint!("{}", err),
        },
    }

    Ok(())
}
