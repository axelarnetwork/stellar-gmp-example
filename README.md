# Stellar GMP Example
This repository contains a standalone example of using Axelar's General Message Passing (GMP) protocol to send and receive messages between Stellar and other blockchains. With this contract, you can:

1. Send messages from Stellar to other supported blockchains
2. Receive messages from other blockchains on Stellar
3. Pay gas fees for cross-chain message execution

## Prerequisites

- [Stellar CLI](https://developers.stellar.org/docs/tools/developer-tools) installed
- A Stellar account on testnet with funds ([Get testnet funds](https://faucet.triangleplatform.com/stellar/testnet))
- Rust toolchain with `wasm32-unknown-unknown` target ([Rust installation guide](https://www.rust-lang.org/tools/install))

## Building the Contract

1. Compile the contract:
   ```bash
   stellar contract build
   ```

2. Optimize the compiled WebAssembly for deployment:
   ```bash
   stellar contract optimize --wasm target/wasm32-unknown-unknown/release/axelar_gmp.wasm
   ```

## Deploying the Contract (Testnet)

Deploy the optimized contract to the Stellar testnet with the following command:

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/axelar_gmp.optimized.wasm \
  --source YOUR_ACCOUNT_NAME \
  --network testnet \
  -- \
  --gateway CCSNWHMQSPTW4PS7L32OIMH7Z6NFNCKYZKNFSWRSYX7MK64KHBDZDT5I \
  --gas_service CAZUKAFB5XHZKFZR7B5HIKB6BBMYSZIV3V2VWFTQWKYEMONWK2ZLTZCT
```

Replace `YOUR_ACCOUNT_NAME` with your Stellar account identifier.

> **Note:** The gateway and gas service addresses are specific to the Stellar testnet. They enable the cross-chain messaging functionality through the Axelar Network.

### Key Addresses

These are the important contract addresses for the Stellar testnet:

- **AxelarGateway**: `CCSNWHMQSPTW4PS7L32OIMH7Z6NFNCKYZKNFSWRSYX7MK64KHBDZDT5I`
- **GasService**: `CAZUKAFB5XHZKFZR7B5HIKB6BBMYSZIV3V2VWFTQWKYEMONWK2ZLTZCT`
- **Gas Token**: `CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC`
- **InterchainTokenService**: `CCXT3EAQ7GPQTJWENU62SIFBQ3D4JMNQSB77KRPTGBJ7ZWBYESZQBZRK`

The Stellar test network is identified as `stellar-2025-q1`.

## Sending Cross-Chain Messages

To send a message from Stellar to another blockchain:

```bash
stellar contract invoke \
  --network testnet \
  --id CONTRACT_ADDRESS \
  --source-account YOUR_ACCOUNT_NAME \
  -- \
  send \
  --caller YOUR_ACCOUNT_NAME \
  --destination_chain '"DESTINATION_CHAIN"' \
  --message '"YOUR_MESSAGE"' \
  --destination_address '"DESTINATION_ADDRESS"' \
  --gas_token '{ "address": "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC", "amount": "10000000000" }'
```

### Parameters Explanation:

- `CONTRACT_ADDRESS`: The address of your deployed GMP contract
- `YOUR_ACCOUNT_NAME`: Your Stellar account identifier
- `DESTINATION_CHAIN`: Target blockchain name (e.g., `"flow"`, `"avalanche"`)
- `YOUR_MESSAGE`: The message you want to send cross-chain
- `DESTINATION_ADDRESS`: The recipient address on the destination chain (typically in that chain's format)
- `gas_token`: Token used to pay for execution on the destination chain

### Example:

```bash
stellar contract invoke \
  --network testnet \
  --id CBD3LALJWP2MB33Y2S2ZKWJRZUXLGLRJGASXNKAQDIRJYPYYGVSHSR4A \
  --source-account idris-stellar \
  -- \
  send \
  --caller idris-stellar \
  --destination_chain '"flow"' \
  --message '"hello world from stellar"' \
  --destination_address '"0x98B2920D53612483F91F12Ed7754E51b4A77919e"' \
  --gas_token '{ "address": "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC", "amount": "10000000000" }'
```

> **Important:** Note the double quotes inside single quotes for string parameters. The Stellar CLI requires this.

## Querying Received Messages

To check if your contract has received any cross-chain messages:

```bash
stellar contract invoke \
  --network testnet \
  --id CONTRACT_ADDRESS \
  --source-account YOUR_ACCOUNT_NAME \
  -- \
  received_message
```

This will return the most recent message received by your contract from another blockchain.

## Tracking Cross-Chain Transactions

After sending a message, you can track its progress across chains using the [Axelarscan Testnet Explorer](https://testnet.axelarscan.io/):

1. Copy the transaction hash from the output of your send command
2. Visit https://testnet.axelarscan.io/gmp/YOUR_TRANSACTION_HASH

For example, you can see a successful transaction here: [https://testnet.axelarscan.io/gmp/a465efc33ed0b7848236bd8d75ba8e21d0d9c1fe1ef5a0d8b3112efa5f109828](https://testnet.axelarscan.io/gmp/a465efc33ed0b7848236bd8d75ba8e21d0d9c1fe1ef5a0d8b3112efa5f109828)

## Supported Chains

This example works with all blockchains supported by Axelar Network. You can send messages from Stellar to these destination chains:

- `ethereum-sepolia` (Ethereum Sepolia)
- `avalanche` (Avalanche Fuji Testnet)
- `flow` (Flow Testnet)

For a complete list of supported chains and their contract addresses, check the [Axelar testnet configuration](https://github.com/axelarnetwork/axelar-contract-deployments/blob/main/axelar-chains-config/info/testnet.json).

## Gas Token

For testnet operations, you should use the following token for gas payments:
- Address: `CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC`
- Recommended amount: `10000000000` (adjust as needed for your message)

## Troubleshooting

### Common Issues:

1. **Invalid gas token**: Ensure you use a valid Stellar asset address for gas payments. The correct testnet gas token is `CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC`.

2. **Insufficient balance**: Ensure your account has enough balance to cover transaction fees and has trust lines set up for the required assets.

3. **Quotes format**: String parameters must be enclosed in double quotes, then single quotes: `'"value"'`. The Stellar CLI requires this.

4. **Authorization error**: The caller address must match your source account. The `--caller` parameter should match your `--source-account`.

5. **Contract re-entry errors**: If you see "Contract re-entry is not allowed" errors, it may indicate an issue with how the gas token is being processed. Make sure you're not trying to call back into the same contract.

6. **Cross-chain transaction not showing up**: Cross-chain transactions can take time to propagate through the Axelar network. Check the transaction status on [Axelarscan](https://testnet.axelarscan.io/).

### Debug Steps:

1. Check your Stellar account balance and trustlines using [Stellar Expert](https://stellar.expert/explorer/testnet).

2. Verify that your transaction was submitted successfully on the Stellar side first.

3. For issues with Axelar integration, check the transaction status in Axelarscan.

If you encounter persistent issues, check the [Axelar Discord community](https://discord.gg/axelar) for support or file an issue in this repository.

## How It Works

This implementation consists of several key components:

1. **Contract Structure**:
   - The `AxelarGMP` contract is the interface for sending and receiving messages.
   - It communicates with the Axelar Gateway and Gas Service contracts on the Stellar network.
   - The ABI encoding/decoding functionality handles proper formatting of messages for cross-chain compatibility.

2. **Message Flow**:
   - When sending a message, the contract encodes it using ABI encoding, pays gas, and calls the Axelar Gateway.
   - The Axelar Network relays the message to the destination chain.
   - The Axelar Gateway calls the contract's execute method when receiving a message.

3. **Storage**:
   - The contract stores references to the Axelar Gateway and Gas Service contracts.
   - It also maintains a record of the most recently received message.

## Additional Resources

- [Axelar Documentation](https://docs.axelar.dev/dev/general-message-passing/stellar-gmp/gmp-example/)
- [Stellar Development Documentation](https://developers.stellar.org/docs)
- [Soroban Documentation](https://developers.stellar.org/)
- [Axelar Contract Deployments](https://github.com/axelarnetwork/axelar-contract-deployments)
- [Axelarscan Explorer](https://testnet.axelarscan.io/)

## License

This example is provided under the [MIT License](LICENSE).