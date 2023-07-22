// Copyright (c) 2023 d-k-bo
// SPDX-License-Identifier: BSD-3-Clause

use std::{borrow::Borrow, ffi::CString, os::unix::prelude::OsStrExt, path::Path};

use crate::{error::CheckResult, Comments, Result};

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
