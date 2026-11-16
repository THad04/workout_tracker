struct Exercise {
    name: String,
    weight: u64,
    sets: Vec<Set>,
}

struct Set {
    rep_count: u8,
    set_count: u8,
    weight: usize,
}

struct Workout {
    name: String,
    date: String,
    exercise_list: Vec<Exercise>,
}

impl Workout {
    fn new(name: String, date: String, exercise_list: Vec<Exercise>) -> Workout {
        Self {
            name,
            date,
            exercise_list,
        }
    }
}

impl Set {
    fn new(rep_count: u8, set_count: u8, weight: usize) -> Set {
        Self {
            rep_count,
            set_count,
            weight,
        }
    }
}

impl Exercise {
    fn new(name: String, weight: u64, sets: Vec<Set>) -> Exercise {
        Self { name, weight, sets }
    }
}

fn main() {
    println!("Hello, world!");
}
