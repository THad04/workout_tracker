use chrono::NaiveDate;

use super::exercise::Exercise;
use super::ids::WorkoutId;
use super::weight::Weight;

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

    pub fn total_volume(&self) -> Weight {
        self.exercises.iter().map(|x| x.exercise_volume()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Reps;
    use crate::domain::ids::ExerciseKindId;
    use crate::domain::set::Set;
    use crate::domain::weight::Weight;

    #[test]
    fn add_exercise_appends() {
        let date = NaiveDate::from_ymd_opt(2026, 11, 16).unwrap();
        let mut w = Workout::new("Push", date);
        w.add_exercise(Exercise::new(
            ExerciseKindId::new(),
            vec![Set::new(Reps::new(4).unwrap(), Weight::from_lbs(225))],
        ));
        assert_eq!(w.exercises.len(), 1);
    }

    #[test]
    fn total_volume_sums_weight_times_reps() {
        let date = NaiveDate::from_ymd_opt(2026, 11, 24).unwrap();
        let mut w = Workout::new("Upper", date);
        assert_eq!(w.total_volume(), Weight::from_lbs(0));

        // Add one exercise
        w.add_exercise(Exercise::new(
            ExerciseKindId::new(),
            vec![Set::new(Reps::new(4).unwrap(), Weight::from_lbs(225))], // 900 lbs of volume
        ));
        assert_eq!(w.total_volume(), Weight::from_lbs(900));

        // Check total after adding two exercises
        w.add_exercise(Exercise::new(
            ExerciseKindId::new(),
            vec![
                // total: 480 lbs of volume
                Set::new(Reps::new(4).unwrap(), Weight::from_lbs(60)), // 240 lbs of volume
                Set::new(Reps::new(4).unwrap(), Weight::from_lbs(60)), // 240 lbs of volume
            ],
        ));
        assert_eq!(w.total_volume(), Weight::from_lbs(1380)); // 900 lbs + 480 lbs = 1380 lbs
    }
}
