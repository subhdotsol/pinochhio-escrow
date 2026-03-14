use pinocchio::{AccountView, Address, ProgramResult};
use crate::instructions::EscrowInstruction;

pub fn process_instruction(
    program_id: &Address,
    accounts: &[AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = EscrowInstruction::unpack(instruction_data)?;

    match instruction {
        EscrowInstruction::Make { amount_a, amount_b } => {
            crate::instructions::make::make(program_id, accounts, amount_a, amount_b)
        }
        EscrowInstruction::Take { amount_a, amount_b } => {
            crate::instructions::take::take(program_id, accounts, amount_a, amount_b)
        }
    }
}
