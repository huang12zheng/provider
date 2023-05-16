//! # Common dependencies
//!
//! This internal crate simply re-exports dependencies that are commonly used across other internal
//! crates. Rust has a small `std` library (["and that's OK"](https://blog.nindalf.com/posts/rust-stdlib/)).
//! This crate acts as our internal version of a standard library, like [`stdx`](https://github.com/brson/stdx)
//! and others.
//!
//! The primary benefit of this crate is that there is only one place that version numbers for
//! commonly used dependencies need to be updated. Some of these crates are in line to become
//! part of the `std` library (e.g. `once_cell`).
// due to use test_case
#![feature(custom_test_frameworks)]
pub use atlist_rs;
// pub use async_recursion;
// pub use async_trait;
// pub use base64;
// pub use chrono;
// pub use clap;
// pub use cn_id_card;
// pub use defaults;
pub use crossbeam;
pub use derivative;
pub use derive_builder;
// pub use derive_more;
// pub use dirs;
// pub use extension_trait;
pub use eyre;
// pub use futures;
pub use generational_token_list;
pub use getset;
// pub use glob;
// pub use indexmap;
// pub use inflector;
pub use itertools;
// pub use json5;
pub use lazy_static;
// pub use libp2p_identity as identity;
// pub use libp2p_noise as noise;
// pub use maplit;
// pub use monostate;
pub use once_cell;
// pub use paste;
// pub use regex;
// pub use serde;
// pub use serde_json;
// pub use serde_with;
// pub use serde_yaml;
// pub use similar;
// pub use slug;
// pub use smol_str;
// pub use strum;
// pub use tempfile;
pub use test_case;
pub use thiserror;
// pub use tokio;
// pub use toml;
// pub use tracing;
// pub use validate_patch;
// pub use validator;
