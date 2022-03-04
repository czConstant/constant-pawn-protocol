pub mod enums;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;
pub mod utils;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

pub use solana_program;
solana_program::declare_id!("LendingbGKPFXCWuBvfkegQfZyiNwAJb9Ss623VQ5DA");
