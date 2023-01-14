//! Rust crate with Serde implementation for json part in .sb3 format. (the Scratch 3 project format)

pub mod asset;
pub mod monitor;
pub mod project;
pub mod string_hashmap;
pub mod target;
pub mod value;

pub mod block;
pub mod broadcast;
pub mod comment;
pub mod list;
pub mod variable;

#[cfg(test)]
pub mod test_serde;

pub(crate) mod prelude;

mod utils;
