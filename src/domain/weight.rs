use std::{iter::Sum, ops::Mul};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Weight(u32); // weight in tenth of a pound

impl Weight {
    pub fn from_lbs(lb: u32) -> Self {
        Weight(lb * 10)
    }

    pub fn from_tenths(t: u32) -> Self {
        Weight(t)
    }

    pub fn as_lb(self) -> f32 {
        self.0 as f32 / 10.0
    }
}

impl Sum for Weight {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Weight(iter.map(|w| w.0).sum())
    }
}

impl Mul<u32> for Weight {
    type Output = Weight;

    fn mul(self, rhs: u32) -> Self::Output {
        Weight(self.0 * rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weight_from_lbs() {
        assert_eq!(Weight::from_lbs(225), Weight(2250));
        assert_eq!(Weight::from_lbs(225).as_lb(), 225.0);
    }

    #[test]
    fn from_tenths_works() {
        assert_eq!(Weight::from_tenths(25).as_lb(), 2.5);
    }
}
