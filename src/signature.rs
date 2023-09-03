use bls12_381::G2Projective;
#[cfg(feature = "pairing")]
use bls12_381::{
    hash_to_curve::{ExpandMsgXmd, HashToCurve},
    G1Affine, G2Affine,
};

#[cfg(feature = "blst")]
use blstrs::{G1Affine, G2Affine};

use crate::{error::Error, key::Serialize};

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

impl Serialize for Signature {
    fn write_bytes(&self, dest: &mut impl std::io::Write) -> std::io::Result<()> {
        dest.write_all(&self.0.to_compressed())?;

        Ok(())
    }

    fn from_bytes(raw: &[u8]) -> Result<Self, Error> {
        let g2 = g2_from_slice(raw)?;
        Ok(g2.into())
    }
}

fn g2_from_slice(raw: &[u8]) -> Result<G2Affine, Error> {
    if raw.len() != G2_COMPRESSED_SIZE {
        return Err(Error::SizeMismatch);
    }

    let mut res = [0u8; G2_COMPRESSED_SIZE];
    res.copy_from_slice(raw);

    Option::from(G2Affine::from_compressed(&res)).ok_or(Error::GroupDecode)
}

/// Hash the given message, as used in the signature
#[cfg(feature = "pairing")]
pub fn hash(msg: &[u8]) -> G2Projective {
    <G2Projective as HashToCurve<ExpandMsgXmd<sha2::Sha256>>>::hash_to_curve(msg, CSUITE)
}

#[cfg(feature = "blst")]
pub fn hash(msg: &[u8]) -> G2Projective {
    G2Projective::hash_to_curve(msg, CSUITE, &[])
}
