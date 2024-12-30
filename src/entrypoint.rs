use crate::processor::process_instruction;
pub use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};
entrypoint!(process_instruction);
