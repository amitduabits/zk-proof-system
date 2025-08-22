//! FFI bindings for the ZK proof system
//!
//! This module provides C-compatible bindings for use from other languages.

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]

pub mod ffi;
pub mod wasm;

/// C-compatible error codes
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ErrorCode {
    /// Success
    Success = 0,
    /// Invalid parameter
    InvalidParameter = 1,
    /// Verification failed
    VerificationFailed = 2,
    /// Unknown error
    Unknown = 99,
}
