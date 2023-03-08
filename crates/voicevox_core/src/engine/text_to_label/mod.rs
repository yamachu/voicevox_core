mod shared;

#[cfg_attr(not(target_family = "wasm"), path = "native.rs")]
// #[cfg_attr(target_family = "wasm", path = "wasm.rs")]
mod implement;

pub use self::shared::*;
pub use self::implement::*;
