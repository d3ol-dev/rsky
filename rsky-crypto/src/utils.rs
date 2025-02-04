use crate::constants::{BASE58_MULTIBASE_PREFIX, DID_KEY_PREFIX};
use anyhow::{bail, Result};
use multibase::decode;

pub fn extract_multikey(did: &String) -> Result<String> {
    if !did.starts_with(DID_KEY_PREFIX) {
        bail!("Incorrect prefix for did:key: {did}")
    }
    Ok(did[DID_KEY_PREFIX.len()..].to_string())
}

pub fn extract_prefixed_bytes(multikey: String) -> Result<Vec<u8>> {
    if !multikey.starts_with(BASE58_MULTIBASE_PREFIX) {
        bail!("Incorrect prefix for multikey: {multikey}")
    }
    let (_base, bytes) = decode(&multikey)?;
    Ok(bytes)
}

pub fn has_prefix(bytes: &Vec<u8>, prefix: &Vec<u8>) -> bool {
    *prefix == bytes[0..prefix.len()]
}
