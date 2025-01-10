use rust_task::programs::Turbin3_prereq::{TurbinePrereqProgram, CompleteArgs};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{read_keypair_file, Keypair, Signer},
    system_program
};

fn main() {
    const RPC_URL: &str = "https://api.devnet.solana.com";

    let rpc_client = RpcClient::new(RPC_URL);

    let signer = read_keypair_file("src/Turbin3-wallet.json").expect("Couldn't find wallet file");

    let prereq = TurbinePrereqProgram::derive_program_address(&[b"prereq",
        signer.pubkey().to_bytes().as_ref()]);

    let args = CompleteArgs {
        github: b"0xnirlin".to_vec()
    };

    let blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");

    let transaction = TurbinePrereqProgram::complete(
        &[&signer.pubkey(), &prereq, &system_program::id()],
        &args,
        Some(&signer.pubkey()),
        &[&signer],
        blockhash
    );

    let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");

    println!("Success! Check out your TX here:\nhttps://explorer.solana.com/tx/{}/?cluster=devnet", signature);
}
