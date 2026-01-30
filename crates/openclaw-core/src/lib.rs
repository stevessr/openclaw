/// Core types and utilities for OpenClaw
///
/// This crate provides the foundational types, traits, and utilities
/// used across the OpenClaw ecosystem.
pub mod config;
pub mod error;
pub mod session;
pub mod types;

pub use error::{Error, Result};
