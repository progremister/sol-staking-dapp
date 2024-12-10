//! Module: PoolStorageAccount
//!
//! This module defines the state for a staking pool in a Solana program.
//! It uses the `borsh` serialization library to serialize and deserialize the state
//! for efficient storage and processing on the Solana blockchain.
//!
//! ## Purpose
//! The `PoolStorageAccount` struct maintains the key state variables needed to manage
//! a staking pool. This includes information about the pool authority, the amount of staked tokens,
//! the number of users participating in the staking pool, and the reward distribution logic.
//!
//! ## Fields
//! - `pool_authority`: Public key of the staking pool's owner or manager.
//! - `total_staked`: Total amount of tokens staked in the pool.
//! - `user_count`: Number of users currently participating in the staking pool.
//! - `rewards_per_token`: Amount of rewards allocated per token staked.
//!
//! ## Usage
//! This struct is serialized and deserialized using the `borsh` library for efficient
//! storage in Solana accounts. It serves as the main state container for the staking pool.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

/// Represents the state of a staking pool in a Solana program.
///
/// This struct holds key information about the staking pool, including the pool authority,
/// the total staked tokens, the number of participating users, and the reward calculation
/// parameters.
///
///
/// ## Best Practices
/// - Always ensure that the `pool_authority` is set to a valid `Pubkey`.
/// - Update `total_staked` and `user_count` accurately during staking and
///   unstaking operations.
/// - `rewards_per_token` should be recalculated and updated as rewards are distributed
///   or additional tokens are staked.

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct PoolStorageAccount {
    /// Public key of the authority or owner of the staking pool.
    ///
    /// This is typically the program or wallet responsible for managing
    /// the staking pool, including reward distribution and pool operations.
    pub pool_authority: Pubkey,

    /// Total amount of tokens currently staked in the pool.
    ///
    /// This value represents the sum of all tokens staked by all users.
    /// It is used for calculating reward distributions and managing pool capacity.
    pub total_staked: u64,

    /// Number of users currently participating in the staking pool.
    ///
    /// This value helps track the number of active participants in the staking
    /// pool. It can be used for analytical purposes or for imposing restrictions
    /// on pool size.
    pub user_count: u64,

    /// Rewards earned per token staked.
    ///
    /// This value defines how many reward tokens are distributed for each token
    /// staked in the pool. It is typically calculated based on pool parameters
    /// and updated periodically.
    pub rewards_per_token: u64,
}
