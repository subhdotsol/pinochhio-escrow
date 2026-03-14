// Feature-gate the entrypoint so this crate can also be used as a lib in tests
#[cfg(feature = "bpf-entrypoint")]
pub mod entrypoint;

pub mod instructions;
pub mod processor;
pub mod state;
