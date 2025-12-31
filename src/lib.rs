// Copyright (c) 2023 d-k-bo
// SPDX-License-Identifier: BSD-3-Clause

//! High-level bindings for
//! [libopusenc](https://opus-codec.org/docs/libopusenc_api-0.2/index.html).
//!
//! # Example
//!
//! ```
//! # use std::io::Read;
//! # use opusenc::{Comments, Encoder, MappingFamily, RecommendedTag};
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let audio_data: Vec<i16> = {
//!     let mut file = std::fs::File::open("/dev/urandom")?;
//!     let mut buf = vec![0; 60 * 48_000 * 2 * 2];
//!     file.read_exact(&mut buf)?;
//!     buf.chunks_exact(2)
//!         .map(|a| i16::from_ne_bytes([a[0], a[1]]))
//!         .collect()
//! };
//!
//! let mut encoder = Encoder::create_file(
//!     "/tmp/noise.opus",
//!     Comments::create()
//!         .add(RecommendedTag::Title, "Random Noise")?
//!         .add(RecommendedTag::Artist, "/dev/urandom")?,
//!     48_000,
//!     2,
//!     MappingFamily::MonoStereo,
//! )?;
//!
//! encoder.write(&audio_data)?;
//! encoder.drain()?;
//!
//! # Ok(())
//! # }
//! ```
//!
//! # Encoder options
//!
//! This crate provides a `encoder-options` feature which enables reading and changing encoder options.
//!
//! **Warning:** Some of these options might not work with opusenc, may be unsafe or even cause UB.
//! They are intended to be used via C macros that don't work with Rust.
//! Make sure to check if the methods you use match their intended behaviour.

use std::ffi::CStr;

pub use crate::{
    comments::{Comments, PicardTag, PictureType, RecommendedTag},
    encoder::{Encoder, MappingFamily, Bitrate},
    error::{Error, OpusencError, Result},
};
pub use opusenc_sys as ffi;

mod comments;
mod encoder;
mod error;
#[cfg(feature = "encoder-options")]
pub mod options;

/// Returns a string representing the version of libopusenc being used at run time.
pub fn version_str() -> &'static CStr {
    unsafe { CStr::from_ptr(crate::ffi::ope_get_version_string()) }
}

/// ABI version for libopusenc's header.
/// Can be used to check for features at run time.
pub fn abi_version() -> i32 {
    unsafe { crate::ffi::ope_get_abi_version() }
}
