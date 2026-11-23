use super::ids::SetId;
use super::weight::Weight;

#[derive(Debug, Clone)]
pub struct Set {
    id: SetId,
    rep_count: u8,
    weight: Weight,
}

impl Set {
    pub fn new(rep_count: u8, weight: Weight) -> Set {
        Self {
            id: SetId::new(),
            rep_count,
            weight,
        }
    }
}
