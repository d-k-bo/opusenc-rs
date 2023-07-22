//! Low-level binding for
//! [libopusenc](https://opus-codec.org/docs/libopusenc_api-0.2/index.html).

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]

use std::ffi::c_int;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// libopus/libopusenc uses C macros to get options.
///
/// A `GET_$X(x)`  macro usually resolves to `GET_$X_REQUEST, x` where x is checked to be a valid pointer
#[inline]
pub unsafe fn ope_encoder_get_option(enc: *mut OggOpusEnc, request: u32, x: *mut c_int) -> c_int {
    ope_encoder_ctl(enc, request as c_int, x)
}
/// libopus/libopusenc uses C macros to get options.
///
/// A `SET_$X(x)`  macro usually resolves to `GET_$X_REQUEST, x` where x is checked to be a valid i32
#[inline]
pub unsafe fn ope_encoder_set_option(enc: *mut OggOpusEnc, request: u32, x: c_int) -> c_int {
    ope_encoder_ctl(enc, request as c_int, x)
}
