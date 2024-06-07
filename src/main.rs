use solana_client::tpu_client::TpuClient;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::{Signer};
use solana_sdk::transaction::Transaction;
use solana_sdk::commitment_config::CommitmentConfig;
use std::sync::Arc;
use solana_sdk::signature::read_keypair_file;
use solana_sdk::compute_budget::ComputeBudgetInstruction;
use solana_sdk::message::Message;

#[tokio::main]
async fn main() {
    let rpc_url = "https://api.devnet.solana.com";
    let rpc_client = Arc::new(RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed()));

    let keypair_path = "/Users/safakc/.config/solana/id.json";
    let my_keypair = read_keypair_file(&keypair_path).expect("Failed to read keypair file");

    let from_pubkey = my_keypair.pubkey();

    let message = Message::new(
        &[
            ComputeBudgetInstruction::set_compute_unit_limit(1_000),
            ComputeBudgetInstruction::set_compute_unit_price(2_000),
        ],
        Some(&from_pubkey),
    );

    let blockhash = rpc_client.get_latest_blockhash().expect("Failed to get blockhash");
    let mut tx = Transaction::new_unsigned(message);

    tx.sign(&[&my_keypair], blockhash);

    let tpu_ws_url = rpc_url.replace("http", "ws");
    let tpu_client = TpuClient::new(
        rpc_client.clone(),
        &tpu_ws_url,
        solana_client::tpu_client::TpuClientConfig::default(),
    ).expect("Failed to create TPU client");

    let result = tpu_client.send_transaction(&tx);
    if result {
        println!("Transaction sent successfully!");

        println!("Transaction Signature: {:?}", tx.signatures.get(0));
    } else {
        eprintln!("Failed to send transaction via TPU client");
    }
}
