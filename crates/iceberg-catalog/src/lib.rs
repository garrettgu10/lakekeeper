#![warn(
    missing_debug_implementations,
    rust_2018_idioms,
    unreachable_pub,
    clippy::pedantic
)]
#![allow(clippy::module_name_repetitions, clippy::large_enum_variant)]
#![forbid(unsafe_code)]

pub mod catalog;
mod config;
pub mod modules;
pub use modules::{ProjectIdent, SecretIdent, WarehouseIdent};

pub use config::{AuthZBackend, OpenFGAAuth, SecretBackend, CONFIG, DEFAULT_PROJECT_ID};

pub mod implementations;

mod request_metadata;

pub mod rest;

#[cfg(feature = "router")]
pub mod metrics;
#[cfg(feature = "router")]
pub(crate) mod tracing;
