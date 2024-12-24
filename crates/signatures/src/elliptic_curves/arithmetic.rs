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
/// let point1 = CurvePoint::Affine { x: BigUint::from(1_u32), y: BigUint::from(2_u32) };
/// let point2 = CurvePoint::Affine { x: BigUint::from(3_u32), y: BigUint::from(4_u32) };
/// let result = add_two_points(point1, point2, curve);
/// println!("{:?}", result); // CurvePoint::Affine { x: ..., y: ... }
/// ```
pub fn add_two_points<T: Curve>(first: CurvePoint, second: CurvePoint, curve: &T) -> CurvePoint {
    // Handle point at infinity cases
    if first.is_infinity() {
        return second; // P + O = P
    }
    if second.is_infinity() {
        return first; // O + Q = Q
    }

    // Extract affine coordinates
    let CurvePoint::Affine { x: x1, y: y1 } = first else {
        unreachable!("Points must be affine");
    };
    let CurvePoint::Affine { x: x2, y: y2 } = second else {
        unreachable!("Points must be affine");
    };

    // Handle the case where P = -Q (result is point at infinity)
    if x1 == x2 && y1 != y2 {
        return CurvePoint::Infinity; // P + (-P) = O
    }

    // Compute the slope (lambda)
    let lambda = if x1 == x2 && y1 == y2 {
        // Doubling case: λ = (3x1^2 + a) / (2y1)
        let numerator = (BigUint::from(3_u8)
            * x1.modpow(&BigUint::from(2_u8), &curve.prime_modulus()))
            + curve.a();
        let denominator = (BigUint::from(2_u8) * y1.clone()) % &curve.prime_modulus();
        (numerator * mod_inv(denominator, &curve.prime_modulus())) % &curve.prime_modulus()
    } else {
        // Addition case: λ = (y2 - y1) / (x2 - x1)
        let numerator = (y2 + &curve.prime_modulus() - y1.clone()) % &curve.prime_modulus();
        let denominator =
            (x2.clone() + &curve.prime_modulus() - x1.clone()) % &curve.prime_modulus();
        (numerator * mod_inv(denominator, &curve.prime_modulus())) % &curve.prime_modulus()
    };

    // Compute the new x-coordinate
    let x3 = (lambda.modpow(&BigUint::from(2_u8), &curve.prime_modulus()) + &curve.prime_modulus()
        - x1.clone()
        - x2)
        % &curve.prime_modulus();

    // Compute the new y-coordinate
    let y3 = (lambda * (x1 + &curve.prime_modulus() - &x3) + &curve.prime_modulus() - y1)
        % &curve.prime_modulus();

    CurvePoint::Affine { x: x3, y: y3 }
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
