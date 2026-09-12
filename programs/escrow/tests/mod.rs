use anchor_lang::{system_program, AccountDeserialize, InstructionData, Key, ToAccountMetas};
use anchor_spl::{associated_token, token::{accessor::mint, Mint}};
use escrow::{msg, ESCROW_SEED};

use litesvm::LiteSVM;
use litesvm_token::{spl_token, CreateAssociatedTokenAccount, CreateMint, MintTo};
use solana_keypair::Keypair;
use solana_message::{Instruction, Message};

use solana_pubkey::Pubkey;
use solana_signer::Signer;
use solana_transaction::Transaction;

// testing make instruction

//accounts
/*
maker
mint_a
mint_b
maker_ata_a
escrow_state
escrow_vault
token_program
system_program
 */

 // data
 /*
 amount_a
 amount_b
 seed
  */

// program
// HS7TLs5KSoFwnQCU6xuMtuxZNUdQeYkBUdjhiK45WkGm

#[test]
fn test() {
    let program_id = escrow::id();

    let payer = Keypair::new();

    let mut svm = LiteSVM::new();

    let bytes = include_bytes!(concat!(
        env!("CARGO_TARGET_TMPDIR"),
        "/../deploy/escrow.so"
    ));

    svm.add_program(program_id, bytes).unwrap();

    svm.airdrop(&payer.pubkey(), 10_000_000_000).unwrap();

    // accounts
    let maker = payer.pubkey();

    let mint_a = CreateMint::new(&mut svm, &payer)
        .decimals(6)
        .authority(&maker)
        .send()
        .unwrap();

    msg!("Mint A: {}\n", mint_a);

    let mint_b = CreateMint::new(&mut svm, &payer)
        .decimals(6)
        .authority(&maker)
        .send()
        .unwrap();

    msg!("Mint B: {}\n", mint_b);

    let maker_ata_a = CreateAssociatedTokenAccount(&mut svm, &payer, &mint_a)
        .owner(&maker)
        .send()
        .unwrap();

    let (escrow_state, ) = Pubkey::find_program_address(&[b"escrow", maker.key().as_ref(), &123u64.to_le_bytes()], &program_id);
    let escrow_vault = associated_token::get_associated_token_address(&escrow_state, &mint_a);

    println!("escrow_vault: {escrow_vault}");

    MintTo::new(&mut svm,&maker, &mint_a, &maker_ata_a, 10_000_000_000)
        .send()
        .unwrap();

    let make_ix = Instruction{
        program_id: escrow.id(),
        accounts: escrow::accounts::Make {
            maker: maker,
            mint_a: mint_a,
            mint_b: mint_b,
            maker_ata_a: maker_ata_a,
            escrow_state: escrow_state,
            escrow_vault: escrow_vault,
            token_program: spl_token::id(),
            system_program: system_program::ID
        }.to_account_metas(None),
        data: escrow::instruction::Make{
            amount_a: 1_000_000_000,
            amount_b: 2_000_000_000,
            seed: &123u64
        }
        .data(),
    };

    let message = Message::new(&[make_ix], Some(&maker.pubkey()));
    let recent_blockhash = svm.latest_blockhash();
    let transaction = Transaction::new(&[payer], message, recent_blockhash);

    let tx = svm.send_transaction(transaction).unwrap();

    println!("Make tx sent\ncu consumed: {}\ntx sig: {}\ntx logs: {}",
    tx.compute_units_consumed, tx.signature, tx.pretty_logs());

    
    // test
    let escrow_account = svm.get_account(&escrow_vault).unwrap();
    let escrow_data = escrow::state::EscrowState::try_deserialize(&mut escrow_account.data.as_ref()).unwrap();

    assert_eq!(escrow_data.amount_a, 1_000_000_000, "testing amount of token A deposited by maker into the vault {} and {}", escrow_data.amount_a, 1_000_000_000);
    assert_eq!(escrow_data.seed, 123u64);
    assert_eq!(escrow_data.maker, maker);
    assert_eq!(escrow_data.mint_a, mint_a);
    assert_eq!(escrow_data.mint_b, mint_b);
    assert_eq!(escrow_data.amount_b, 2_000_000_000);

}