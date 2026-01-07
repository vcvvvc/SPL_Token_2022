use dotenv::dotenv;
use std::env;

use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::{
    signature::{read_keypair_file, Keypair},
    signer::Signer,
};

pub fn get_client() -> RpcClient {
    dotenv().ok();
    let rpc_url = env::var("RPC_URL").expect("RPC_URL must be set in .env file");
    RpcClient::new_with_commitment(
        rpc_url,
        CommitmentConfig::confirmed(),
    )
}

// 获取你的钱包 (Payer)
pub fn get_payer() -> Keypair {
    dotenv().ok();
    let wallet_path = env::var("WALLET_PATH").expect("WALLET_PATH must be set in .env file");
    println!("正在尝试从 {} 读取钱包...", wallet_path);

    let payer = read_keypair_file(&wallet_path)
        .expect(&format!("❌ 找不到钱包文件: {}\n请检查路径是否正确！", wallet_path));

    println!("✅ 成功加载钱包: {}", payer.pubkey());

    payer
}