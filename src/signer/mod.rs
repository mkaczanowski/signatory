//! Unified signing API for all Signatory providers

pub(crate) mod bytes;
#[cfg(all(feature = "digest", feature = "generic-array"))]
pub(crate) mod digest;
pub(crate) mod sha2;

use error::Error;
use Signature;

#[cfg(all(feature = "digest", feature = "generic-array"))]
pub use self::digest::DigestSigner;
pub use self::{bytes::ByteSigner, sha2::Sha256Signer};

/// Common trait for all signature providers
pub trait Signer<I, S: Signature>: Send + Sync {
    /// Sign the given input (i.e. message or digest) with this signer's
    /// private key, returning a signature.
    ///
    /// This trait should be implemented for the input type which is closest
    /// to the provider's own API, i.e. if the provider's signing API is
    /// designed to sign message digests, this should accept digests as input.
    fn sign(&self, input: I) -> Result<S, Error>;
}