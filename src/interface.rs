use stellar_axelar_gateway::executable::AxelarExecutableInterface;
use stellar_axelar_std::types::Token;
use stellar_axelar_std::{Address, Env, String};

pub trait AxelarGMPInterface: AxelarExecutableInterface {
    /// Retrieves the address of the gas service.
    fn gas_service(env: &Env) -> Address;
    
    /// Sends a message to a specified destination chain.
    ///
    /// The function also handles the payment of gas for the cross-chain transaction.
    ///
    /// # Arguments
    /// * `caller` - The address of the caller initiating the message.
    /// * `destination_chain` - The name of the destination chain where the message will be sent.
    /// * `destination_address` - The address on the destination chain where the message will be sent.
    /// * `message` - The message to be sent.
    /// * `gas_token` - An optional gas token used to pay for gas during the transaction.
    ///
    /// # Authorization
    /// - The `caller` must authorize.
    fn send(
        env: &Env,
        caller: Address,
        destination_chain: String,
        destination_address: String,
        message: String,
        gas_token: Option<Token>,
    );
    
    /// Returns the most recently received message.
    fn received_message(env: &Env) -> String;
}