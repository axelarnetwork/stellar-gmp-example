use alloy_sol_types::{sol, SolType};
use soroban_sdk::{Bytes, Env, Error, String};

extern crate alloc;

// Define a Solidity-compatible `bytes` type
//type SolidityBytes = sol!(bytes);
sol!(bytes EncodedMessage);

impl Message {
    pub fn abi_encode(self, env: &Env) -> Result<Bytes, Error> {
        // ABI encode the byte slice
        let encoded = EncodedMessage.abi_encode();

        // Return as Soroban's `Bytes` type
        Ok(Bytes::from_slice(env, &encoded))
    }
}
