/*
1\ 准备metadata.json
2\ 创建mint账户，开启 Metadata Pointer（元数据指针）扩展。
3\ 关联元数据：把第 2 步拿到的 JSON 链接，写入到 Mint 账户的指针里。
4\ 创建一个 Token Account(ATA)， mint代币
5\ 销毁Freeze / mint / Metadata Update / Metadata Pointer
*/
use anyhow::Result;
use dotenv::dotenv;
use std::env;

use solana_sdk::{
    signature::Keypair,
    signer::Signer,
    transaction::Transaction,
};
use solana_system_interface::instruction::create_account;
use spl_associated_token_account_interface::{
    address::get_associated_token_address, instruction::create_associated_token_account, address::get_associated_token_address_with_program_id, instruction::create_associated_token_account_idempotent
};
use spl_token_2022_interface::{
    extension::{
        metadata_pointer::{
            instruction::initialize as initialize_metadata_pointer,
        },
        ExtensionType,
    },
    instruction::{
        initialize_mint,
        mint_to
    },
    state:: {
        Mint,
        Account,
    },
    ID as TOKEN_2022_PROGRAM_ID
};
use solana_program_pack::Pack;
use spl_token_metadata_interface::{
    instruction::{initialize as initialize_token_metadata, update_field},
    state::{Field, TokenMetadata},
};

use autotoken::{ get_payer, get_client };

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let metadata_uri = env::var("METADATA_URI").expect("METADATA_URI must be set in .env file");

    let client = get_client();

    let fee_payer = get_payer();

    // Generate keypair for the mint
    let mint = Keypair::new();

    // Define Token metadata
    let token_metadata  = TokenMetadata {
        update_authority: Some(fee_payer.pubkey()).try_into()?,
        mint: mint.pubkey(),
        name: "Dos".to_string(),
        symbol : "DOS".to_string(),
        uri : metadata_uri,
        additional_metadata: vec![("description".to_string(),"Dos on Solana.".to_string())]
    };

    // Calculate space for mint with metadata pointer and token metadata extensions
    let mint_space =
        ExtensionType::try_calculate_account_len::<Mint>(&[ExtensionType::MetadataPointer])?;

    let metadata_len = token_metadata.tlv_size_of()?;

    let mint_rent = client
        .get_minimum_balance_for_rent_exemption(mint_space + metadata_len)
        .await?;

    // === token mint
    // Create mint account instruction
    let create_mint_account_instruction = create_account(
        &fee_payer.pubkey(),
        &mint.pubkey(),
        mint_rent,
        mint_space as u64,
        &TOKEN_2022_PROGRAM_ID,
    );

    // Instruction to initialize metadata pointer (pointing to itself for self-managed metadata)
    //确定指针指向
    let initialize_metadata_pointer_instruction = initialize_metadata_pointer(
        &TOKEN_2022_PROGRAM_ID,
        &mint.pubkey(),
        Some(fee_payer.pubkey()), // authority
        Some(mint.pubkey()),      // metadata address (pointing to self)
    )?;

    // Instruction to initialize mint account data
    //代币授权 / 9位小数
    let initialize_mint_instruction = initialize_mint(
        &TOKEN_2022_PROGRAM_ID,    // program id
        &mint.pubkey(),            // mint
        &fee_payer.pubkey(),       // mint authority
        Some(&fee_payer.pubkey()), // freeze authority
        9,                         // decimals
    )?;

    // Instruction to initialize token metadata
    //  确定metadata
    let initialize_metadata_instruction = initialize_token_metadata(
        &TOKEN_2022_PROGRAM_ID,            // program id
        &mint.pubkey(),                    //metadata
        &fee_payer.pubkey(),               // update authority
        &mint.pubkey(),                    // mint
        &fee_payer.pubkey(),               // mint authority
        token_metadata.name.to_string(),   // name
        token_metadata.symbol.to_string(), // symbol
        token_metadata.uri.to_string(),    // uri
    );

    // Create update field instructions from token_metadata.additional_metadata
    // Additional metadata must be initialized separately using the update_field instruction
    // If the field already exists, it will be updated instead of creating a new field
    let update_field_instructions: Vec<_> = token_metadata
        .additional_metadata
        .iter()
        .map(|(key, value)| {
            update_field(
                &TOKEN_2022_PROGRAM_ID,
                &mint.pubkey(),
                &fee_payer.pubkey(),
                Field::Key(key.clone()),
                value.clone(),
            )
        })
        .collect();

    // Construct transaction with all instructions
    let mut instructions = vec![
        create_mint_account_instruction,
        initialize_metadata_pointer_instruction,
        initialize_mint_instruction,
        initialize_metadata_instruction,
    ];
    instructions.extend(update_field_instructions);

    let latest_blockhash = client.get_latest_blockhash().await?;
    let create_token_transaction = Transaction::new_signed_with_payer(
        &instructions,
        Some(&fee_payer.pubkey()),
        &[&fee_payer, &mint],
        latest_blockhash,
    );

    let mint_transaction_signature = client.send_and_confirm_transaction(&create_token_transaction).await?;

    println!("Mint Address: {}", mint.pubkey());
    println!("Transaction Signature: {}", mint_transaction_signature);

    // ==== token account - ATA
    let ata_token_address = get_associated_token_address_with_program_id(
        &fee_payer.pubkey(),
        &mint.pubkey(),
        &TOKEN_2022_PROGRAM_ID,
    );

    println!(" ATA address: {}", ata_token_address);

    let ata_token_account = create_associated_token_account_idempotent(
        &fee_payer.pubkey(),
        &fee_payer.pubkey(),
        &mint.pubkey(),
        &TOKEN_2022_PROGRAM_ID,
    );

    let latest_blockhash = client.get_latest_blockhash().await?;
    let ata_transaction = Transaction::new_signed_with_payer(
        &[ata_token_account],
        Some(&fee_payer.pubkey()),
        &[&fee_payer],
        latest_blockhash,
    );

    let ata_transaction_signature = client.send_and_confirm_transaction(&ata_transaction).await?;
    println!("ATA Transaction Signature: {}", ata_transaction_signature);

    // ==== mint tokens
    let decimals: u8 = 9;
    let amount_to_mint: u64 = 1_000_000_000 * 10u64.pow(decimals as u32);
    let mint_tokens_instruction = mint_to(
        &TOKEN_2022_PROGRAM_ID,
        &mint.pubkey(),
        &ata_token_address,
        &fee_payer.pubkey(),
        &[&fee_payer.pubkey()],    // signer
        amount_to_mint,
    )?;

    let latest_blockhash = client.get_latest_blockhash().await?;
    let mint_token_transaction = Transaction::new_signed_with_payer(
        &[mint_tokens_instruction],
        Some(&fee_payer.pubkey()),
        &[&fee_payer], // fee_payer 必須簽名，因為他是 Mint Authority
        latest_blockhash,
    );

    let mint_token_transaction_signature = client.send_and_confirm_transaction(&mint_token_transaction).await?;

    let mint_account = client.get_account(&mint.pubkey()).await?;
    let mint_data = Mint::unpack_from_slice(&mint_account.data)?;

    let token = client.get_account(&ata_token_address).await?;
    let token_data = Account::unpack_from_slice(&token.data)?;

    println!("Minted tokens to the associated token account");
    println!("\nMint Address: {}", mint.pubkey());
    println!("{:#?}", mint_data);

    println!(
        "\nAssociated Token Account Address: {}",
        ata_token_address
    );
    println!("{:#?}", token_data);

    println!("Transaction Signature: {}", mint_token_transaction_signature);

    Ok(())
}
