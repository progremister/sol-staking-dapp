//! # Staking Error Module
//!
//! This module defines custom errors for a Solana staking pool program.
//! These errors provide more descriptive and precise feedback for common
//! issues encountered during staking pool operations.
//!
//! ## Purpose
//! Custom error definitions enable:
//! - Clear and detailed debugging for program logic failures.
//! - Efficient error handling using `ProgramError` conversions.
//!
//! By leveraging `thiserror`, the module provides human-readable error messages
//! that improve developer experience and program maintainability.

use solana_program::program_error::ProgramError;
use thiserror::Error;

/// Custom errors for the staking pool program.
///
/// These errors represent specific conditions that can occur during the
/// execution of staking pool instructions. Each variant is mapped to
/// a unique error code for use with Solana's `ProgramError`.
#[derive(Debug, Copy, Clone, Error)]
pub enum StakingError {
    /// The provided instruction data is invalid or unrecognized.
    #[error("Invalid Instruction")]
    InvalidInstruction,

    /// The signer account is either missing or not properly authorized.
    #[error("Invalid signer")]
    InvalidSigner,

    /// The owner of the provided account is invalid or does not match the program ID.
    #[error("Invalid owner")]
    InvalidOwner,

    /// The account has already been initialized and cannot be initialized again.
    #[error("Account already initialized")]
    AccountInitialized,
}

/// Converts `StakingError` into Solana's `ProgramError`.
///
/// This implementation maps each `StakingError` variant to a `ProgramError::Custom`
/// code.
///
/// # Usage
/// Solana programs can use the `?` operator with `StakingError` to return errors
/// as `ProgramError` seamlessly.
///
/// # Example
/// ```rust
/// return Err(StakingError::InvalidSigner.into());
/// ```
impl From<StakingError> for ProgramError {
    fn from(err: StakingError) -> Self {
        ProgramError::Custom(err as u32)
    }
}
