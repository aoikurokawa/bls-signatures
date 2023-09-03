use std::sync::atomic::{AtomicBool, Ordering};

use bls12_381::{G1Affine, Gt};
#[cfg(feature = "multicore")]
use rayon::prelude::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
};

#[cfg(feature = "pairing")]
use bls12_381::{
    hash_to_curve::{ExpandMsgXmd, HashToCurve},
    Bls12, G2Affine, G2Projective, MillerLoopResult,
};
use pairing_lib::MultiMillerLoop;

#[cfg(feature = "blst")]
use blstrs::{G1Affine, G2Affine};

use crate::{
    error::Error,
    key::{PublicKey, Serialize},
};

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

/// Aggregate signatures by multiplying them together.
/// Calculated by `signature = \sum_{i = 0}^n signature_i`.
#[cfg(feature = "multicore")]
pub fn aggregate(signatures: &[Signature]) -> Result<Signature, Error> {
    if signatures.is_empty() {
        return Err(Error::ZeroSizedInput);
    }

    let res = signatures
        .into_par_iter()
        .fold(G2Projective::identity, |mut acc, signature| {
            acc += &signature.0;
            acc
        })
        .reduce(G2Projective::identity, |acc, val| acc + val);

    Ok(Signature(res.into()))
}

/// Aggregate signatures by multiplying them together.
/// Calculated by `signature = \sum_{i = 0}^n signature_i`.
#[cfg(not(feature = "multicore"))]
pub fn aggregate(signatures: &[Signature]) -> Result<Signature, Error> {
    if signatures.is_empty() {
        return Err(Error::ZeroSizedInput);
    }

    let res = signatures
        .into_iter()
        .fold(G2Projective::identity(), |acc, signature| {
            acc + &signature.0
        });

    Ok(Signature(res.into()))
}

/// Verifies that the signature is the actual aggreated signature of hashes - pubkeys.
/// Calculated by `e(g1, signature) == \prod_{i = 0}^n e(pk_i, hash_i)`.
pub fn verify(signature: &Signature, hashes: &[G2Projective], public_keys: &[PublicKey]) -> bool {
    if hashes.is_empty() || public_keys.is_empty() {
        return false;
    }

    let n_hashes = hashes.len();

    if n_hashes != public_keys.len() {
        return false;
    }

    // zero key & single hash should fail
    if n_hashes == 1 && public_keys[0].0.is_identity().into() {
        return false;
    }

    // Enforce that messages are distinct as a countermeasure against BLS's rogue-key attack.
    // See Section 3.1. of the IRTF's BLS signatures spec:
    for i in 0..(n_hashes - 1) {
        for j in (i + 1)..n_hashes {
            if hashes[i] == hashes[j] {
                return false;
            }
        }
    }

    let is_valid = AtomicBool::new(true);

    #[cfg(feature = "multicore")]
    let mut ml = public_keys
        .par_iter()
        .zip(hashes.par_iter())
        .map(|(pk, h)| {
            if pk.0.is_identity().into() {
                is_valid.store(false, Ordering::Relaxed);
            }
            let pk = pk.as_affine();
            let h = G2Affine::from(h).into();
            Bls12::multi_miller_loop(&[(&pk, &h)])
        })
        .reduce(MillerLoopResult::default, |acc, cur| acc + cur);

    #[cfg(not(feature = "multicore"))]
    let mut ml = public_keys
        .iter()
        .zip(hashes.par_iter())
        .map(|(pk, h)| {
            if pk.0.is_identity().into() {
                is_valid.store(false, Ordering::Relaxed);
            }
            let pk = pk.as_affine();
            let h = G2Affine::from(h).into();
            Bls12::multi_miller_loop(&[(&pk, &h)])
        })
        .reduce(MillerLoopResult::default(), |acc, cur| acc + cur);

    if !is_valid.load(Ordering::Relaxed) {
        return false;
    }

    let g1_neg = -G1Affine::generator();

    ml += Bls12::multi_miller_loop(&[(&g1_neg, &signature.0.into())]);

    ml.final_exponentiation() == Gt::identity()
}

/// Verifies that the signature is the actual aggregated signature of messages - pubkeys.
/// Calculated by `e(g1 - signature) == \prod_{i = 0}^n e(pk_i, hash_i)`.
#[cfg(feature = "pairing")]
pub fn verify_messages(
    signature: &Signature,
    messages: &[&[u8]],
    public_keys: &[PublicKey],
) -> bool {
    #[cfg(feature = "multicore")]
    let hashes: Vec<_> = messages.par_iter().map(|msg| hash(msg)).collect();

    #[cfg(not(feature = "multicore"))]
    let hashes: Vec<_> = messages.iter().map(|msg| hash(msg)).collect();

    verify(signature, &hashes, public_keys)
}

/// Verifies that the signature is the actual aggregated signature of messages - pubkeys.
/// Calculated by `e(g1 - signature) == \prod_{i = 0}^n e(pk_i, hash_i)`.
#[cfg(all(feature = "blst", feature = "multicore"))]
pub fn verify_messages(
    signature: &Signature,
    messages: &[&[u8]],
    public_keys: &[PublicKey],
) -> bool {
    if messages.is_empty() || public_keys.is_empty() {
        return false;
    }

    let n_messages = messages.len();

    if n_messages != public_keys.len() {
        return false;
    }

    // zero key & single message should fail
    if n_messages == 1 && public_keys[0].0.is_identity().into() {
        return false;
    }

    // Enforce that messages are distinct as a countermeasure against BLS's rogue-key attack.
    if !blstrs::unique_messages(messages) {
        return false;
    }

    let valid = AtomicBool::new(true);

    let n_workers = std::cmp::min(rayon::current_num_threads(), n_messages);
    let mut pairings = messages
        .par_iter()
        .zip(public_keys.par_iter())
        .chunks(n_messages / n_workers)
        .map(|chunk| {
            let mut pairing = blstrs::PairingG1G2::new(true, CSUITE);

            for (message, public_key) in chunk {
                let res = pairing.aggregate(&public_key.0.into(), None, message, &[]);
                if res.is_err() {
                    valid.store(false, Ordering::Relaxed);
                    break;
                }
            }
            if valid.load(Ordering::Relaxed) {
                pairing.commit();
            }

            pairing
        })
        .collect::<Vec<_>>();

    let mut gtsig = Gt::default();
    if valid.load(Ordering::Relaxed) {
        blstrs::PairingG1G2::aggregated(&mut gtsig, &signature.0);
    }

    let mut acc = pairings.pop().unwrap();
    for pairing in &pairings {
        let res = acc.merge(pairing);
        if res.is_err() {
            return false;
        }
    }

    valid.load(Ordering::Relaxed) && acc.finalverify(Some(&gtsig))
}

/// Verifies that the signature is the actual aggregated signature of messages - pubkeys.
/// Calculated by `e(g1 - signature) == \prod_{i = 0}^n e(pk_i, hash_i)`.
#[cfg(all(feature = "blst", not(feature = "multicore")))]
pub fn verify_messages(
    signature: &Signature,
    messages: &[&[u8]],
    public_keys: &[PublicKey],
) -> bool {
    if messages.is_empty() || public_keys.is_empty() {
        return false;
    }

    let n_messages = messages.len();

    if n_messages != public_keys.len() {
        return false;
    }

    // zero key & single message should fail
    if n_messages == 1 && public_keys[0].0.is_identity().into() {
        return false;
    }

    // Enforce that messages are distinct as a countermeasure against BLS's rogue-key attack.
    if !blstrs::unique_messages(messages) {
        return false;
    }

    let mut valid = true;
    let mut pairing = blstrs::PairingG1G2::new(true, CSUITE);

    for (message, public_key) in messages.iter().zip(public_keys.iter()) {
        let res = pairing.aggregate(&public_key.0.into(), None, message, &[]);
        if res.is_err() {
            valid = false;
            break;
        }

        pairing.commit();
    }

    let mut gtsig = Gt::default();
    if valid {
        blstrs::PairingG1G2::aggregated(&mut gtsig, &signature.0);
    }

    valid && pairing.finalverify(Some(&gtsig))
}
