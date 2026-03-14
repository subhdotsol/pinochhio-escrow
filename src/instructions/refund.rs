use pinocchio::{AccountView, Address, ProgramResult};

pub fn refund(
    _program_id: &Address,
    _accounts: &[AccountView],
    _data: &[u8],
) -> ProgramResult {
    Ok(())
}
