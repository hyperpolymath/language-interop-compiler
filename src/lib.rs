// SPDX-License-Identifier: PMPL-1.0-or-later
//! Language Interoperability Compiler
//!
//! Automatic, formally verified language bindings via protocol-squisher integration.
//!
//! # The Breakthrough
//!
//! **Language types ARE serialization formats.**
//!
//! - ReScript `type user` ≈ JSON schema ≈ Protobuf message
//! - protocol-squisher generates adapters between serialization formats
//! - Therefore, protocol-squisher can generate language bindings!
//!
//! # Example
//!
//! ```ignore
//! use language_interop::analyzers::*;
//!
//! // Analyze ReScript type
//! let rescript_source = r#"
//! type user = {
//!   id: int,
//!   name: string,
//!   email: string,
//!   active: bool,
//! }
//! "#;
//! let rescript_type = rescript_analyzer::analyze_rescript_type(rescript_source).unwrap();
//!
//! // Analyze Rust type
//! let rust_source = r#"
//! pub struct User {
//!     pub id: i64,
//!     pub name: String,
//!     pub email: String,
//!     pub active: bool,
//! }
//! "#;
//! let rust_type = rust_analyzer::analyze_rust_type(rust_source).unwrap();
//!
//! // Generate compatibility report
//! let report = compatibility_report(&rescript_type, &rust_type);
//! println!("{}", report);
//! ```
//!
//! # Transport Classes
//!
//! - **Concorde**: 100% fidelity (ReScript ↔ Rust)
//! - **Business Class**: 95-99% fidelity
//! - **Economy**: 80-94% fidelity
//! - **Wheelbarrow**: <80% fidelity
//!
//! # Integration with protocol-squisher
//!
//! This library extends protocol-squisher (located at `~/Documents/hyperpolymath-repos/protocol-squisher`)
//! with language type analyzers, enabling automatic FFI binding generation.

pub mod analyzers;

// Re-export main types
pub use analyzers::{
    TransportClass,
    calculate_transport_class,
    compatibility_report,
};
