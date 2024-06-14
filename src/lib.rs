//! # bevy-pigeon
//!
//! The bevy plugin for `carrier-pigeon`.
//!
//! Building on `carrier-pigeon`, this crate provides high level network abstractions to allow making a game a breeze.
//! `bevy-pigeon` takes care of most of the networking for you, so you barely have to see it or think about it.
//!
//! ### Add bevy-pigeon to your `Cargo.toml`:
//!
//! `bevy-pigeon = "0.5.0"`
//!
//! ## Examples
//!
//! Complete examples are provided in the
//! [`examples/` directory](https://github.com/MitchellMarinoDev/carrier-pigeon/blob/main/examples)
//! on the GitHub repo.

#![warn(missing_debug_implementations, missing_copy_implementations)]
pub mod app;
pub mod sync;
#[cfg(feature = "types")]
pub mod types;

pub use app::{AppExt, ClientPlugin, NetLabel, ServerPlugin, SyncC};
