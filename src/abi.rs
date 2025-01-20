use crate::abi::alloc::{string::String as StdString, vec};
use alloy_sol_types::{sol_data, SolType};
use soroban_sdk::{contracterror, Bytes, Env, String};

extern crate alloc;

#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AbiError {
    InvalidUtf8 = 1,
}

pub fn abi_encode(env: &Env, message: String) -> Result<Bytes, AbiError> {
    let message = to_std_string(message)?;
    let encoded = sol_data::String::abi_encode(&message);

    Ok(Bytes::from_slice(&env, &encoded))
}
// soroban string to std string
fn to_std_string(soroban_string: String) -> Result<StdString, AbiError> {
    let length = soroban_string.len() as usize;
    let mut bytes = vec![0u8; length];

    soroban_string.copy_into_slice(&mut bytes);
    StdString::from_utf8(bytes).map_err(|_| AbiError::InvalidUtf8)
}
