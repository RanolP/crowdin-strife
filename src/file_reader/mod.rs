#[cfg(not(target_arch = "wasm32"))]
mod non_wasm;
#[cfg(not(target_arch = "wasm32"))]
pub use non_wasm::*;

#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
pub use wasm::*;
