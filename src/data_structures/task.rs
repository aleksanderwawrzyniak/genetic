use super::FloatDVector;

#[derive(Debug, Clone)]
pub struct Task {
    pub number_of_objects: usize,
    pub max_weight: usize,
    pub max_size: usize,
    weights: FloatDVector,
    sizes: FloatDVector,
    costs: FloatDVector,
}

impl Task {
    pub fn from_args(
        number_of_objects: usize,
        max_weight: usize,
        max_size: usize,
        weights: FloatDVector,
        sizes: FloatDVector,
        costs: FloatDVector,
    ) -> Self {
        Self {
            number_of_objects,
            max_weight,
            max_size,
            weights,
            sizes,
            costs,
        }
    }

    pub fn get_weights(&self) -> &FloatDVector {
        &self.weights
    }

    pub fn get_sizes(&self) -> &FloatDVector {
        &self.sizes
    }

    pub fn get_costs(&self) -> &FloatDVector {
        &self.costs
    }
}
