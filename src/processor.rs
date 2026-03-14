use crate::instructions::EscrowInstruction;
use pinocchio::{AccountView, Address, ProgramResult, error::ProgramError};

pub fn process_instruction(
    program_id: &Address,
    accounts: &[AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    let (discriminator, data) = instruction_data.split_first().ok_or(ProgramError::InvalidInstructionData)?;
    let instruction = EscrowInstruction::try_from(*discriminator)?;

    match instruction {
        EscrowInstruction::Make => crate::instructions::make::make(program_id, accounts, data),
        EscrowInstruction::Take => crate::instructions::take::take(program_id, accounts, data),
        EscrowInstruction::Refund => crate::instructions::refund::refund(program_id, accounts, data),
    }
}
