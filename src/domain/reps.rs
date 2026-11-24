use std::num::NonZeroU8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Reps(NonZeroU8);

impl Reps {
    pub fn new(n: u8) -> Option<Self> {
        NonZeroU8::new(n).map(Reps)
    }

    pub fn get(self) -> u8 {
        self.0.get()
    }
}

impl From<Reps> for u32 {
    fn from(value: Reps) -> u32 {
        value.get() as u32
    }
}
