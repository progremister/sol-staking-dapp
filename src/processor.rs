//! # Staking Pool Processing Module
//!
//! This module handles the core logic for a staking pool program in Solana.
//! It processes instructions received by the program and updates the state
//! accordingly. The module is designed to ensure efficient and secure
//! handling of staking pool initialization.
//!
//! ## Instructions Supported
//!
//! - **Initialize**: Initializes the staking pool with a given reward rate per token.
//!
//! ## Key Functions
//!
//! - `process`: Entry point for processing instructions in the program.
//! - `process_initialize_pool`: Handles the `Initialize` instruction, setting up the staking pool's state.

use crate::error::StakingError;
use crate::instruction::Instruction;
use borsh::BorshDeserialize;
use solana_program::{account_info::*, entrypoint::ProgramResult, msg, pubkey::Pubkey};

/// Entry point for processing instructions in the staking pool program.
///
/// This function deserializes the incoming instruction data and routes it
/// to the appropriate processing function based on the instruction type.
///
/// # Parameters
/// - `program_id`: The public key of the currently executing program.
/// - `accounts`: The list of account information provided to the program.
/// - `instruction_data`: Serialized data for the instruction to be processed.
///
/// # Errors
/// - Returns `StakingError::InvalidInstruction` if the instruction is unrecognized.
///
pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Deserialize the instruction from the provided data
    let instruction = Instruction::try_from_slice(instruction_data)?;

    // Match the instruction type and call the appropriate handler
    match instruction {
        Instruction::Initialize { rewards_per_token } => {
            msg!("Initialize pool");
            process_initialize_pool(program_id, accounts, rewards_per_token)
        }
        _ => Err(StakingError::InvalidInstruction.into()),
    }
}

/// Processes the `Initialize` instruction.
///
/// This function initializes a new staking pool by setting up the
/// pool authority, initial reward rate, and other state variables.
///
/// # Parameters
/// - `program_id`: The public key of the currently executing program.
/// - `accounts`: The list of account information provided to the program.
/// - `rewards_per_token`: The reward rate per token for the staking pool.
///
/// # Account Requirements
/// - The first account must be the signer of the transaction (authority).
/// - The second account must be the storage account for the staking pool and
///   must belong to the executing program.
///
/// # Errors
/// - Returns `StakingError::InvalidSigner` if the first account is not a signer.
/// - Returns `StakingError::InvalidOwner` if the storage account is not owned by the program.
/// - Returns `StakingError::AlreadyInitialized` if the staking pool has already been initialized.
///
fn process_initialize_pool(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    rewards_per_token: u64,
) -> ProgramResult {
    // Get the iterator over the accounts
    let accounts_iter = &mut accounts.iter();

    // Verify that the first account is a valid signer
    let signer = next_account_info(accounts_iter)?;
    if !signer.is_signer {
        return Err(StakingError::InvalidSigner.into());
    }

    // Verify that the second account is the storage account and is owned by the program
    let storage = next_account_info(accounts_iter)?;
    if storage.owner != program_id {
        return Err(StakingError::InvalidOwner.into());
    }

    // Deserialize the storage account data into a PoolStorageAccount
    let mut storage_data = PoolStorageAccount::try_from_slice(&storage.data.borrow())?;
    if storage_data.is_initialized() {
        return Err(StakingError::AlreadyInitialized.into());
    }

    // Initialize the staking pool state
    storage_data.pool_authority = *signer.key;
    storage_data.total_staked = 0u64;
    storage_data.user_count = 0u64;
    storage_data.rewards_per_token = rewards_per_token;
    storage_data.is_initialized = true;

    // Serialize the updated state back into the storage account
    storage_data.serialize(&mut &mut storage.data.borrow_mut()[..])?;

    // Log the initialization details for debugging
    msg!("Staking pool is initialized {:#?}", storage_data);

    Ok(())
}
