#![no_std]

mod contract;
pub mod event;
pub mod interface;
mod storage;
pub mod abi;

pub use contract::{AxelarGMP, AxelarGMPClient};