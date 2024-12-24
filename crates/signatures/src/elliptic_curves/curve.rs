use num_bigint::{BigUint, RandBigInt};
use rand::thread_rng;

use super::arithmetic::add_two_points;

#[derive(Clone)]
pub enum CurvePoint {
    Affine { x: BigUint, y: BigUint },
    Infinity,
}

impl CurvePoint {
    /// Check if the point is the point at infinity
    pub fn is_infinity(&self) -> bool {
        matches!(self, CurvePoint::Infinity)
    }
}

/// Trait representing an elliptic curve
pub trait Curve {
    /// Returns the generator point of the curve
    fn generator_point(&self) -> CurvePoint;
    /// Returns the prime modulus \( p \)
    fn prime_modulus(&self) -> BigUint;
    /// Returns the curve parameter \( a \)
    fn a(&self) -> BigUint;
    /// Returns the curve parameter \( b \)
    fn b(&self) -> BigUint;
    /// Returns the order of the group
    fn order(&self) -> BigUint;
    /// Returns the identity point (point at infinity)
    fn identity(&self) -> CurvePoint;

    /// Generate a random secret key
    fn generate_secret_key(&self) -> BigUint {
        let mut rng = thread_rng();
        let order = self.order();
        rng.gen_biguint_below(&order)
    }

    /// Calculates the public key by scalar multiplication of the secret key with the generator point.
    ///
    /// Uses the double-and-add method to perform scalar multiplication:
    /// 1. Start with the identity point as the result.
    /// 2. For each bit of the secret key:
    ///    - If the bit is set, add the current generator multiple to the result.
    ///    - Double the current generator multiple.
    ///
    /// # Requirements
    /// This implementation requires that the `Curve` trait implementation is `Sized`.
    fn calculate_public_key(&self, secret_key: BigUint) -> CurvePoint
    where
        Self: Sized, // Add a `Sized` constraint to ensure `self` is a statically sized type
    {
        let mut result = self.identity(); // Start with the identity element (point at infinity)
        let mut current = self.generator_point(); // Start with the generator point (G)

        // Iterate over each bit of the secret key
        for i in 0..secret_key.bits() {
            // Check if the i-th bit is set
            if ((secret_key.clone() >> i) & BigUint::from(1u8)) == BigUint::from(1u8) {
                // Add the current point to the result
                result = add_two_points(result, current.clone(), self);
            }
            // Double the current point
            current = add_two_points(current.clone(), current, self);
        }

        result
    }
}
