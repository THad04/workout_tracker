use super::ids::{ExerciseId, ExerciseKindId};
use super::set::Set;

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
}

#[derive(Debug, Clone)]
pub struct ExerciseKind {
    id: ExerciseKindId,
    name: String,
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
