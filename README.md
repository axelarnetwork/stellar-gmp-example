# stellar-gmp

0. Setup
   Run `stellar contract build` to compile contract

1. Deploy
   Run `stellar contract deploy --wasm target/wasm32-unknown-unknown/release/axelar_gmp.optimized.wasm --source benTwo --network testnet -- --gateway CBECMRORSIPG4XG4CNZILCH233OXYMLCY4GL3GIO4SURSHTKHDAPEOVM --gas_service CD3KZOLEACWMQSDEQFUJI6ZWC7A7CC7AE7ZFVE4X2DBPYAC6L663GCNN` to deploy on testnet

2. Send gmp msg
   Run `stellar contract invoke --network testnet --id <CONTRACT_ADDRESS> --source-account <MY_ACCOUNT> -- send --caller <CALLER_ADDR_NAME> --destination_chain '"<DEST_CHAIN_NAME>"' --message '"<MESSAGE>"' --destination_address '"<DEST_ADDR>"' --gas_token '{ "address": "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC", "amount": "10000000000" }'` to send a cross-chain gmp message

For Example:
`stellar contract invoke --network testnet --id CDRTYHZ7HQTER4R5WOXWKS2QJYLLWM5GN4HIXTL3X2JDG3YHCCFG6OTA --source-account benTwo -- send --caller ben --destination_chain '"avalanche-fuji"' --message '"hello from stellar"' --destination_address '"0xEab7407d5E7F51D32a52A2d744f45ca79fc7d40D"' --gas_token '{ "address": "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC", "amount": "10000000000" }`

3. To query a received cross-chain message run `stellar contract invoke --network testnet --id <CONTRACT_ADDRESS> --source-account <CALLER_ADDR_NAME> -- received_message`

For example:
`stellar contract invoke --network testnet --id CDSMVKAQMN47F4BS74KVRSTLH42RIP7IDMXAFA6YINGWRYU6RAGK36IO --source-account benTwo -- received_message`.