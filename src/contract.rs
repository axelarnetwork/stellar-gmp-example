use stellar_axelar_gas_service::AxelarGasServiceClient;
use stellar_axelar_gateway::executable::{AxelarExecutableInterface, CustomAxelarExecutable};
use stellar_axelar_gateway::AxelarGatewayMessagingClient;
use stellar_axelar_std::events::Event;
use stellar_axelar_std::types::Token;
use stellar_axelar_std::{
    contract, contracterror, contractimpl, soroban_sdk, Address, AxelarExecutable,
    Bytes, Env, String,
};

use crate::event::ExecutedEvent;
use crate::interface::AxelarGMPInterface;
use crate::storage::DataKey;
use crate::abi::{abi_decode_string, abi_encode};

#[contract]
#[derive(AxelarExecutable)]
pub struct AxelarGMP;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum AxelarGMPError {
    NotApproved = 1,
    FailedDecoding = 2,
}

impl CustomAxelarExecutable for AxelarGMP {
    type Error = AxelarGMPError;

    fn __gateway(env: &Env) -> Address {
        env.storage().instance().get(&DataKey::Gateway).unwrap()
    }

    fn __execute(
        env: &Env,
        source_chain: String,
        message_id: String,
        source_address: String,
        payload: Bytes,
    ) -> Result<(), Self::Error> {
        let decoded_msg = abi_decode_string(env, payload.clone()).map_err(|_| AxelarGMPError::FailedDecoding)?;
        
        // Store the received message
        env.storage().instance().set(&DataKey::ReceivedMessage, &decoded_msg);
        
        // Emit event
        ExecutedEvent {
            source_chain,
            message_id,
            source_address,
            payload,
        }
        .emit(env);

        Ok(())
    }
}

#[contractimpl]
impl AxelarGMP {
    pub fn __constructor(
        env: &Env,
        gateway: Address,
        gas_service: Address,
    ) {
        env.storage().instance().set(&DataKey::Gateway, &gateway);
        env.storage().instance().set(&DataKey::GasService, &gas_service);
    }
}

#[contractimpl]
impl AxelarGMPInterface for AxelarGMP {
    fn gas_service(env: &Env) -> Address {
        env.storage().instance().get(&DataKey::GasService).unwrap()
    }

    fn send(
        env: &Env,
        caller: Address,
        destination_chain: String,
        destination_address: String,
        message: String,
        gas_token: Option<Token>,
    ) {
        let gateway = AxelarGatewayMessagingClient::new(env, &Self::gateway(env));
        let gas_service = AxelarGasServiceClient::new(env, &Self::gas_service(env));

        caller.require_auth();

        let encoded_msg = abi_encode(env, message).unwrap();

        if let Some(gas_token) = gas_token {
            gas_service.pay_gas(
                &env.current_contract_address(),
                &destination_chain,
                &destination_address,
                &encoded_msg,
                &caller,
                &gas_token,
                &Bytes::new(env),
            );
        }

        gateway.call_contract(
            &env.current_contract_address(),
            &destination_chain,
            &destination_address,
            &encoded_msg,
        );
    }
    
    fn received_message(env: &Env) -> String {
        env.storage().instance().get(&DataKey::ReceivedMessage)
            .unwrap_or_else(|| String::from_str(env, ""))
    }
}