use super::ids::SetId;
use super::reps::Reps;
use super::weight::Weight;

#[derive(Debug, Clone)]
pub struct Set {
    id: SetId,
    rep_count: Reps,
    weight: Weight,
}

impl Set {
    pub fn new(rep_count: Reps, weight: Weight) -> Set {
        Self {
            id: SetId::new(),
            rep_count,
            weight,
        }
    }

    pub fn get_weight(&self) -> Weight {
        self.weight
    }

    pub fn get_rep_count(&self) -> Reps {
        self.rep_count
    }
}
