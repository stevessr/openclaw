/// Core types and utilities for OpenClaw
///
/// This crate provides the foundational types, traits, and utilities
/// used across the OpenClaw ecosystem.
pub mod config;
pub mod error;
pub mod logging;
pub mod serialization;
pub mod session;
pub mod types;
pub mod utils;

pub use error::{Error, Result};
