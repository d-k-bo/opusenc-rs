// Copyright (c) 2023 d-k-bo
// SPDX-License-Identifier: BSD-3-Clause

use std::ffi::c_int;
use std::{borrow::Borrow, ffi::CString, os::unix::prelude::OsStrExt, path::Path};

use crate::{error::CheckResult, Comments, Result};

// Encoder CTLs
const OPUS_SET_BITRATE: c_int = 4002; // in i32

// Bitrate
const OPUS_AUTO: c_int = -1000;
const OPUS_BITRATE_MAX: c_int = -1;

/// Encoder for creating Ogg Opus files.
///
/// Create an encoder using [`Encoder::create_file`], write samples using
/// [`Encoder::write`]/[`Encoder::write_float`] and make sure to call
/// [`Encoder::drain`] before dropping it.
#[derive(Debug)]
pub struct Encoder {
    ptr: *mut crate::ffi::OggOpusEnc,
    channels: usize,
}

impl Drop for Encoder {
    fn drop(&mut self) {
        unsafe { crate::ffi::ope_encoder_destroy(self.ptr) }
    }
}

impl Encoder {
    /// Create a new OggOpus file.
    pub fn create_file(
        path: impl AsRef<Path>,
        comments: impl Borrow<Comments>,
        rate: i32,
        channels: usize,
        family: MappingFamily,
    ) -> Result<Self> {
        let path = CString::new(path.as_ref().as_os_str().as_bytes())?;

        let mut error = 0;
        let ptr = unsafe {
            crate::ffi::ope_encoder_create_file(
                path.as_ptr(),
                comments.borrow().ptr(),
                rate,
                channels.try_into().unwrap(),
                family as i32,
                &mut error,
            )
        };
        error.check_result()?;
        assert!(!ptr.is_null());

        Ok(Self { ptr, channels })
    }

    /// Create a new OggOpus encoder in pull style.
    ///
    /// Use together with [`Encoder::get_page`] to get the encoded data.
    pub fn create_pull(
        comments: impl Borrow<Comments>,
        rate: i32,
        channels: usize,
        family: MappingFamily,
    ) -> Result<Self> {
        let mut error = 0;
        let ptr = unsafe {
            crate::ffi::ope_encoder_create_pull(
                comments.borrow().ptr(),
                rate,
                channels.try_into().unwrap(),
                family as i32,
                &mut error,
            )
        };
        error.check_result()?;
        assert!(!ptr.is_null());

        Ok(Self { ptr, channels })
    }
}

/// Possible bitrates.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Bitrate {
    /// Explicit bitrate choice (in bits/second).
    Bits(i32),
    /// Maximum bitrate allowed (up to maximum number of bytes for the packet).
    Max,
    /// Default bitrate decided by the encoder (not recommended).
    Auto,
}

impl Encoder {
    /// Set the encoder bitrate.
    pub fn set_bitrate(&mut self, bitrate: Bitrate) -> Result<()> {
        let bitrate: i32 = match bitrate {
            Bitrate::Auto => OPUS_AUTO,
            Bitrate::Max => OPUS_BITRATE_MAX,
            Bitrate::Bits(b) => b,
        };
        unsafe { crate::ffi::ope_encoder_ctl(self.ptr, OPUS_SET_BITRATE, bitrate) }.check_result()
    }
}

impl Encoder {
    /// Add/encode any number of float samples to the stream.
    ///
    /// Data for multiple channels must be interleaved.
    ///
    /// # Panics
    ///
    /// Panics if the sample count isn't a multiple of the channel count.
    pub fn write_float(&mut self, pcm: &[f32]) -> Result<()> {
        if pcm.len() % self.channels != 0 {
            panic!("sample count must be a multiple of channel count")
        }

        unsafe {
            crate::ffi::ope_encoder_write_float(
                self.ptr,
                pcm.as_ptr(),
                (pcm.len() / self.channels).try_into().unwrap(),
            )
        }
        .check_result()
    }
    /// Add/encode any number of 16-bit linear samples to the stream.
    ///
    /// Data for multiple channels must be interleaved.
    ///
    /// # Panics
    ///
    /// Panics if the sample count isn't a multiple of the channel count.
    pub fn write(&mut self, pcm: &[i16]) -> Result<()> {
        if pcm.len() % self.channels != 0 {
            panic!("sample count must be a multiple of channel count")
        }

        unsafe {
            crate::ffi::ope_encoder_write(
                self.ptr,
                pcm.as_ptr(),
                (pcm.len() / self.channels).try_into().unwrap(),
            )
        }
        .check_result()
    }

    /// Get the next page from the stream
    ///
    /// Only use if creating encoder with [`Encoder::create_pull`].
    pub fn get_page(&mut self, flush: bool) -> Option<&[u8]> {
        let mut page_ptr: *mut std::ffi::c_uchar = std::ptr::null_mut();
        let mut page_size = 0;
        unsafe {
            let available = crate::ffi::ope_encoder_get_page(
                self.ptr,
                &mut page_ptr,
                &mut page_size,
                flush as i32,
            );
            if available == 1 {
                Some(std::slice::from_raw_parts(page_ptr, page_size as usize))
            } else {
                None
            }
        }
    }
    /// Finalizes the stream.
    pub fn drain(&mut self) -> Result<()> {
        unsafe { crate::ffi::ope_encoder_drain(self.ptr) }.check_result()
    }
    /// Ends the stream and create a new stream within the same file.
    pub fn chain_current(&mut self, comments: impl Borrow<Comments>) -> Result<()> {
        unsafe { crate::ffi::ope_encoder_chain_current(self.ptr, comments.borrow().ptr()) }
            .check_result()
    }
    /// Ends the stream and create a new file.
    pub fn continue_new_file(
        &mut self,
        path: impl AsRef<Path>,
        comments: impl Borrow<Comments>,
    ) -> Result<()> {
        let path = CString::new(path.as_ref().as_os_str().as_bytes())?;
        unsafe {
            crate::ffi::ope_encoder_continue_new_file(
                self.ptr,
                path.as_ptr(),
                comments.borrow().ptr(),
            )
        }
        .check_result()
    }
    /// Write out the header now rather than wait for audio to begin.
    pub fn flush_header(&mut self) -> Result<()> {
        unsafe { crate::ffi::ope_encoder_flush_header(self.ptr) }.check_result()
    }
    /// Gets encoder options.
    #[inline]
    #[cfg(feature = "encoder-options")]
    pub fn get_option(&self, request: u32) -> Result<i32> {
        let mut x = 0;
        unsafe { crate::ffi::ope_encoder_get_option(self.ptr, request, &mut x) }.check_result()?;
        Ok(x)
    }
    /// Sets encoder options.
    #[inline]
    #[cfg(feature = "encoder-options")]
    pub fn set_option(&mut self, request: u32, value: i32) -> Result<()> {
        unsafe { crate::ffi::ope_encoder_set_option(self.ptr, request, value) }.check_result()
    }
}

impl Encoder {
    pub fn ptr(&self) -> *mut crate::ffi::OggOpusEnc {
        self.ptr
    }
}

/// Describes wether the channels are mapped as mono, stereo or surround.
#[derive(Copy, Clone, Debug)]
#[repr(i32)]
pub enum MappingFamily {
    MonoStereo,
    Surround,
}
