use chrono::NaiveDate;

#[derive(Debug, Clone)]
struct Exercise {
    kind_id: ExerciseKindId,
    sets: Vec<Set>,
}

#[derive(Debug, Clone)]
struct Set {
    rep_count: u8,
    weight_lb: f32,
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
    fn new(rep_count: u8, weight_lb: f32) -> Set {
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

fn main() {
    let set1 = Set::new(4, 75.0);
    let set2 = Set::new(8, 225.0);

    let bench_id = ExerciseKindId(1);
    let bench = Exercise::new(bench_id, vec![set1, set2]);

    let date = NaiveDate::from_ymd_opt(2026, 11, 16).unwrap();
    let mut workout = Workout::new(String::from("Push Day"), date, vec![bench]);

    let squat = Exercise::new(ExerciseKindId(2), vec![Set::new(5, 250.0)]);
    workout.add_exercise(squat);

    workout.print_workout();
}
