//! Program state processor
use crate::{
    error::OneSolError,
};
use num_traits::FromPrimitive;
use solana_program::{
    entrypoint::ProgramResult,
    msg, pubkey::Pubkey, account_info::AccountInfo,
    program_error::{PrintProgramError},
    decode_error::DecodeError,
};

/// Program state handler.
pub struct Processor {}
impl Processor {
    /// Processes an [Instruction](enum.Instruction.html).
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
        msg!("good");
        Ok(())
    }
}

impl PrintProgramError for OneSolError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        match self {
            OneSolError::Unknown => msg!("Error: Unknown")
        }
    }
}