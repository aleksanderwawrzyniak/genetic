use std::path::PathBuf;

#[derive(StructOpt, Debug, Clone)]
#[structopt(rename_all = "kebab-case")]
pub struct AlgorithmConfiguration {
    /// number of iterations, defines, how many evolutions (iterations of algorithm) will be performed
    /// should be positive, non zero integer
    #[structopt(short, long)]
    pub iterations: usize,

    /// population size,
    /// should be positive, non zero integer
    #[structopt(short = "p", long)]
    pub population_size: usize,

    /// crossover rate, defines if crossover will take place.
    /// should be positive floating point number in range [0, 1)
    #[structopt(short = "c", long)]
    pub crossover_rate: f64,

    /// crossover barrier says, how many genes are taken from the first parent,
    /// and the rest will be taken from the second.
    ///
    /// Should be positive, non negative integer.
    ///
    /// Can result in crashes, if set higher, than the `number of genes` that is set during generating
    #[structopt(short = "b", long)]
    pub cutting_point: usize,

    /// mutation rate,
    /// should be set in range [0, 1)
    #[structopt(short = "m", long)]
    pub mutation_rate: f64,

    /// tournament size,
    /// should be positive non zero number
    #[structopt(short = "t", long)]
    pub tournament_size: usize,

    /// input file with generated objects
    /// by default, it will search for "tasks.csv" file the same one,
    /// where Generate sub command will store it in,
    /// however, if the tasks were stored in different file, it must be specified
    #[structopt(long = "input", parse(from_os_str))]
    pub input_file: Option<PathBuf>,

    /// file to store the output in
    #[structopt(long = "output", parse(from_os_str))]
    pub output_file: PathBuf,

    /// if this flag is set, the program will prevent crash if `crossover-barrier` is set too high
    /// and change it to random, safe value.
    #[structopt(short = "r", long = "try-recover")]
    pub try_recover_from_barrier_overflow: bool,

    /// if this flag is set, the program will always use random `crossover-barrier` size
    /// for each crossover
    #[structopt(short = "R", long = "rand-crossover-barrier")]
    pub use_random_barrier: bool,
}
