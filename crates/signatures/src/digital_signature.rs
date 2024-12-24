use num_bigint::BigUint;

use crate::elliptic_curves::curve::Curve;

pub struct Signature<T: Curve> {
    pub curve: T,
    pub secret: BigUint,
    pub public_key: BigUint,
}

impl<T: Curve> Signature<T> {
    /// Generates a new keypair, if not already present
    pub fn generate_keypair(&mut self) -> &mut Self {
        self.secret = self.curve.generate_secret_key();
        // self.public_key = self.secret.mod
        self
    }
}
