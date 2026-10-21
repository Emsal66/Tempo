//! Platform abstraction traits implemented by the shell (`src-tauri`).
//!
//! Core logic depends only on these traits so it builds and tests on Linux;
//! the Windows implementations (Credential Manager, power notifications) live
//! in the shell behind `#[cfg(windows)]`.

use std::fmt;

use zeroize::Zeroizing;

/// The 256-bit vault master key (D-18).
///
/// Wiped from memory on drop. Deliberately has no `Clone` and no
/// `Serialize`, and its `Debug` output never contains key material.
pub struct MasterKey(Zeroizing<[u8; 32]>);

impl MasterKey {
    /// Length of the key in bytes.
    pub const LEN: usize = 32;

    /// Generates a fresh random key from the OS RNG (via aes-gcm's key
    /// generation, which uses `getrandom`).
    pub fn generate() -> MasterKey {
        use aes_gcm::Aes256Gcm;
        use aes_gcm::aead::{Generate, Key};

        let mut generated = Key::<Aes256Gcm>::generate();
        let mut bytes = Zeroizing::new([0u8; Self::LEN]);
        bytes.copy_from_slice(generated.as_slice());
        zeroize::Zeroize::zeroize(generated.as_mut_slice());
        MasterKey(bytes)
    }

    /// Builds a key from raw bytes (for example loaded from Credential
    /// Manager). Rejects anything that is not exactly 32 bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<MasterKey, KeyProviderError> {
        if bytes.len() != Self::LEN {
            return Err(KeyProviderError::Corrupt(format!(
                "ana anahtar {} bayt olmalı, {} bayt bulundu",
                Self::LEN,
                bytes.len()
            )));
        }
        let mut key = Zeroizing::new([0u8; Self::LEN]);
        key.copy_from_slice(bytes);
        Ok(MasterKey(key))
    }

    /// Borrows the raw key bytes. Never log or persist the result outside a
    /// `KeyProvider`.
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Debug for MasterKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("MasterKey(<redacted>)")
    }
}

/// Errors from a [`KeyProvider`].
#[derive(Debug, thiserror::Error)]
pub enum KeyProviderError {
    /// The key store cannot be reached (Credential Manager error, ...).
    #[error("anahtar deposuna ulaşılamıyor: {0}")]
    Unavailable(String),
    /// A stored key exists but is malformed.
    #[error("kayıtlı anahtar bozuk: {0}")]
    Corrupt(String),
}

/// Source of the vault master key (Windows: Credential Manager, local
/// persistence; tests: in-memory mock).
pub trait KeyProvider: Send + Sync {
    /// Loads the stored key, `Ok(None)` when no key has been stored yet.
    fn load(&self) -> Result<Option<MasterKey>, KeyProviderError>;
    /// Stores (or replaces) the key.
    fn store(&self, key: &MasterKey) -> Result<(), KeyProviderError>;
    /// Deletes the stored key. Deleting a missing key is not an error.
    fn delete(&self) -> Result<(), KeyProviderError>;
}

/// OS power transitions forwarded by the shell to the scheduler.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerEvent {
    /// The machine is about to sleep or hibernate.
    Suspend,
    /// The machine resumed from sleep or hibernate.
    Resumed,
}
