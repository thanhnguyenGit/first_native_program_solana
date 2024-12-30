use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    program::invoke,
    pubkey::Pubkey,
    system_instruction,
};

use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum TransferSOLInstructions {
    TransferWithCPI,
    Transfer,
}

pub fn transfer_sol_with_cpi(accounts: &[AccountInfo], amounts: u64) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let recepient = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    // invoke a cross-program invokation
    invoke(
        &system_instruction::transfer(payer.key, recepient.key, amounts),
        &[payer.clone(), recepient.clone(), system_program.clone()],
    )?;
    Ok(())
}

pub fn transfer_sol(_program_id: &Pubkey, accounts: &[AccountInfo], amounts: u64) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let recipient = next_account_info(accounts_iter)?;

    let mut payer_lamport = payer.try_borrow_mut_lamports()?;
    **payer_lamport = payer_lamport.checked_sub(amounts).expect("Overflow");

    let mut recipient_lamport = recipient.try_borrow_mut_lamports()?;
    **recipient_lamport = recipient_lamport.checked_add(amounts).expect("Overflow");

    Ok(())
}

