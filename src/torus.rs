use std::ops::{Add, Mul, Sub};

use crypto_bigint::{
    modular::runtime_mod::{DynResidue, DynResidueParams},
    AddMod, Uint, U256,
};
use fast_ntt::numbers::{BigIntType::U16, *};
use num_bigfloat::BigFloat;

#[derive(Debug, Clone, Copy)]
pub struct Torus {
    v: BigFloat,
}

impl Torus {
    pub fn new(v: BigFloat, q: u128) -> Torus {
        // TODO optimize exponentiations
        let v = v * BigFloat::from(2).pow(&BigFloat::from(q));
        Torus { v }
    }
}

impl Add for Torus {
    type Output = Torus;

    fn add(self, rhs: Self) -> Self::Output {
        Torus {
            v: (self.v + rhs.v).rem(&BigFloat::from(1)),
        }
    }
}

impl Sub for Torus {
    type Output = Torus;

    fn sub(self, rhs: Self) -> Self::Output {
        Torus {
            v: (self.v - rhs.v).rem(&BigFloat::from(1)),
        }
    }
}

impl Mul for Torus {
    type Output = Torus;

    fn mul(self, rhs: Self) -> Self::Output {
        Torus {
            v: (self.v * rhs.v).rem(&BigFloat::from(1)),
        }
    }
}

impl PartialEq for Torus {
    fn eq(&self, other: &Self) -> bool {
        self.v == other.v
    }
}

#[cfg(test)]
mod tests {
    use super::Torus;
    use num_bigfloat::BigFloat;
    #[test]
    fn test_new() {
        let q = 32;
        let a = Torus::new(BigFloat::from(0.25), q);
        let b = Torus::new(BigFloat::from(0.4), q);
        let c = Torus::new(BigFloat::from(0.3), q);

        let lhs = (a + b) * c;
        let rhs = a * c + b * c;

        assert_ne!(lhs, rhs);
    }
}
