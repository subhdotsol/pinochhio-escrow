use pinocchio::{AccountView, Address, ProgramResult};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Address,
        accounts: &mut [AccountView],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = EscrowInstruction::unpack(data)?;

        instruction.process(program_id, accounts)
    }
}
