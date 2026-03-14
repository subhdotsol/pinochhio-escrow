use pinocchio::{AccountView, Address};

#[repr(C)]
pub struct Escrow {
    pub maker: Address,
    pub mint_x: Address,
    pub mint_y: Address,
    pub amount: u64,
    pub bump: u8,
}

impl Escrow {
    pub const SIZE: usize = 32 + 32 + 32 + 8 + 1;

    pub fn from_account_view_unchecked(account_view: &AccountView) -> &mut Self {
        unsafe { &mut *(account_view.data_ptr() as *mut Self) }
    }

    pub fn from_account_view(account_view: &AccountView) -> &mut Self {
        unsafe {
            assert_eq!(account_view.data_len(), Escrow::SIZE);
            &mut *(account_view.data_ptr() as *mut Self)
        }
    }
}

