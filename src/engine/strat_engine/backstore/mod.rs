// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[allow(clippy::module_inception)]
mod backstore;
mod blockdev;
mod blockdevmgr;
mod cache_tier;
mod crypt;
mod data_tier;
mod devices;
mod range_alloc;
mod shared;
mod transaction;

#[cfg(test)]
pub use self::crypt::crypt_metadata_size;
pub use self::{
    backstore::Backstore,
    blockdev::{StratBlockDev, UnderlyingDevice},
    crypt::{CryptActivationHandle, CryptHandle, CryptMetadataHandle, CLEVIS_TANG_TRUST_URL},
    devices::{initialize_devices, process_and_verify_devices},
};
