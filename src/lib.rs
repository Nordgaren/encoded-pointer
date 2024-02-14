#![doc = include_str!("../README.md")]

pub mod decoded;
pub mod encoded;
#[cfg(test)]
#[cfg_attr(feature = "double-encoded", path= "tests_double.rs")]
mod tests;
