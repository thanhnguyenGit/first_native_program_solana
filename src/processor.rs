use crate::{instructions::transfer_sol, instructions::transfer_sol_with_cpi};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum TransferSOLInstructions {
    TransferWithCPI(u64),
    Transfer(u64),
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = TransferSOLInstructions::try_from_slice(instruction_data)?;
    match instruction {
        TransferSOLInstructions::Transfer(amount) => transfer_sol(program_id, accounts, amount),
        TransferSOLInstructions::TransferWithCPI(amount) => transfer_sol_with_cpi(accounts, amount),
    }
}
