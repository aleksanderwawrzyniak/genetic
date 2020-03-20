use super::opt::Generate as GenerateConfig;
use crate::data_structures::DynamicResult;
use rand::distributions::{Distribution, Uniform};
use std::fs::File;
use std::io::Write;

fn generate_tasks(
    n: usize,
    max_w: usize,
    max_s: usize,
    multiplier: usize,
    file_name: &str,
) -> DynamicResult<(f64, f64)> {
    let mut file = File::create(&file_name)?;

    let mut rng = rand::thread_rng();
    let weight_generator = Uniform::from(1f64..(multiplier as f64 * max_w as f64 / n as f64));
    let size_generator = Uniform::from(1f64..(multiplier as f64 * max_s as f64 / n as f64));
    let cost_generator = Uniform::from(1f64..n as f64);

    file.write_fmt(format_args!("{},{},{}\n", n, max_w, max_s))?;
    let mut sum_weight = 0_f64;
    let mut sum_size = 0_f64;

    for _ in 0..n {
        let weight = weight_generator.sample(&mut rng);
        let size = size_generator.sample(&mut rng);
        let cost = cost_generator.sample(&mut rng);

        sum_size += size;
        sum_weight += weight;

        file.write_fmt(format_args!("{},{},{}\n", weight, size, cost))?;
    }

    Ok((sum_weight, sum_size))
}

pub fn generate(config: GenerateConfig) -> DynamicResult<()> {
    let n = config.number_of_items;
    let max_w = config.max_weight;
    let max_s = config.max_size;
    let multiplier = config.multiplier;
    let file_name = config.output_file.unwrap_or("tasks.csv".to_string());

    while {
        let (sum_weight, sum_size) = generate_tasks(n, max_w, max_s, multiplier, &file_name)?;
        println!(
            "weight: {} | {}; size: {} | {}",
            sum_weight,
            10 * max_w,
            sum_size,
            10 * max_s
        );

        sum_weight <= 2_f64 * max_w as f64 || sum_size <= 2_f64 * max_s as f64
    } {}

    Ok(())
}
