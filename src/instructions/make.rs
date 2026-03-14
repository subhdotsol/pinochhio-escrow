use pinocchio::{
    AccountView, Address, ProgramResult,
    cpi::{Seed, Signer},
    error::ProgramError,
    sysvars::{Sysvar, rent::Rent},
};
use pinocchio_log::log;
use pinocchio_system::instructions::CreateAccount;
use pinocchio_token::{instructions::Transfer, state::TokenAccount};

use crate::state::Escrow;

/// # Make Instruction
///
/// This function creates a new escrow for token exchange between two parties.
/// The maker creates an offer to trade their token X for someone else's token Y.
///
/// ## Business Logic:
/// 1. Maker offers token X in exchange for token Y
/// 2. Tokens are locked in a vault until someone takes the trade
/// 3. Escrow account stores the trade parameters and ownership information
///
/// ## Accounts expected:
/// 0. `[signer]` maker - The account initiating the escrow trade
/// 1. `[]` mint_x - The mint of the token being offered
/// 2. `[]` mint_y - The mint of the token requested in exchange
/// 3. `[mut]` maker_ata - Maker's associated token account for mint_x
/// 4. `[mut]` vault - Token account to temporarily hold the offered tokens
/// 5. `[mut]` escrow - Account to store the escrow state data
/// 6. `[]` system_program - System program for account creation
/// 7. `[]` token_program - SPL Token program for token operations
///
/// ## Data parameters:
/// 0. [u8; 1] - Bump seed for PDA derivation
/// 1. [u64; 1] - Amount of token_y the maker wants to receive
/// 9. [u64; 1] - Amount of token_x the maker is offering
pub fn make(program_id: &Address, accounts: &[AccountView], data: &[u8]) -> ProgramResult {
    // Unpack the required accounts from the accounts array
    let [
        maker,
        mint_x,
        mint_y,
        maker_ata,
        vault,
        escrow,
        _system_program,
        _token_program,
        _remaining @ ..,
    ] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if data.len() < 17 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Extract the bump seed from instruction data and prepare seeds for PDA validation
    let bump = unsafe { *(data.as_ptr() as *const u8) }.to_le_bytes();
    let seed = [(b"escrow"), maker.address().as_ref(), bump.as_ref()];
    #[allow(unused_variables)]
    let seeds = &seed[..];

    // Derive the expected PDA and verify it matches the provided escrow account
    #[cfg(any(target_os = "solana", target_arch = "bpf", feature = "curve25519"))]
    let pda = Address::create_program_address(seeds, program_id)
        .map_err(|_| ProgramError::InvalidSeeds)?;
    #[cfg(not(any(target_os = "solana", target_arch = "bpf", feature = "curve25519")))]
    let pda = escrow.address().clone(); // Dummy for cargo check on host without curve25519

    assert_eq!(&pda, escrow.address());

    if escrow.is_data_empty() {
        unsafe {
            // Verify that the provided mint accounts are legitimate SPL token mints
            assert_eq!(mint_x.owner(), &pinocchio_token::ID);
            assert_eq!(mint_y.owner(), &pinocchio_token::ID);

            // Verify that the vault is owned by the escrow account (for later token operations)
            assert!(
                TokenAccount::from_account_view_unchecked(vault)
                    .unwrap()
                    .owner()
                    == escrow.address()
            );

            // Check if the escrow account needs to be created (first-time initialization)
            if escrow.owner() != program_id {
                log!("Creating Escrow Account");
                let seed = [
                    Seed::from(b"escrow"),
                    Seed::from(maker.address().as_ref()),
                    Seed::from(&bump),
                ];
                let signers = Signer::from(&seed);

                // Create the escrow account with enough space for the state data
                CreateAccount {
                    from: maker,
                    to: escrow,
                    lamports: Rent::get()?.try_minimum_balance(Escrow::SIZE)?,
                    space: Escrow::SIZE as u64,
                    owner: program_id,
                }
                .invoke_signed(&[signers.clone()])?;

                // Initialize the escrow data with the trade parameters
                let escrow_account = Escrow::from_account_view_unchecked(escrow);
                escrow_account.maker = maker.address().clone();
                escrow_account.mint_x = mint_x.address().clone();
                escrow_account.mint_y = mint_y.address().clone();
                escrow_account.amount = *(data.as_ptr().add(1) as *const u64); // Amount of token Y to receive
                escrow_account.bump = *data.as_ptr(); // Store bump for future PDA derivation
                let amount = *(data.as_ptr().add(1 + 8) as *const u64); // amount of token X to deposit in the vault

                log!("Amount to deposit: {}", amount);

                // Transfer the offered tokens from maker's account to the vault
                Transfer {
                    from: maker_ata,
                    to: vault,
                    authority: maker,
                    amount, // Amount of token X to deposit
                }
                .invoke()?;
            } else {
                return Err(ProgramError::AccountAlreadyInitialized);
            }
        }
    }

    // Escrow successfully created and tokens deposited
    Ok(())
}
