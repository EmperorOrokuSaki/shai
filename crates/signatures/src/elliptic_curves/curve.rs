use num_bigint::{BigUint, RandBigInt};
use rand::thread_rng;

use super::arithmetic::add_two_points;

#[derive(Clone, Debug, PartialEq)]
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
        rng.gen_biguint_range(&BigUint::from(1_u8), &order)
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

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigUint;

    struct DummyCurve;

    impl Curve for DummyCurve {
        fn generator_point(&self) -> CurvePoint {
            CurvePoint::Affine {
                x: BigUint::from(2u8),
                y: BigUint::from(3u8),
            }
        }

        fn prime_modulus(&self) -> BigUint {
            BigUint::from(7u8)
        }

        fn a(&self) -> BigUint {
            BigUint::from(1u8)
        }

        fn b(&self) -> BigUint {
            BigUint::from(6u8)
        }

        fn order(&self) -> BigUint {
            BigUint::from(13u8)
        }

        fn identity(&self) -> CurvePoint {
            CurvePoint::Infinity
        }
    }
    #[test]
    fn test_is_infinity() {
        let infinity_point = CurvePoint::Infinity;
        let affine_point = CurvePoint::Affine {
            x: BigUint::from(2u8),
            y: BigUint::from(3u8),
        };

        assert!(infinity_point.is_infinity());
        assert!(!affine_point.is_infinity());
    }

    #[test]
    fn test_generate_secret_key() {
        let curve = DummyCurve;
        let secret_key = curve.generate_secret_key();

        assert!(secret_key < curve.order());
        assert!(secret_key != BigUint::ZERO); // Ensure the secret key is not zero
    }

    #[test]
    fn test_calculate_public_key() {
        let curve = DummyCurve;
        let secret_key = BigUint::from(3u8);

        let public_key = curve.calculate_public_key(secret_key.clone());

        // Manually calculate the expected result (example values for dummy curve)
        let expected_public_key = add_two_points(
            add_two_points(curve.generator_point(), curve.generator_point(), &curve),
            curve.generator_point(),
            &curve,
        );

        assert_eq!(public_key, expected_public_key);
    }

    #[test]
    fn test_identity() {
        let curve = DummyCurve;
        let identity = curve.identity();

        assert!(identity.is_infinity());
    }

    #[test]
    fn test_generator_point() {
        let curve = DummyCurve;
        let generator = curve.generator_point();

        match generator {
            CurvePoint::Affine { x, y } => {
                assert_eq!(x, BigUint::from(2u8));
                assert_eq!(y, BigUint::from(3u8));
            }
            _ => panic!("Generator point is not in affine coordinates"),
        }
    }
}
