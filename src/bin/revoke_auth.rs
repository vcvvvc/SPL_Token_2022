/* 销毁Freeze / mint / Metadata Update / Metadata Pointer */
use dotenv::dotenv;
use std::env;
use anyhow::Result;
use std::str::FromStr;

use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
    pubkey::Pubkey,
};

use spl_token_2022_interface::{
    instruction::{
        set_authority,
        AuthorityType,
    },
    ID as TOKEN_2022_PROGRAM_ID
};
use spl_token_metadata_interface::instruction::update_authority;
use spl_pod::optional_keys::OptionalNonZeroPubkey;

use autotoken::{get_payer, get_client};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let token_addr = env::var("TOKEN_ADDRESS").expect("TOKEN_ADDRESS must be set in .env file");

    let client = get_client();
    let fee_payer = get_payer();
    let mint_pubkey = Pubkey::from_str(&token_addr)
        .expect("❌ 地址格式错误，请检查 TOKEN_ADDRESS");
    println!("钱包地址 (Pubkey): {}", fee_payer.pubkey());


    // ===freeze
    let set_freeze_authority = set_authority(
        &TOKEN_2022_PROGRAM_ID,
        &mint_pubkey,
        None,
        AuthorityType::FreezeAccount,
        &fee_payer.pubkey(),
        &[&fee_payer.pubkey()],
    )?;
    let latest_blockhash = client.get_latest_blockhash().await?;

    let set_freeze_transaction = Transaction::new_signed_with_payer(
        &[set_freeze_authority],
        Some(&fee_payer.pubkey()),
        &[&fee_payer],
        latest_blockhash,
    );

    let set_freeze_transaction_signature = client.send_and_confirm_transaction(&set_freeze_transaction).await?;
    println!("set_freeze_transaction_signature: {}", set_freeze_transaction_signature);


    // ===mint
    let set_mint_authority = set_authority(
        &TOKEN_2022_PROGRAM_ID,
        &mint_pubkey,
        None,
        AuthorityType::MintTokens,
        &fee_payer.pubkey(),
        &[&fee_payer.pubkey()],
    )?;

    let latest_blockhash = client.get_latest_blockhash().await?;

    let set_mint_transaction = Transaction::new_signed_with_payer(
        &[set_mint_authority],
        Some(&fee_payer.pubkey()),
        &[&fee_payer],
        latest_blockhash,
    );

    let set_mint_transaction_signature = client.send_and_confirm_transaction(&set_mint_transaction).await?;
    println!("set_mint_transaction_signature: {}", set_mint_transaction_signature);

    // ===Metadata uodate
    let disable_metadata_update = update_authority(
        &TOKEN_2022_PROGRAM_ID,
        &mint_pubkey,
        &fee_payer.pubkey(),
        OptionalNonZeroPubkey::default(),
    );

    let latest_blockhash = client.get_latest_blockhash().await?;

    let disable_meta_update_transaction = Transaction::new_signed_with_payer(
        &[disable_metadata_update],
        Some(&fee_payer.pubkey()),
        &[&fee_payer],
        latest_blockhash,
    );

    let disable_metadata_update_signature = client.send_and_confirm_transaction(&disable_meta_update_transaction).await?;
    println!("disable_metadata_update_signature: {}", disable_metadata_update_signature);

    // ===Metadata Pointer
    let set_metadata_pointer_authority = set_authority(
        &TOKEN_2022_PROGRAM_ID,
        &mint_pubkey,
        None,
        AuthorityType::MetadataPointer,
        &fee_payer.pubkey(),
        &[&fee_payer.pubkey()],
    )?;

    let latest_blockhash = client.get_latest_blockhash().await?;

    let set_metadata_pointer_transaction = Transaction::new_signed_with_payer(
        &[set_metadata_pointer_authority],
        Some(&fee_payer.pubkey()),
        &[&fee_payer],
        latest_blockhash,
    );

    let set_metadata_pointer_signature = client.send_and_confirm_transaction(&set_metadata_pointer_transaction).await?;
    println!("set_metadata_pointer_signature: {}", set_metadata_pointer_signature);


    Ok(())
}