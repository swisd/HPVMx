use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::cmp::PartialEq;
use spin::Mutex;
use crate::keys::types::KUID;

/// Defines an error type specific to your custom OS keyring operations
#[derive(Debug, PartialEq, Eq)]
pub enum KeyringError {
    NotFound,
    AccessDenied,
    StorageFull,
}

pub struct Key {
    pub payload: Vec<u8>,
    pub owner_uid: KUID,
}

/// The global Keyring Manager state
pub struct KeyringManager {
    store: BTreeMap<(Vec<u8>, Vec<u8>), Key>,
}

// Global thread-safe instance available across the OS environment
pub static mut GLOBAL_KEYRING: Mutex<KeyringManager> = Mutex::new(KeyringManager {
    store: BTreeMap::new(),
});


impl KeyringManager {
    /// Inserts a new secret into the keyring or updates an existing one
    pub fn set_secret(
        &mut self,
        service: &[u8],
        name: &[u8],
        secret: &[u8],
        caller_uid: KUID
    ) -> Result<(), KeyringError> {
        let key = (service.to_vec(), name.to_vec());

        // Security check: If it already exists, verify ownership
        if let Some(existing) = self.store.get(&key) {
            if existing.owner_uid != caller_uid {
                return Err(KeyringError::AccessDenied);
            }
        }

        let new_key = Key {
            payload: secret.to_vec(),
            owner_uid: caller_uid,
        };

        self.store.insert(key, new_key);
        Ok(())
    }

    /// Retrieves a secret if it exists and the caller has proper permissions
    pub fn get_secret(
        &self,
        service: &[u8],
        name: &[u8],
        caller_uid: KUID
    ) -> Result<Vec<u8>, KeyringError> {
        let key = (service.to_vec(), name.to_vec());

        match self.store.get(&key) {
            Some(entry) => {
                // Ensure isolation: processes cannot steal other processes' keys
                if entry.owner_uid == caller_uid || caller_uid == KUID::none() { // 0 as Kernel/Root override
                    Ok(entry.payload.clone())
                } else {
                    Err(KeyringError::AccessDenied)
                }
            }
            None => Err(KeyringError::NotFound),
        }
    }

    /// Explicitly wipes and removes a credential
    pub fn delete_secret(
        &mut self,
        service: &[u8],
        name: &[u8],
        caller_uid: KUID
    ) -> Result<(), KeyringError> {
        let key = (service.to_vec(), name.to_vec());

        if let Some(entry) = self.store.get(&key) {
            if entry.owner_uid != caller_uid && caller_uid != KUID::none() {
                return Err(KeyringError::AccessDenied);
            }

            // Explicitly zero-out or clear the vector before dropping for security
            if let Some(mut removed) = self.store.remove(&key) {
                for byte in removed.payload.iter_mut() {
                    *byte = 0;
                }
            }
            return Ok(());
        }
        Err(KeyringError::NotFound)
    }
}