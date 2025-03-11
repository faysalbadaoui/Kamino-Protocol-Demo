use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_pack::Pack;
use anchor_lang::solana_program::system_program;
use kamino_cpi_program::{self, instruction::*, state::*};
use kamino_demo::{self, instruction::*};
use solana_program_test::*;
use solana_sdk::{signer::Signer, transaction::Transaction};

#[tokio::test]
async fn test_deposit() {
    let program_test = ProgramTest::new(
        "kamino_demo",
        kamino_demo::ID,
        processor!(kamino_demo::entry),
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Define accounts needed for deposit
    let user = payer.pubkey();
    let strategy = Pubkey::new_unique();
    let vault = Pubkey::new_unique();
    let kamino_program = kamino_cpi_program::ID;

    let deposit_instruction = kamino_demo::instruction::Deposit {
        amount: 1_000_000, // 1 token
    };

    let accounts = kamino_demo::accounts::Deposit {
        user,
        strategy,
        vault,
        kamino_program,
        system_program: system_program::ID,
    };

    let ix = Instruction::new_with_borsh(
        kamino_demo::ID,
        &deposit_instruction,
        vec![
            AccountMeta::new(user, true),
            AccountMeta::new(strategy, false),
            AccountMeta::new(vault, false),
            AccountMeta::new(kamino_program, false),
            AccountMeta::new(system_program::ID, false),
        ],
    );

    let mut transaction = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    transaction.sign(&[&payer], recent_blockhash);
    assert!(banks_client.process_transaction(transaction).await.is_ok());
}

#[tokio::test]
async fn test_borrow() {
    let program_test = ProgramTest::new(
        "kamino_demo",
        kamino_demo::ID,
        processor!(kamino_demo::entry),
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let user = payer.pubkey();
    let strategy = Pubkey::new_unique();
    let vault = Pubkey::new_unique();
    let kamino_program = kamino_cpi_program::ID;

    let borrow_instruction = kamino_demo::instruction::Borrow {
        amount: 500_000, // 0.5 token
    };

    let accounts = kamino_demo::accounts::Borrow {
        user,
        strategy,
        vault,
        kamino_program,
        system_program: system_program::ID,
    };

    let ix = Instruction::new_with_borsh(
        kamino_demo::ID,
        &borrow_instruction,
        vec![
            AccountMeta::new(user, true),
            AccountMeta::new(strategy, false),
            AccountMeta::new(vault, false),
            AccountMeta::new(kamino_program, false),
            AccountMeta::new(system_program::ID, false),
        ],
    );

    let mut transaction = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    transaction.sign(&[&payer], recent_blockhash);
    assert!(banks_client.process_transaction(transaction).await.is_ok());
}
