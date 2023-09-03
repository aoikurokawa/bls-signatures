use bls12_381::G2Projective;
#[cfg(feature = "pairing")]
use bls12_381::{G1Affine, G2Affine};

#[cfg(feature = "blst")]
use blstrs::{G1Affine, G2Affine};

const CSUITE: &[u8] = b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_NUL_";
const G2_COMPRESSED_SIZE: usize = 96;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Signature(G2Affine);

impl From<G2Projective> for Signature {
    fn from(value: G2Projective) -> Self {
        Signature(value.into())
    }
}

impl From<Signature> for G2Projective {
    fn from(value: Signature) -> Self {
        value.0.into()
    }
}

impl From<G2Affine> for Signature {
    fn from(value: G2Affine) -> Self {
        Signature(value)
    }
}

impl From<Signature> for G2Affine {
    fn from(value: Signature) -> Self {
        value.0
    }
}


