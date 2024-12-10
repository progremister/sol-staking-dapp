//! This module defines the entry point for the Solana program.
//! 
//! 
//! # Structure
//! - The `process_instruction` function serves as the program's entry point.
//! - Incoming instructions are routed to the `process` function for handling.
//!
//! # Usage
//! - This module should be used as part of a Solana on-chain program.
//! - It is automatically called by the Solana runtime when a transaction targets
//!   the program's ID.

use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

use crate::processor::process;

entrypoint!(process_instruction); //define Solana entrypoint 

/// The entry point for the Solana program.
///
/// This function is automatically invoked by the Solana runtime whenever
/// the program is called by a transaction. It delegates the handling of
/// instructions to the `process` function.
///
/// # Parameters
///
/// - `program_id`: The public key of the program. This is used to verify
///   that the program is correctly invoked.
/// - `accounts`: A slice of `AccountInfo` structures representing the accounts
///   involved in the instruction. These may include the program's accounts and
///   any other accounts required by the instruction.
/// - `instruction_data`: A byte array containing the serialized instruction data.
///
/// # Returns
///
/// This function returns a `ProgramResult`, which indicates success (`Ok(())`)
/// or failure (`Err`) with an appropriate error code.
///
/// # Errors
///
/// If the instruction data cannot be parsed, or if the processing logic
/// encounters an issue, an appropriate error will be returned.
///
/// # Example
///
/// ```rust
/// use solana_program::{
///     account_info::AccountInfo, pubkey::Pubkey, entrypoint::ProgramResult,
/// };
///
/// fn process_instruction(
///     program_id: &Pubkey,
///     accounts: &[AccountInfo],
///     instruction_data: &[u8],
/// ) -> ProgramResult {
///     // Example implementation
///     Ok(())
/// }
/// ```
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    process(program_id, accounts, instruction_data)
}