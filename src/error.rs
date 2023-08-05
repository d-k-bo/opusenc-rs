// Copyright (c) 2023 d-k-bo
// SPDX-License-Identifier: BSD-3-Clause

use std::{
    ffi::{CStr, NulError},
    fmt::Display,
};

use num_enum::{FromPrimitive, IntoPrimitive};
#[cfg(feature = "encoder-options")]
use num_enum::{TryFromPrimitive, TryFromPrimitiveError};

pub type Result<T> = std::result::Result<T, Error>;

/// The error type returned by all library functions.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// The provided string contains a nul byte.
    InvalidString(NulError),
    /// libopusenc returned an error.
    Opusenc(OpusencError),
    /// libopusenc returned an invalid value for an option.
    InvalidValue(&'static str, i32),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidString(e) => e.fmt(f),
            Error::Opusenc(e) => e.fmt(f),
            Error::InvalidValue(name, value) => write!(
                f,
                "libopusenc returned invald or unknown value `{value}` for `{name}`"
            ),
        }
    }
}

impl From<NulError> for Error {
    fn from(e: NulError) -> Self {
        Error::InvalidString(e)
    }
}

impl From<OpusencError> for Error {
    fn from(e: OpusencError) -> Self {
        Error::Opusenc(e)
    }
}

#[cfg(feature = "encoder-options")]
impl<Enum> From<TryFromPrimitiveError<Enum>> for Error
where
    Enum: TryFromPrimitive<Primitive = i32> + 'static,
{
    fn from(e: TryFromPrimitiveError<Enum>) -> Self {
        Error::InvalidValue(Enum::NAME, e.number)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::InvalidString(e) => Some(e),
            Error::Opusenc(e) => Some(e),
            Error::InvalidValue(_, _) => None,
        }
    }
}

/// Error code as returned from libopusenc.
#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum OpusencError {
    BadArg = -11,
    InternalError = -13,
    Unimplemented = -15,
    AllocFail = -17,
    CannotOpen = -30,
    TooLate = -31,
    InvalidPicture = -32,
    InvalidIcon = -33,
    WriteFail = -34,
    CloseFail = -35,

    #[num_enum(catch_all)]
    Other(i32) = 1,
}

impl Display for OpusencError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe { CStr::from_ptr(crate::ffi::ope_strerror(i32::from(*self))) }
            .to_string_lossy()
            .fmt(f)
    }
}

impl std::error::Error for OpusencError {}

pub(crate) trait CheckResult {
    fn check_result(self) -> Result<()>;
}

impl CheckResult for i32 {
    fn check_result(self) -> Result<()> {
        if self == 0 {
            Ok(())
        } else {
            Err(Error::Opusenc(self.into()))
        }
    }
}
