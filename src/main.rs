use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Debug, Clone)]
struct Exercise {
    id: ExerciseId,
    kind_id: ExerciseKindId,
    sets: Vec<Set>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ExerciseId(Uuid);

impl ExerciseId {
    fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

#[derive(Debug, Clone)]
struct Set {
    id: SetId,
    rep_count: u8,
    weight: Weight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Weight(u32); // weight in tenth of a pound

impl Weight {
    fn from_lbs(lb: u32) -> Self {
        Weight(lb * 10)
    }

    fn from_tenths(t: u32) -> Self {
        Weight(t)
    }

    fn as_lb(&self) -> f32 {
        self.0 as f32 / 10.0
    }
}

#[derive(Debug, Clone)]
struct Workout {
    id: WorkoutId,
    name: String,
    date: NaiveDate,
    exercises: Vec<Exercise>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ExerciseKindId(Uuid);

impl ExerciseKindId {
    fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct WorkoutId(Uuid);

impl WorkoutId {
    fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct SetId(Uuid);

impl SetId {
    fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

#[derive(Debug, Clone)]
struct ExerciseKind {
    id: ExerciseKindId,
    name: String,
}

impl Workout {
    fn new(name: impl Into<String>, date: NaiveDate) -> Workout {
        Self {
            id: WorkoutId::new(),
            name: name.into(),
            date,
            exercises: vec![],
        }
    }

    fn add_exercise(&mut self, exercise: Exercise) {
        self.exercises.push(exercise);
    }

    fn print_workout(&self) {
        for exercise in &self.exercises {
            println!("{:?}", exercise);
        }
    }
}

impl Set {
    fn new(rep_count: u8, weight: Weight) -> Set {
        Self {
            id: SetId::new(),
            rep_count,
            weight,
        }
    }
}

impl Exercise {
    fn new(kind_id: ExerciseKindId, sets: Vec<Set>) -> Exercise {
        Self {
            id: ExerciseId::new(),
            kind_id,
            sets,
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_exercise_appends() {
        let date = NaiveDate::from_ymd_opt(2026, 11, 16).unwrap();
        let mut w = Workout::new("Push", date);
        w.add_exercise(Exercise::new(
            ExerciseKindId::new(),
            vec![Set::new(4, Weight::from_lbs(225))],
        ));
        assert_eq!(w.exercises.len(), 1);
    }

    #[test]
    fn weight_from_lbs() {
        assert_eq!(Weight::from_lbs(225), Weight(2250));
        assert_eq!(Weight::from_lbs(225).as_lb(), 225.0);
    }

    #[test]
    fn exercises_get_unique_ids() {
        let kind = ExerciseKindId::new();
        let a = Exercise::new(kind, vec![]);
        let b = Exercise::new(kind, vec![]);
        assert_eq!(a.kind_id, b.kind_id);
        assert_ne!(a.id, b.id);
    }

    #[test]
    fn from_tenths_works() {
        assert_eq!(Weight::from_tenths(25).as_lb(), 2.5);
    }
}
