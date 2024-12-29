use mpl_core::instructions::CreateV1Builder;
use solana_client::{nonblocking::rpc_client, rpc_client::RpcClient};
use solana_sdk::{signature::{read_keypair_file, Keypair}, signer::Signer, transaction::Transaction};

fn main() {
    let rpc_client = RpcClient::new("https://api.devnet.solana.com".to_string());

    let keypair_path = "/Users/ytakahashi/.config/solana/id.json";

    let keypair = read_keypair_file(keypair_path).unwrap();

    let asset = Keypair::new();

    let create_asset_instruction = CreateV1Builder::new()
        .asset(asset.pubkey())
        .payer(keypair.pubkey())
        .name("test".to_string())
        .uri("ttes".to_string())
        .instruction();
    
    let signers = vec![&keypair, &asset];

    let last_blockhash = rpc_client.get_latest_blockhash().unwrap();

    let create_asset_transaction = Transaction::new_signed_with_payer(
        &[create_asset_instruction],
        Some(&keypair.pubkey()),
        &signers,
        last_blockhash
    );

    let res = rpc_client.send_and_confirm_transaction(&create_asset_transaction).unwrap();

    println!("siggnature is {:?}", res);






}