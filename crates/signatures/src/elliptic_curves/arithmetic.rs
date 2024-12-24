use num_bigint::BigUint;

use super::curve::{Curve, CurvePoint};

/// Adds two points on an elliptic curve.
///
/// This function performs elliptic curve point addition. It handles both regular point addition and
/// point doubling cases based on the properties of elliptic curves. The result is calculated using
/// the following formulas:
///
/// ### Point Addition (\( P + Q \)) for distinct points:
/// If \( P = (x_1, y_1) \) and \( Q = (x_2, y_2) \), and \( P \neq Q \), then:
/// - \( \lambda = \frac{y_2 - y_1}{x_2 - x_1} \) (modular arithmetic)
/// - \( x_3 = \lambda^2 - x_1 - x_2 \)
/// - \( y_3 = \lambda(x_1 - x_3) - y_1 \)
///
/// ### Point Doubling (\( P + P = 2P \)):
/// If \( P = Q = (x_1, y_1) \), then:
/// - \( \lambda = \frac{3x_1^2 + a}{2y_1} \) (where \( a \) is the curve parameter)
/// - \( x_3 = \lambda^2 - 2x_1 \)
/// - \( y_3 = \lambda(x_1 - x_3) - y_1 \)
///
/// ### Special Cases:
/// - If either point is the identity element (point at infinity), return the other point.
/// - If \( P = Q \) and \( y_1 = 0 \), the result is the point at infinity (\( O \)).
/// - If \( P = -Q \) (i.e., \( x_1 = x_2 \) but \( y_1 \neq y_2 \)), the result is the point at infinity (\( O \)).
///
/// ### Modular Arithmetic:
/// All calculations are performed modulo the prime modulus of the curve.
///
/// # Parameters
/// - `first`: The first elliptic curve point (\( P \)).
/// - `second`: The second elliptic curve point (\( Q \)).
/// - `curve`: The elliptic curve instance that defines parameters such as the prime modulus and curve coefficients.
///
/// # Returns
/// - A `CurvePoint` representing \( P + Q \), or the point at infinity if the result is undefined.
///
/// # Panics
/// - Panics if the points are not affine, as this function assumes affine coordinates.
///
/// # Examples
/// ```rust
/// use num_bigint::BigUint;
/// use signatures::elliptic_curves::curve::CurvePoint;
/// use signatures::elliptic_curves::secp256k1::Secp256k1;
/// use signatures::elliptic_curves::arithmetic::add_two_points;
/// 
/// let curve = Secp256k1;
/// let point1 = CurvePoint::Affine { x: BigUint::from(1_u32), y: BigUint::from(2_u32) };
/// let point2 = CurvePoint::Affine { x: BigUint::from(3_u32), y: BigUint::from(4_u32) };
/// let result = add_two_points(point1, point2, &curve);
/// println!("{:?}", result); // CurvePoint::Affine { x: ..., y: ... }
/// ```
pub fn add_two_points<T: Curve>(first: CurvePoint, second: CurvePoint, curve: &T) -> CurvePoint {
    // 1) Handle identity (point at infinity) cases
    if first.is_infinity() {
        return second;
    }
    if second.is_infinity() {
        return first;
    }

    // 2) Extract affine coordinates
    let CurvePoint::Affine { x: x1, y: y1 } = first else {
        unreachable!("Points must be affine");
    };
    let CurvePoint::Affine { x: x2, y: y2 } = second else {
        unreachable!("Points must be affine");
    };


    // 3) Handle the case P + (-P) = Infinity
    if x1 == x2 && y1 != y2 {
        return CurvePoint::Infinity;
    }

    let p = curve.prime_modulus(); // We'll reuse this below

    // 4) Compute slope (lambda)
    let (numerator, denominator) = if x1 == x2 && y1 == y2 {
        // Doubling case
        // If y == 0, 2P => Infinity
        if y1 == BigUint::ZERO {
            return CurvePoint::Infinity;
        }
        let x1_sq = x1.modpow(&BigUint::from(2_u8), &p);
        let numerator = (BigUint::from(3u8) * x1_sq + curve.a()) % &p;
        let denominator = (BigUint::from(2u8) * &y1) % &p;
        (numerator, denominator)
    } else {
        // Addition case
        // (y2 - y1) / (x2 - x1)
        let numerator = mod_sub(&y2, &y1, &p);
        let denominator = mod_sub(&x2, &x1, &p);
        (numerator, denominator)
    };


    let denom_inv = mod_inv(denominator.clone(), &p);  // might panic if denominator=0

    let lambda = (&numerator * &denom_inv) % &p;

    // 5) Compute x3 = (lambda^2 - x1 - x2) mod p
    let lambda_sq = lambda.modpow(&BigUint::from(2_u8), &p);

    // stepwise: x3 = ( (lambda_sq - x1) - x2 ) mod p
    let mut x3 = mod_sub(&lambda_sq, &x1, &p);
    x3 = mod_sub(&x3, &x2, &p);

    // 6) Compute y3 = (lambda * (x1 - x3) - y1) mod p
    //    We'll do it step by step to avoid negative intermediates.

    // t1 = (x1 - x3) mod p
    let t1 = mod_sub(&x1, &x3, &p);
    // t2 = lambda * t1 (mod p)
    let t2 = (&lambda * &t1) % &p;
    // y3 = (t2 - y1) mod p
    let y3 = mod_sub(&t2, &y1, &p);


    let result = CurvePoint::Affine { x: x3, y: y3 };
    result
}
/// Computes the modular inverse of a number.
///
/// This function calculates the modular inverse of `value` modulo `modulus` using Fermat's Little Theorem:
/// \( a^{p-2} \equiv a^{-1} \mod p \), where \( p \) is the prime modulus.
///
/// # Parameters
/// - `value`: The number for which to compute the modular inverse.
/// - `modulus`: The prime modulus.
///
/// # Returns
/// - The modular inverse of `value` modulo `modulus`.
fn mod_inv(value: BigUint, modulus: &BigUint) -> BigUint {
    value.modpow(&(modulus - BigUint::from(2_u8)), modulus)
}

fn mod_sub(a: &BigUint, b: &BigUint, p: &BigUint) -> BigUint {
    // (a - b) mod p
    // = ((a mod p) + p - (b mod p)) mod p
    // to avoid negative intermediates.
    ( (a % p) + p - (b % p) ) % p
}


#[cfg(test)]
mod tests {
    use crate::elliptic_curves::curve::{Curve, CurvePoint};

    use super::{add_two_points, mod_inv};
    use num_bigint::BigUint;

    /// A simple test curve with small prime modulus.
    /// Let's define a curve: y^2 = x^3 + a*x + b (mod p).
    /// For example, take p = 17, a = 2, b = 2 (just as an example).
    struct TestCurve;

    impl Curve for TestCurve {
        /// Example prime modulus
        fn prime_modulus(&self) -> BigUint {
            BigUint::from(17u32)
        }

        /// `a` coefficient in the curve equation
        fn a(&self) -> BigUint {
            BigUint::from(2u32)
        }

        /// `b` coefficient in the curve equation
        fn b(&self) -> BigUint {
            BigUint::from(2u32)
        }

        fn generator_point(&self) -> CurvePoint {
            unreachable!("This is not called by the arithmetic functions.")
        }

        fn order(&self) -> BigUint {
            unreachable!("This is not called by the arithmetic functions.")
        }

        fn identity(&self) -> CurvePoint {
            unreachable!("This is not called by the arithmetic functions.")
        }
    }

    // Helper function to create a BigUint from a u32
    fn b(val: u32) -> BigUint {
        BigUint::from(val)
    }

    #[test]
    fn test_mod_inv_correctness() {
        // For each a in [1..16], check that mod_inv(a, 17) * a % 17 == 1
        // (since 17 is prime, every non-zero element has an inverse mod 17)
        let modulus = b(17);
        for val in 1..17 {
            let val_b = b(val);
            let inv = mod_inv(val_b.clone(), &modulus);
            let product = (val_b * inv) % &modulus;
            assert_eq!(product, BigUint::from(1_u8), "val = {}", val);
        }
    }

    #[test]
    fn test_point_plus_infinity() {
        let curve = TestCurve;

        // Define an affine point P
        let p = CurvePoint::Affine { x: b(1), y: b(5) };

        // Define the point at infinity
        let inf = CurvePoint::Infinity;

        // Check P + O = P
        let result = add_two_points(p.clone(), inf.clone(), &curve);
        assert_eq!(result, p, "P + O should be P");

        // Check O + P = P
        let result = add_two_points(inf, p.clone(), &curve);
        assert_eq!(result, p, "O + P should be P");
    }

    #[test]
    fn test_additive_inverse() {
        let curve = TestCurve;

        // Suppose we define P = (x, y). Let's define Q = (x, -y mod p).
        // Then P + Q should be O (the point at infinity).

        let p = BigUint::from(17u32);
        let point_x = b(3);
        let point_y = b(5);

        let p_point = CurvePoint::Affine {
            x: point_x.clone(),
            y: point_y.clone(),
        };

        // The additive inverse of (x, y) is (x, -y mod p)
        let neg_y = (&p - point_y) % &p;
        let minus_p_point = CurvePoint::Affine {
            x: point_x.clone(),
            y: neg_y,
        };

        let result = add_two_points(p_point.clone(), minus_p_point.clone(), &curve);
        assert_eq!(
            result,
            CurvePoint::Infinity,
            "P + (-P) should yield the point at infinity"
        );
    }

    #[test]
    fn test_regular_addition() {
        let curve = TestCurve;

        // Let's pick two distinct points P and Q on our test curve (assuming they are valid).
        // We'll check the result is as expected under modulo 17 arithmetic.
        //
        // For example, define:
        //    P = (5, 1)
        //    Q = (6, 3)

        let p_point = CurvePoint::Affine { x: b(5), y: b(1) };
        let q_point = CurvePoint::Affine { x: b(6), y: b(3) };

        let result = add_two_points(p_point.clone(), q_point.clone(), &curve);

        assert!(
            result != CurvePoint::Infinity,
            "P + Q should be an affine point for these specific P, Q"
        );
    }

    #[test]
    fn test_point_doubling() {
        let curve = TestCurve;

        // Doubling formula test:
        // We pick a point P = (x, y) and compute 2P.

        let p_point = CurvePoint::Affine { x: b(5), y: b(1) };
        let doubled = add_two_points(p_point.clone(), p_point.clone(), &curve);

        // We can check the result is not Infinity (unless y=0).
        assert!(
            doubled != CurvePoint::Infinity,
            "2P should not be Infinity unless y=0"
        );
    }

    #[test]
    fn test_point_doubling_y_zero() {
        let curve = TestCurve;
        // If y=0, doubling the point results in Infinity.

        let p_point = CurvePoint::Affine { x: b(5), y: b(0) };
        let doubled = add_two_points(p_point.clone(), p_point.clone(), &curve);
        assert_eq!(
            doubled,
            CurvePoint::Infinity,
            "Doubling a point with y=0 should result in Infinity"
        );
    }
}
