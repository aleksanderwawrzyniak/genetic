use nalgebra::{DMatrix, DVector};
use std::error::Error;

pub mod algorithm_configuration;
pub mod individual;
pub mod population;
pub mod task;
pub mod utils;

// pub type Float = f64;
pub type Float = u32;
pub type FloatDVector = DVector<Float>;
pub type DynamicResult<T> = Result<T, Box<dyn Error>>;
pub type PopulationMatrix = DMatrix<Float>;
