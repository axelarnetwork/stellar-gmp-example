use crate::event;
use crate::storage_types::DataKey;
use axelar_gas_service::AxelarGasServiceClient;
use axelar_gateway::AxelarGatewayMessagingClient;
use axelar_soroban_std::types::Token;
use soroban_sdk::{contract, contractimpl, Address, Bytes, Env, String};

use crate::abi::{abi_decode_string, abi_encode};
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

        let decoded_msg = abi_decode_string(&env, payload);

        //store msg
        env.storage()
            .instance()
            .set(&DataKey::ReceivedMessage, &decoded_msg);

        // event::executed(&env, source_chain, message_id, source_address, decoded_msg);
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

    pub fn received_message(env: Env) -> String {
        env.storage()
            .instance()
            .get(&DataKey::ReceivedMessage)
            .unwrap_or_else(|| String::from_str(&env, ""))
    }
}
