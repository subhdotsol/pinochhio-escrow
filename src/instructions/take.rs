use pinocchio::{
    AccountView, Address, ProgramResult,
};

pub fn take(
    _program_id: &Address,
    _accounts: &[AccountView],
    _amount_a: u64,
    _amount_b: u64,
) -> ProgramResult {
    Ok(())
}
