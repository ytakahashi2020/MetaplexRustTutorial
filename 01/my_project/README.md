### 1 create a new project

```
cargo new my_project
cd my_project
```

### 2 run the code

`cargo run`

### 3 add dependencies

`cargo add mpl-core@0.8.1-beta.1`

`cargo add solana-client solana-sdk`

`cargo add mpl-core@0.8.1-beta.1`

### 4 create a rpc client

rpc_client::RpcClient::new()
from solana_client::rpc_clilent

### 5 create a keypair

solana_sdk::signature::read_keypair_file

### 6 create an asset keypair

Keypair::new() from solana_sdk::signature

### 7 create an instruction

CreateV1Builder::new() from mpl_core::instructions

- asset
- payer
- name
- uri
- instruction

### 8 create signers

vec

### 9 get a last blockhash

rpc_client.get_latest_blockhash()

### 10 create a transaction

Transaction::new_signed_with_payer from solana_sdk

### 11 send and confirm transaction

rpc_client.send_and_confirm_transaction
