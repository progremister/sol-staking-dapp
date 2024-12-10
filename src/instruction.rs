use borsh::{BorshDeserialize, BorshSerialize};

/// The `Instruction` enum defines the set of instructions that can be issued to the program.
///
/// Each variant represents a distinct operation. This enum is serializable and deserializable
/// using the Borsh format, ensuring compatibility with Solana's on-chain execution model.
///
/// # Variants
///
/// - `Initialize`: Initializes the program with specific parameters.
/// - `CreateUser`: Creates a user account within the program.
/// - `Stake`: Stakes a specified amount of tokens.
/// - `Unstake`: Unstakes a specified amount of tokens.
/// - `Claim`: Claims rewards for the user.
///
/// # Serialization
///
/// This enum uses the [Borsh](https://docs.rs/borsh) format for serialization and deserialization,
/// which is compact and efficient, making it well-suited for blockchain applications.
///
/// # Example
///
/// ```rust
/// use borsh::{BorshDeserialize, BorshSerialize};
/// use my_program::Instruction;
///
/// // Example of creating an instruction
/// let instruction = Instruction::Initialize { rewards_per_token: 100 };
///
/// // Serialize the instruction
/// let serialized = instruction.try_to_vec().unwrap();
///
/// // Deserialize the instruction
/// let deserialized = Instruction::try_from_slice(&serialized).unwrap();
///
/// assert_eq!(instruction, deserialized);
/// ```
#[derive(BorshSerialize, BorshDeserialize)]
pub enum Instruction {
    /// Initializes the program with specific parameters.
    ///
    /// # Fields
    ///
    /// - `rewards_per_token`: The rewards rate per token, specified as a `u64`.
    Initialize { rewards_per_token: u64 },

    /// Creates a new user account within the program.
    ///
    /// This instruction sets up a user account to participate in staking and rewards.
    CreateUser {},

    /// Stakes a specified amount of tokens.
    ///
    /// # Fields
    ///
    /// - `amount`: The amount of tokens to stake, specified as a `u64`.
    Stake { amount: u64 },

    /// Unstakes a specified amount of tokens.
    ///
    /// # Fields
    ///
    /// - `amount`: The amount of tokens to unstake, specified as a `u64`.
    Unstake { amount: u64 },

    /// Claims rewards for the user.
    ///
    /// This instruction allows the user to claim accumulated rewards based on their staking activity.
    Claim {},
}
