use chrono::NaiveDate;

use super::exercise::Exercise;
use super::ids::WorkoutId;

#[derive(Debug, Clone)]
pub struct Workout {
    id: WorkoutId,
    name: String,
    date: NaiveDate,
    exercises: Vec<Exercise>,
}

impl Workout {
    pub fn new(name: impl Into<String>, date: NaiveDate) -> Workout {
        Self {
            id: WorkoutId::new(),
            name: name.into(),
            date,
            exercises: vec![],
        }
    }

    pub fn add_exercise(&mut self, exercise: Exercise) {
        self.exercises.push(exercise);
    }

    pub fn print_workout(&self) {
        for exercise in &self.exercises {
            println!("{:?}", exercise);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::ids::ExerciseKindId;
    use crate::domain::set::Set;
    use crate::domain::weight::Weight;

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
}
