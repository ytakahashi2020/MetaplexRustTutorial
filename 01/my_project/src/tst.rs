use mpl_core::instructions::CreateV1Builder;
use solana_client::rpc_client;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

fn main() {
    let rpc_client = rpc_client::RpcClient::new("https://api.devnet.solana.com".to_string());

    let keypair_path = "/Users/ytakahashi/.config/solana/id.json";
    let keypair = solana_sdk::signature::read_keypair_file(keypair_path).unwrap();
    let asset = Keypair::new();

    let create_asset_ix = CreateV1Builder::new()
        .asset(asset.pubkey())
        .payer(keypair.pubkey())
        .name("My Asset".into())
        .uri("https://example.com/my-asset.json".into())
        .instruction();

    let signers = vec![&asset, &keypair];

    let last_blockhash = rpc_client.get_latest_blockhash().unwrap();

    let create_asset_tx = Transaction::new_signed_with_payer(
        &[create_asset_ix],
        Some(&keypair.pubkey()),
        &signers,
        last_blockhash,
    );

    let res = rpc_client.send_and_confirm_transaction(&create_asset_tx).unwrap();

    println!("Signature: {:?}", res);
}
