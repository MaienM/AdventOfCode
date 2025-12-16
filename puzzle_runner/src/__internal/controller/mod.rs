mod bincontroller;
mod cli;

#[cfg(not(target_arch = "wasm32"))]
pub use bincontroller::BinController;
pub use cli::main;
