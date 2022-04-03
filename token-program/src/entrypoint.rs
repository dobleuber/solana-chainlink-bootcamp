use crate::processor::Processor;

use solana_program::{
    account_info::{AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Create a new processor
    msg!(
        "process_instruction: {}: {} accounts, data={:?}",
        _program_id,
        accounts.len(),
        instruction_data
    );

    Processor::process_instruction(_program_id, accounts, instruction_data)
}