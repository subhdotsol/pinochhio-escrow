// How to do it in anchor

// pub mod make; 
// pub mod take; 

// pub use make::*; 
// pub use take::*; 

pub mod make; 
pub mod take; 

use pinocchio::error::ProgramError; 

pub enum EscrowInstruction {
    Make {
        amount_a: u64,
        amount_b: u64,
    },
    Take {
        amount_a: u64,
        amount_b: u64,
    },
}

impl EscrowInstruction {
    pub fn unpack(data: &[u8]) -> Result<Self, ProgramError> {
        let (instruction, rest) = data.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        match instruction {
            0 => Ok(Self::Make {
                amount_a: u64::from_le_bytes(rest[0..8].try_into().map_err(|_| ProgramError::InvalidInstructionData)?),
                amount_b: u64::from_le_bytes(rest[8..16].try_into().map_err(|_| ProgramError::InvalidInstructionData)?),
            }),
            1 => Ok(Self::Take {
                amount_a: u64::from_le_bytes(rest[0..8].try_into().map_err(|_| ProgramError::InvalidInstructionData)?),
                amount_b: u64::from_le_bytes(rest[8..16].try_into().map_err(|_| ProgramError::InvalidInstructionData)?),
            }),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}