use chrono::NaiveDate;

#[derive(Debug, Clone)]
struct Exercise {
    kind_id: ExerciseKindId,
    sets: Vec<Set>,
}

#[derive(Debug, Clone)]
struct Set {
    rep_count: u8,
    weight_lb: Weight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Weight(u32); // weight in tenth of a pound

impl Weight {
    fn from_lbs(lb: u32) -> Self {
        Weight(lb * 10)
    }

    fn from_tenths(t: u32) {
        Weight(t);
    }

    fn as_lb(&self) -> f32 {
        self.0 as f32 / 10.0
    }
}

#[derive(Debug, Clone)]
struct Workout {
    name: String,
    date: NaiveDate,
    exercise_list: Vec<Exercise>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ExerciseKindId(u64);

#[derive(Debug, Clone)]
struct ExerciseKind {
    id: ExerciseKindId,
    name: String,
}

impl Workout {
    fn new(name: String, date: NaiveDate, exercise_list: Vec<Exercise>) -> Workout {
        Self {
            name,
            date,
            exercise_list,
        }
    }

    fn add_exercise(&mut self, exercise: Exercise) {
        self.exercise_list.push(exercise);
    }

    fn print_workout(&self) {
        for exercise in &self.exercise_list {
            println!("{:?}", exercise);
        }
    }
}

impl Set {
    fn new(rep_count: u8, weight_lb: Weight) -> Set {
        Self {
            rep_count,
            weight_lb,
        }
    }
}

impl Exercise {
    fn new(kind_id: ExerciseKindId, sets: Vec<Set>) -> Exercise {
        Self { kind_id, sets }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_exercise_appends() {
        let date = NaiveDate::from_ymd_opt(2026, 11, 16).unwrap();
        let mut w = Workout::new("Push".to_string(), date, vec![]);
        w.add_exercise(Exercise::new(
            ExerciseKindId(1),
            vec![Set::new(4, Weight(225))],
        ));
        assert_eq!(w.exercise_list.len(), 1);
    }
}
