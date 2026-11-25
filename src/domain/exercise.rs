use super::ids::{ExerciseId, ExerciseKindId, SetId};
use super::reps::Reps;
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

    pub fn add_set(&mut self, set: Set) {
        self.sets.push(set);
    }

    /// Inserts a copy of the given set right after it.
    /// Returns the new set's id, or None if no set has that id.
    pub fn duplicate_set(&mut self, id: SetId) -> Option<SetId> {
        let idx = self.sets.iter().position(|s| s.id() == id)?;
        let copy = self.sets[idx].duplicate();
        let new_id = copy.id();
        self.sets.insert(idx + 1, copy);
        Some(new_id)
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

    #[test]
    fn add_set_to_exercise_then_duplicate() {
        let mut a = Exercise::new(ExerciseKindId::new(), vec![]);
        let set = Set::new(Reps::new(4).unwrap(), Weight::from_lbs(25));
        let set_id = set.id();
        a.add_set(set);
        let new_id = a.duplicate_set(set_id).unwrap();
        assert_eq!(a.sets().len(), 2);

        let original = &a.sets()[0];
        let copy = &a.sets[1];
        assert_eq!(original.id(), set_id);
        assert_eq!(copy.id(), new_id);
        assert_ne!(copy.id(), original.id());
        assert_eq!(copy.rep_count(), original.rep_count());
        assert_eq!(copy.weight(), original.weight());
    }
}
