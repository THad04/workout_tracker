struct Exercise {
    name: String,
    sets: Vec<Set>,
}

struct Set {
    rep_count: u8,
    weight: f32,
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

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_date(&self) -> &str {
        &self.date
    }

    fn get_exercise_list(&self) -> &Vec<Exercise> {
        &self.exercise_list
    }

    fn add_exercise(&mut self, exercise: Exercise) {
        self.exercise_list.push(exercise);
    }
}

impl Set {
    fn new(rep_count: u8, weight: f32) -> Set {
        Self { rep_count, weight }
    }

    fn get_rep_count(&self) -> u8 {
        self.rep_count
    }

    fn get_weight(&self) -> f32 {
        self.weight
    }
}

impl Exercise {
    fn new(name: String, sets: Vec<Set>) -> Exercise {
        Self { name, sets }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_sets(&self) -> &Vec<Set> {
        &self.sets
    }
}

fn main() {
    println!("Hello, world!");
}
