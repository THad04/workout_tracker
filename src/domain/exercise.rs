use super::ids::{ExerciseId, ExerciseKindId};
use super::set::Set;
use super::weight::Weight;

#[derive(Debug, Clone)]
pub struct Exercise {
    id: ExerciseId,
    kind_id: ExerciseKindId,
    sets: Vec<Set>,
}

impl Exercise {
    pub fn new(kind_id: ExerciseKindId, sets: Vec<Set>) -> Exercise {
        Self {
            id: ExerciseId::new(),
            kind_id,
            sets,
        }
    }

    pub fn id(&self) -> ExerciseId {
        self.id
    }

    pub fn kind_id(&self) -> ExerciseKindId {
        self.kind_id
    }

    pub fn sets(&self) -> &[Set] {
        &self.sets
    }

    pub fn exercise_volume(&self) -> Weight {
        self.sets
            .iter()
            .map(|x| x.weight() * x.rep_count().into())
            .sum()
    }
}

#[derive(Debug, Clone)]
pub struct ExerciseKind {
    id: ExerciseKindId,
    name: String,
}

impl ExerciseKind {
    pub fn new(name: String) -> ExerciseKind {
        ExerciseKind {
            id: ExerciseKindId::new(),
            name,
        }
    }

    pub fn id(&self) -> ExerciseKindId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exercises_get_unique_ids() {
        let kind = ExerciseKindId::new();
        let a = Exercise::new(kind, vec![]);
        let b = Exercise::new(kind, vec![]);
        assert_eq!(a.kind_id, b.kind_id);
        assert_ne!(a.id, b.id);
    }
}
