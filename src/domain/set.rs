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

    pub fn id(&self) -> SetId {
        self.id
    }

    pub fn weight(&self) -> Weight {
        self.weight
    }

    pub fn rep_count(&self) -> Reps {
        self.rep_count
    }
}
