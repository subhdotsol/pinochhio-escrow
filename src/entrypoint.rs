use pinocchio::{AccountView, Address, ProgramResult, entrypoint};

use crate::processor;

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Address,
    accounts: &[AccountView],
    data: &[u8],
) -> ProgramResult {
    processor::process_instruction(program_id, accounts, data)
}
