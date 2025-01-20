use crate::event;
use crate::storage_types::DataKey;
use axelar_gas_service::AxelarGasServiceClient;
use axelar_gateway::AxelarGatewayMessagingClient;
use axelar_soroban_std::types::Token;
use soroban_sdk::{contract, contractimpl, Address, Bytes, Env, String};

use crate::abi::abi_encode;
use axelar_gateway::executable::AxelarExecutableInterface;

#[contract]
pub struct AxelarGMP;

#[contractimpl]
impl AxelarExecutableInterface for AxelarGMP {
    fn gateway(env: &Env) -> Address {
        env.storage().instance().get(&DataKey::Gateway).unwrap()
    }

    fn execute(
        env: Env,
        source_chain: String,
        message_id: String,
        source_address: String,
        payload: Bytes,
    ) {
        let _ = Self::validate_message(&env, &source_chain, &message_id, &source_address, &payload);

        event::executed(&env, source_chain, message_id, source_address, payload);
    }
}

#[contractimpl]
impl AxelarGMP {
    pub fn __constructor(env: Env, gateway: Address, gas_service: Address) {
        env.storage().instance().set(&DataKey::Gateway, &gateway);
        env.storage()
            .instance()
            .set(&DataKey::GasService, &gas_service);
    }

    pub fn gas_service(env: &Env) -> Address {
        env.storage().instance().get(&DataKey::GasService).unwrap()
    }

    pub fn send(
        env: Env,
        caller: Address,
        destination_chain: String,
        destination_address: String,
        message: String,
        gas_token: Token,
    ) {
        let gateway = AxelarGatewayMessagingClient::new(&env, &Self::gateway(&env));
        let gas_service: AxelarGasServiceClient<'_> =
            AxelarGasServiceClient::new(&env, &Self::gas_service(&env));

        caller.require_auth();

        let encoded_msg = abi_encode(&env, message).unwrap();

        gas_service.pay_gas(
            &env.current_contract_address(),
            &destination_chain,
            &destination_address,
            &encoded_msg,
            &caller,
            &gas_token,
            &Bytes::new(&env),
        );

        gateway.call_contract(
            &env.current_contract_address(),
            &destination_chain,
            &destination_address,
            &encoded_msg,
        );
    }
}

// stellar contract deploy --wasm target/wasm32-unknown-unknown/release/axelar_gmp.optimized.wasm --source benTwo --network testnet -- --gateway CBECMRORSIPG4XG4CNZILCH233OXYMLCY4GL3GIO4SURSHTKHDAPEOVM --gas_service CD3KZOLEACWMQSDEQFUJI6ZWC7A7CC7AE7ZFVE4X2DBPYAC6L663GCNN

// stellar contract invoke --network testnet --id CDRTYHZ7HQTER4R5WOXWKS2QJYLLWM5GN4HIXTL3X2JDG3YHCCFG6OTA --source-account benTwo -- send --caller ben --destination_chain '"avalanche-fuji"' --message '"hello from stellar"' --destination_address '"0xEab7407d5E7F51D32a52A2d744f45ca79fc7d40D"' --gas_token '{ "address": "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC", "amount": "10000000000" }'
