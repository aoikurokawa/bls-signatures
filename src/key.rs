use std::io;

use ff::{PrimeField, PrimeFieldBits};
use group::Curve;
use rand_core::{CryptoRng, RngCore};

#[cfg(feature = "pairing")]
use bls12_381::{hash_to_curve::HashToField, G1Affine, G1Projective, Scalar};
#[cfg(feature = "pairing")]
use hkdf::Hkdf;
#[cfg(feature = "pairing")]
use sha2::{
    digest::{consts::U48, generic_array::GenericArray},
    Sha256,
};

#[cfg(feature = "blst")]
use blstrs::G2Affine;

pub(crate) struct ScalarRepr(<Scalar as PrimeFieldBits>::ReprBits);

use crate::signature::{hash, verify_messages};
use crate::{error::Error, signature::Signature};

pub(crate) const G1_COMPRESSED_SIZE: usize = 48;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PublicKey(pub(crate) G1Projective);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PrivateKey(pub(crate) Scalar);

impl From<G1Projective> for PublicKey {
    fn from(value: G1Projective) -> Self {
        PublicKey(value)
    }
}

impl From<PublicKey> for G1Projective {
    fn from(value: PublicKey) -> Self {
        value.0
    }
}

impl From<Scalar> for PrivateKey {
    fn from(value: Scalar) -> Self {
        PrivateKey(value)
    }
}

impl From<PrivateKey> for Scalar {
    fn from(value: PrivateKey) -> Self {
        value.0
    }
}

impl From<PrivateKey> for ScalarRepr {
    fn from(value: PrivateKey) -> Self {
        ScalarRepr(value.0.to_le_bits().into_inner())
    }
}

impl<'a> From<&'a PrivateKey> for ScalarRepr {
    fn from(value: &'a PrivateKey) -> Self {
        (*value).into()
    }
}

pub trait Serialize: ::std::fmt::Debug + Sized {
    /// Writes the key to the given writer.
    fn write_bytes(&self, dest: &mut impl io::Write) -> io::Result<()>;

    /// Recreate the key from bytes in the same form as `write_bytes` produced.
    fn from_bytes(raw: &[u8]) -> Result<Self, Error>;

    fn as_bytes(&self) -> Vec<u8> {
        let mut res = Vec::with_capacity(8 * 4);
        self.write_bytes(&mut res).expect("preallocated");
        res
    }
}

impl PrivateKey {
    /// Generate a deterministic private key from the given bytes.
    ///
    /// They must be at least 32 bytes long to be secure, will panic otherwise.
    pub fn new<T: AsRef<[u8]>>(msg: T) -> Self {
        PrivateKey(key_gen(msg))
    }

    /// Generate a new private key.
    pub fn generate<R: RngCore + CryptoRng>(rng: &mut R) -> Self {
        // IKM must be at least 32 bytes long:
        let mut ikm = [0u8; 32];
        rng.try_fill_bytes(&mut ikm)
            .expect("unable to produce secure randomness");

        Self::new(ikm)
    }

    /// Sign the given message.
    /// Calculated by `signature = hash_into_g2(message) * sk`
    #[cfg(feature = "pairing")]
    pub fn sign<T: AsRef<[u8]>>(&self, message: T) -> Signature {
        let mut p = hash(message.as_ref());
        p *= self.0;

        p.into()
    }

    /// Sign the given message.
    /// Calculated by `signature = hash_into_g2(message) * sk`
    #[cfg(feature = "blst")]
    pub fn sign<T: AsRef<[u8]>>(&self, message: T) -> Signature {
        let p = hash(message.as_ref());
        let mut sig = G2Affine::identity();

        unsafe {
            blst_lib::blst_sign_pk2_in_g1(
                std::ptr::null_mut(),
                sig.as_mut(),
                p.as_ref(),
                &self.0.into(),
            );
        }

        sig.into()
    }

    /// Get the public key for this private key.
    /// Calculated by `pk = g1 * sk`.
    #[cfg(feature = "pairing")]
    pub fn public_key(&self) -> PublicKey {
        let mut pk = G1Projective::generator();
        pk *= self.0;

        PublicKey(pk)
    }

    /// Get the public key for this private key.
    /// Calculated by `pk = g1 * sk`.
    #[cfg(feature = "blst")]
    pub fn public_key(&self) -> PublicKey {
        let mut pk = G1Affine::identity();

        unsafe {
            blst_lib::blst_sk_to_pk2_in_g1(std::ptr::null_mut(), pk.as_mut(), &self.0.into());
        }

        PublicKey(pk.into())
    }

    /// Deserialzes a private key from the field element as a decimal number.
    pub fn from_string<T: AsRef<str>>(s: T) -> Result<Self, Error> {
        match Scalar::from_str_vartime(s.as_ref()) {
            Some(f) => Ok(f.into()),
            None => Err(Error::InvalidPrivateKey),
        }
    }
}

impl Serialize for PrivateKey {
    fn write_bytes(&self, dest: &mut impl io::Write) -> io::Result<()> {
        for digit in &self.0.to_le_bits().data {
            dest.write_all(&digit.to_le_bytes())?;
        }
        Ok(())
    }

    fn from_bytes(raw: &[u8]) -> Result<Self, Error> {
        const FR_SIZE: usize = (Scalar::NUM_BITS as usize + 8 - 1) / 8;
        if raw.len() != FR_SIZE {
            return Err(Error::SizeMismatch);
        }

        let mut res = [0u8; FR_SIZE];
        res.copy_from_slice(&raw[..FR_SIZE]);

        Scalar::from_repr_vartime(res)
            .map(Into::into)
            .ok_or(Error::InvalidPrivateKey)
    }
}

impl PublicKey {
    pub fn as_affine(&self) -> G1Affine {
        self.0.to_affine()
    }

    pub fn verify<T: AsRef<[u8]>>(&self, sig: Signature, message: T) -> bool {
        verify_messages(&sig, &[message.as_ref()], &[*self])
    }
}

impl Serialize for PublicKey {
    fn write_bytes(&self, dest: &mut impl io::Write) -> io::Result<()> {
        let t = self.0.to_affine();
        let tmp = t.to_compressed();
        dest.write_all(tmp.as_ref())?;

        Ok(())
    }

    fn from_bytes(raw: &[u8]) -> Result<Self, Error> {
        if raw.len() != G1_COMPRESSED_SIZE {
            return Err(Error::SizeMismatch);
        }

        let mut res = [0u8; G1_COMPRESSED_SIZE];
        res.as_mut().copy_from_slice(raw);
        let affine: G1Affine =
            Option::from(G1Affine::from_compressed(&res)).ok_or(Error::GroupDecode)?;

        Ok(PublicKey(affine.into()))
    }
}

/// Generates a secret key as defined in
/// https://tools.ietf.org/html/draft-irtf-cfrg-bls-signature-02#section-2.3
#[cfg(feature = "pairing")]
fn key_gen<T: AsRef<[u8]>>(data: T) -> Scalar {
    // "BLS-SIG-KEYGEN-SALT-"
    const SALT: &[u8] = b"BLS-SIG-KEYGEN-SALT-";

    let data = data.as_ref();
    assert!(data.len() >= 32, "IKM must be at least 32 bytes");

    // HKDF-Extract
    let mut msg = data.as_ref().to_vec();
    // append zero byte
    msg.push(0);
    let prk = Hkdf::<Sha256>::new(Some(SALT), &msg);

    // HKDF-Expand
    // `result` has enough length to hold the output from HKDF expansion
    let mut result = GenericArray::<u8, U48>::default();
    assert!(prk.expand(&[0; 48], &mut result).is_ok());

    Scalar::from_okm(&result)
}

/// Generates a secret key as defined in
/// https://tools.ietf.org/html/draft-irtf-cfrg-bls-signature-02#section-2.3
#[cfg(feature = "blst")]
fn key_gen<T: AsRef<[u8]>>(data: T) -> Scalar {
    use std::convert::TryInto;

    let data = data.as_ref();
    assert!(data.len() >= 32, "IKM must be at least 32 bytes");

    let key_info = &[];
    let mut out = blst_lib::blst_scalar::default();
    unsafe {
        blst_lib::blst_keygen(
            &mut out,
            data.as_ptr(),
            data.len(),
            key_info.as_ptr(),
            key_info.len(),
        )
    };

    out.try_into().expect("invalid key generated")
}

