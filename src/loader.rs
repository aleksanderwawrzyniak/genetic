use super::data_structures::{task::Task, FloatDVector};
use crate::data_structures::{DynamicResult, Float};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn read<P: AsRef<Path>>(input_file: P) -> DynamicResult<Task> {
    let mut line = String::new();

    let file = File::open(input_file)?;
    let mut reader = BufReader::new(file);

    reader.read_line(&mut line)?;
    let (number_of_objects, max_weight, max_size) = get_first_line_variables(&line)?;

    let mut weights: Vec<Float> = Vec::with_capacity(number_of_objects);
    let mut sizes: Vec<Float> = Vec::with_capacity(number_of_objects);
    let mut costs: Vec<Float> = Vec::with_capacity(number_of_objects);

    for l in reader.lines() {
        let parsed_line = parse_string_to_vec(&l?)?;

        weights.push(parsed_line[0] as Float);
        sizes.push(parsed_line[1] as Float);
        costs.push(parsed_line[2] as Float);
    }
    let weights = FloatDVector::from_vec(weights);
    let sizes = FloatDVector::from_vec(sizes);
    let costs = FloatDVector::from_vec(costs);

    Ok(Task::from_args(
        number_of_objects,
        max_weight,
        max_size,
        weights,
        sizes,
        costs,
    ))
}

fn get_first_line_variables(s: &str) -> DynamicResult<(usize, usize, usize)> {
    let v: Vec<usize> = s
        .trim()
        .split(',')
        // NOTE: at this point it seems impossible to propagate error in this case using `? (try!)`
        // due to `error[E0277]: the `?` operator can only be used in a closure
        // that returns `Result` or `Option` (or another type that implements `std::ops::Try`)`
        // however, panic should be enough for now
        .map(|x| x.parse::<usize>().expect("cannot parse argument"))
        .collect();

    if v.len() < 3 {
        return Err("the first line of the file is not formatted properly".into());
    }

    Ok((v[0], v[1], v[2]))
}

fn parse_string_to_vec(s: &str) -> DynamicResult<Vec<f64>> {
    s.trim()
        .split(',')
        .map(|x| {
            x.parse::<f64>()
                // TODO: refactor error
                .map_err(|_| "cannot parse {} as f64".into())
        })
        .collect()
}
