use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExerciseId(Uuid);

impl ExerciseId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SetId(Uuid);

impl SetId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorkoutId(Uuid);

impl WorkoutId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExerciseKindId(Uuid);

impl ExerciseKindId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}
