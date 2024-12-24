use num_bigint::{BigUint, RandBigInt};
use rand::thread_rng;

pub struct CurvePoint {
    pub x: BigUint,
    pub y: BigUint,
}

struct Secp256k1;

pub trait Curve {
    fn generator_point(&self) -> CurvePoint;
    fn prime_modulus(&self) -> BigUint;
    fn a(&self) -> BigUint;
    fn b(&self) -> BigUint;
    fn order(&self) -> BigUint;
    fn generate_secret_key(&self) -> BigUint {
        let mut rng = thread_rng();
        let order = self.order();
        rng.gen_biguint_below(&order)
    }
}

impl Curve for Secp256k1 {
    fn generator_point(&self) -> CurvePoint {
        CurvePoint {
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
}
