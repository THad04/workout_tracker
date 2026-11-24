use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExerciseId(Uuid);

impl ExerciseId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for ExerciseId {
    fn default() -> Self {
        ExerciseId::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SetId(Uuid);

impl SetId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn id(&self) -> Uuid {
        self.0
    }
}

impl Default for SetId {
    fn default() -> Self {
        SetId::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorkoutId(Uuid);

impl WorkoutId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn id(&self) -> Uuid {
        self.0
    }
}

impl Default for WorkoutId {
    fn default() -> Self {
        WorkoutId::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExerciseKindId(Uuid);

impl ExerciseKindId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn id(&self) -> Uuid {
        self.0
    }
}

impl Default for ExerciseKindId {
    fn default() -> Self {
        ExerciseKindId::new()
    }
}
