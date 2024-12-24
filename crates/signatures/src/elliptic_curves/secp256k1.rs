use num_bigint::BigUint;

use super::curve::{Curve, CurvePoint};

/// Implementation of the secp256k1 elliptic curve
struct Secp256k1;

impl Curve for Secp256k1 {
    fn generator_point(&self) -> CurvePoint {
        CurvePoint::Affine {
            x: BigUint::parse_bytes(
                b"55066263022277343669578718895168534326250603453777594175500187360389116729240",
                16,
            )
            .unwrap(),
            y: BigUint::parse_bytes(
                b"32670510020758816978083085130507043184471273380659243275938904335757337482424",
                16,
            )
            .unwrap(),
        }
    }

    fn prime_modulus(&self) -> BigUint {
        BigUint::parse_bytes(
            b"115792089237316195423570985008687907853269984665640564039457584007908834671663",
            16,
        )
        .unwrap()
    }

    fn a(&self) -> BigUint {
        BigUint::from(0_u32)
    }

    fn b(&self) -> BigUint {
        BigUint::from(7_u32)
    }

    fn order(&self) -> BigUint {
        BigUint::parse_bytes(
            b"115792089237316195423570985008687907852837564279074904382605163141518161494337",
            16,
        )
        .unwrap()
    }

    fn identity(&self) -> CurvePoint {
        CurvePoint::Infinity
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::BigUint;

    use super::{Curve, Secp256k1};

    #[test]
    fn should_create_new_non_zero_secret_key() {
        let curve = Secp256k1;
        assert!(BigUint::ZERO < curve.generate_secret_key());
    }

    #[test]
    fn secret_key_should_be_less_than_the_upper_bound() {
        let curve = Secp256k1;
        assert!(curve.generate_secret_key() < curve.prime_modulus() - BigUint::from(1_u8));
    }
}
