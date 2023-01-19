#![forbid(unsafe_code)]
#![deny(
    clippy::all, //
    clippy::must_use_candidate, //
)]

#[macro_use]
mod utils;

#[macro_use]
mod error;

mod auth;
mod header;
mod http;
mod ops;
mod signature_v4;
mod xml;

pub mod dto;
pub mod path;
pub mod service;

pub use self::auth::*;
pub use self::error::*;
pub use self::ops::S3;

#[cfg(test)]
mod tests {
    mod xml;
}
