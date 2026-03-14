use pinocchio::{
    AccountView, Address, ProgramResult, error::ProgramError,
    cpi::{Seed, Signer},
};
use pinocchio_log::log;
use pinocchio_token::{instructions::Transfer, instructions::CloseAccount, state::TokenAccount};

use crate::state::Escrow;

/// # Refund Instruction
/// 
/// This function allows the original maker to reclaim their tokens from the escrow
/// if they change their mind before someone takes the trade.
///
/// ## Business Logic:
/// 1. Only the original maker who created the escrow can refund
/// 2. All tokens in the vault are returned to the maker's account
/// 3. The vault and escrow accounts are closed, and rent is reclaimed
///
/// ## Accounts expected:
/// 0. `[signer]` maker - The original creator of the escrow
/// 1. `[]` mint_a - The mint of the token the maker initially deposited
/// 2. `[mut]` maker_ata_a - The maker's associated token account for mint_a
/// 3. `[mut]` escrow - The escrow account holding the trade data
/// 4. `[mut]` vault - The token account holding the locked tokens
/// 5. `[]` token_program - SPL Token program
/// 6. `[]` system_program - System program
pub fn refund(
    _program_id: &Address,
    accounts: &[AccountView],
    _data: &[u8],
) -> ProgramResult {
    let [
        maker, 
        mint_a, 
        maker_ata_a, 
        escrow, 
        vault, 
        _token_program, 
        _system_program,
        _remaining @..
    ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Ensure the maker is a signer, this prevents unauthorized refunds
    if !maker.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    unsafe {
        // Get the escrow state from the escrow account
        let escrow_account = Escrow::from_account_view_unchecked(escrow);
        
        // Validate that the escrow belongs to this maker and the mint is correct
        // This ensures we're refunding the correct escrow and tokens
        assert_eq!(escrow_account.maker, *maker.address());
        assert_eq!(escrow_account.mint_x, *mint_a.address());

        // Load the vault account to access token balance and verify ownership
        // In pinocchio-token, TokenAccount::from_account_view_unchecked is the correct way
        let vault_account = TokenAccount::from_account_view_unchecked(vault).unwrap();
        
        // Verify that the vault is owned by the escrow PDA
        // This ensures we're operating on the correct vault associated with this escrow
        assert_eq!(vault_account.owner(), escrow.address());

        // Prepare the PDA seeds needed for signing operations
        // The escrow account is a PDA (Program Derived Address) that can sign for transactions
        let bump = [escrow_account.bump.to_le()];
        let seed = [Seed::from(b"escrow"), Seed::from(maker.address().as_ref()), Seed::from(&bump)];
        let signers = Signer::from(&seed);

        log!("Refunding tokens to maker");

        // Transfer all tokens from the vault back to the maker's token account
        // The escrow PDA signs this transaction using the computed seeds
        Transfer {
            from: vault,
            to: maker_ata_a,
            authority: escrow,
            amount: vault_account.amount(),
        }.invoke_signed(&[signers.clone()])?;

        // Close the vault account and reclaim its rent
        // The funds are sent to the maker as they paid for the account creation
        CloseAccount {
            account: vault,
            destination: maker,
            authority: escrow,
            // Signed with the escrow PDA authority
        }.invoke_signed(&[signers])?;

        // Manually transfer the escrow account's lamports to the maker
        // This effectively closes the escrow account and returns rent
        // For pinocchio, lamports are manipulated via deref or pointer
        let maker_lamports = maker.lamports();
        let escrow_lamports = escrow.lamports();
        
        maker.set_lamports(maker_lamports + escrow_lamports);
        escrow.set_lamports(0);
    }

    // All operations were successful, complete the refund process
    Ok(())
}
