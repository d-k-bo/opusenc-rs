// Copyright (c) 2023 d-k-bo
// SPDX-License-Identifier: BSD-3-Clause

use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::{Encoder, Result};

pub use self::{ope::*, opus::*};
pub use crate::ffi::{OPUS_AUTO, OPUS_BITRATE_MAX};

mod opus {
    use super::*;

    #[derive(Copy, Clone, Debug, PartialEq, TryFromPrimitive, IntoPrimitive)]
    #[repr(i32)]
    pub enum Application {
        /// Process signal for improved speech intelligibility.
        Voip = crate::ffi::OPUS_APPLICATION_VOIP as i32,
        /// Favor faithfulness to the original input.
        Audio = crate::ffi::OPUS_APPLICATION_AUDIO as i32,
        /// Configure the minimum possible coding delay by disabling certain modes of operation.
        RestrictedLowdelay = crate::ffi::OPUS_APPLICATION_RESTRICTED_LOWDELAY as i32,
    }

    #[derive(Copy, Clone, Debug, PartialEq, TryFromPrimitive, IntoPrimitive)]
    #[repr(i32)]
    pub enum Framesize {
        /// Select frame size from the argument (default).
        Arg = crate::ffi::OPUS_FRAMESIZE_ARG as i32,
        /// Use 2.5 ms frames.
        Ms2_5 = crate::ffi::OPUS_FRAMESIZE_2_5_MS as i32,
        /// Use 5 ms frames.
        Ms5 = crate::ffi::OPUS_FRAMESIZE_5_MS as i32,
        /// Use 10 ms frames.
        Ms10 = crate::ffi::OPUS_FRAMESIZE_10_MS as i32,
        /// Use 20 ms frames.
        Ms20 = crate::ffi::OPUS_FRAMESIZE_20_MS as i32,
        /// Use 40 ms frames.
        Ms40 = crate::ffi::OPUS_FRAMESIZE_40_MS as i32,
        /// Use 60 ms frames.
        Ms60 = crate::ffi::OPUS_FRAMESIZE_60_MS as i32,
        /// Use 80 ms frames.
        Ms80 = crate::ffi::OPUS_FRAMESIZE_80_MS as i32,
        /// Use 100 ms frames.
        Ms100 = crate::ffi::OPUS_FRAMESIZE_100_MS as i32,
        /// Use 120 ms frames.
        Ms120 = crate::ffi::OPUS_FRAMESIZE_120_MS as i32,
    }

    #[derive(Copy, Clone, Debug, PartialEq, TryFromPrimitive, IntoPrimitive)]
    #[repr(i32)]
    pub enum ForceChannels {
        /// Not forced (default)
        NotForced = OPUS_AUTO,
        /// Forced mono
        ForcedMono = 1,
        /// Forced stereo
        ForcedStereo = 2,
    }

    #[derive(Copy, Clone, Debug, PartialEq, TryFromPrimitive, IntoPrimitive)]
    #[repr(i32)]
    pub enum Bandwidth {
        Auto = OPUS_AUTO,
        /// 4 kHz passband
        Narrowband = crate::ffi::OPUS_BANDWIDTH_NARROWBAND as i32,
        /// 6 kHz passband
        Mediumband = crate::ffi::OPUS_BANDWIDTH_MEDIUMBAND as i32,
        /// 8 kHz passband
        Wideband = crate::ffi::OPUS_BANDWIDTH_WIDEBAND as i32,
        /// 12 kHz passband
        Superwideband = crate::ffi::OPUS_BANDWIDTH_SUPERWIDEBAND as i32,
        /// 20 kHz passband (default)
        Fullband = crate::ffi::OPUS_BANDWIDTH_FULLBAND as i32,
    }

    #[derive(Copy, Clone, Debug, PartialEq, TryFromPrimitive, IntoPrimitive)]
    #[repr(i32)]
    pub enum SignalType {
        /// (default)
        Auto = crate::ffi::OPUS_AUTO,
        /// Bias thresholds towards choosing LPC or Hybrid modes.
        Voice = crate::ffi::OPUS_SIGNAL_VOICE as i32,
        /// Bias thresholds towards choosing MDCT modes.
        Music = crate::ffi::OPUS_SIGNAL_MUSIC as i32,
    }

    /// Encoder options from libopus.
    ///
    /// **Warning:** Some of these options might not work with opusenc, may be unsafe or even cause UB.
    /// They are intended to be used via C macros that don't work with Rust.
    /// Make sure to check if the methods you use match their intended behaviour.
    ///
    /// See [`opus_defines.h`](https://gitlab.xiph.org/xiph/opus/-/blob/master/include/opus_defines.h) and <https://opus-codec.org/docs/opus_api-1.3.1/group__opus__encoderctls.html>
    impl Encoder {
        /// Gets the encoder's configured application.
        pub fn get_application(&self) -> Result<Application> {
            self.get_option(crate::ffi::OPUS_GET_APPLICATION_REQUEST)?
                .try_into()
                .map_err(Into::into)
        }
        /// Configures the encoder's intended application.
        pub fn set_application(&mut self, v: Application) -> Result<()> {
            self.set_option(crate::ffi::OPUS_SET_APPLICATION_REQUEST, v.into())
        }
        /// Sets the encoder's bandpass to a specific value.
        ///
        /// This prevents the encoder from automatically selecting the bandpass based on the available bitrate. If an application knows the bandpass of the input audio it is providing, it should normally use [`Encoder::set_max_bandwidth`] instead, which still gives the encoder the freedom to reduce the bandpass when the bitrate becomes too low, for better overall quality.
        pub fn set_bandwidth(&mut self, v: Bandwidth) -> Result<()> {
            self.set_option(crate::ffi::OPUS_SET_BANDWIDTH_REQUEST, v.into())
        }
        /// Gets the encoder's bitrate configuration.
        ///
        /// Returns the bitrate in bits per second. The default is determined based on the number of channels and the input sampling rate.
        pub fn get_bitrate(&self) -> Result<i32> {
            self.get_option(crate::ffi::OPUS_GET_BITRATE_REQUEST)
        }
        /// Configures the bitrate in the encoder.
        ///
        /// Rates from 500 to 512000 bits per second are meaningful, as well as the special values [`OPUS_AUTO`] and [`OPUS_BITRATE_MAX`]. The value [`OPUS_BITRATE_MAX`] can be used to cause the codec to use as much rate as it can, which is useful for controlling the rate by adjusting the output buffer size.
        ///
        /// The default is determined based on the number of channels and the input sampling rate.
        pub fn set_bitrate(&mut self, v: i32) -> Result<()> {
            self.set_option(crate::ffi::OPUS_SET_BITRATE_REQUEST, v)
        }
        /// Gets the encoder's complexity configuration.
        ///
        /// Returns a value in the range 0-10, inclusive.
        pub fn get_complexity(&self) -> Result<i32> {
            self.get_option(crate::ffi::OPUS_GET_COMPLEXITY_REQUEST)
        }
        /// Configures the encoder's computational complexity.
        ///
        /// The supported range is 0-10 inclusive with 10 representing the highest complexity.
        pub fn set_complexity(&mut self, v: i32) -> Result<()> {
            self.set_option(crate::ffi::OPUS_SET_COMPLEXITY_REQUEST, v)
        }
        /// Gets encoder's configured use of discontinuous transmission.
        ///
        /// - `false` - DTX disabled (default).
        /// - `true` - DTX enabled.
        pub fn get_dtx(&self) -> Result<bool> {
            Ok(self.get_option(crate::ffi::OPUS_GET_DTX_REQUEST)? != 0)
        }
        /// Configures the encoder's use of discontinuous transmission (DTX).
        ///
        /// - `false` - DTX disabled (default).
        /// - `true` - DTX enabled.
        ///
        /// **Note:** This is only applicable to the LPC layer
        pub fn set_dtx(&mut self, v: bool) -> Result<()> {
            self.set_option(crate::ffi::OPUS_SET_DTX_REQUEST, v.into())
        }
        /// Gets the encoder's configured use of variable duration frames.
        pub fn get_expert_frame_duration(&self) -> Result<Framesize> {
            self.get_option(crate::ffi::OPUS_GET_EXPERT_FRAME_DURATION_REQUEST)?
                .try_into()
                .map_err(Into::into)
        }
        /// Configures the encoder's use of variable duration frames.
        ///
        /// When variable duration is enabled, the encoder is free to use a shorter frame size than the one requested in the opus_encode*() call. It is then the user's responsibility to verify how much audio was encoded by checking the ToC byte of the encoded packet. The part of the audio that was not encoded needs to be resent to the encoder for the next call. Do not use this option unless you **really** know what you are doing.
        pub fn set_expert_frame_duration(&mut self, v: Framesize) -> Result<()> {
            self.set_option(crate::ffi::OPUS_SET_EXPERT_FRAME_DURATION_REQUEST, v.into())
        }
        /// Gets the encoder's forced channel configuration.
        pub fn get_force_channels(&self) -> Result<ForceChannels> {
            self.get_option(crate::ffi::OPUS_GET_FORCE_CHANNELS_REQUEST)?
                .try_into()
                .map_err(Into::into)
        }
        /// Configures mono/stereo forcing in the encoder.
        ///
        /// This can force the encoder to produce packets encoded as either mono or stereo, regardless of the format of the input audio. This is useful when the caller knows that the input signal is currently a mono source embedded in a stereo stream.
        pub fn set_force_channels(&mut self, v: ForceChannels) -> Result<()> {
            self.set_option(crate::ffi::OPUS_SET_FORCE_CHANNELS_REQUEST, v.into())
        }
        /// Gets encoder's configured use of inband forward error correction.
        ///
        /// - `false` - Inband FEC disabled (default).
        /// - `true` - Inband FEC enabled.
        pub fn get_inband_fec(&self) -> Result<bool> {
            Ok(self.get_option(crate::ffi::OPUS_GET_INBAND_FEC_REQUEST)? != 0)
        }
        /// Configures the encoder's use of inband forward error correction (FEC).
        ///
        /// - `false` - Inband FEC disabled (default).
        /// - `true` - Inband FEC enabled.
        ///
        /// **Note:** This is only applicable to the LPC layer
        pub fn set_inband_fec(&mut self, v: bool) -> Result<()> {
            self.set_option(crate::ffi::OPUS_SET_INBAND_FEC_REQUEST, v.into())
        }
        /// Gets the total samples of delay added by the entire codec.
        ///
        /// This can be queried by the encoder and then the provided number of samples can be skipped on from the start of the decoder's output to provide time aligned input and output. From the perspective of a decoding application the real data begins this many samples late.
        ///
        /// The decoder contribution to this delay is identical for all decoders, but the encoder portion of the delay may vary from implementation to implementation, version to version, or even depend on the encoder's initial configuration. Applications needing delay compensation should call this CTL rather than hard-coding a value.
        pub fn get_lookahead(&self) -> Result<i32> {
            self.get_option(crate::ffi::OPUS_GET_LOOKAHEAD_REQUEST)
        }
        /// Gets the encoder's configured signal depth.
        ///
        /// Returns Input precision in bits, between 8 and 24 (default: 24).
        pub fn get_lsb_depth(&self) -> Result<i32> {
            self.get_option(crate::ffi::OPUS_GET_LSB_DEPTH_REQUEST)
        }
        /// Configures the depth of signal being encoded.
        ///
        /// This is a hint which helps the encoder identify silence and near-silence. It represents the number of significant bits of linear intensity below which the signal contains ignorable quantization or other noise.
        ///
        /// For example, OPUS_SET_LSB_DEPTH(14) would be an appropriate setting for G.711 u-law input. OPUS_SET_LSB_DEPTH(16) would be appropriate for 16-bit linear pcm input with opus_encode_float().
        ///
        /// When using opus_encode() instead of opus_encode_float(), or when libopus is compiled for fixed-point, the encoder uses the minimum of the value set here and the value 16.
        pub fn set_lsb_depth(&mut self, v: i32) -> Result<()> {
            self.set_option(crate::ffi::OPUS_SET_LSB_DEPTH_REQUEST, v)
        }
        /// Gets the encoder's configured maximum allowed bandpass.
        pub fn get_max_bandwidth(&self) -> Result<Bandwidth> {
            self.get_option(crate::ffi::OPUS_GET_MAX_BANDWIDTH_REQUEST)?
                .try_into()
                .map_err(Into::into)
        }
        /// Configures the maximum bandpass that the encoder will select automatically.
        ///
        /// Applications should normally use this instead of [`Encoder::set_bandwidth`] (leaving that set to the default, [`Bandwidth::Auto`]). This allows the application to set an upper bound based on the type of input it is providing, but still gives the encoder the freedom to reduce the bandpass when the bitrate becomes too low, for better overall quality.
        pub fn set_max_bandwidth(&mut self, v: Bandwidth) -> Result<()> {
            self.set_option(crate::ffi::OPUS_SET_MAX_BANDWIDTH_REQUEST, v.into())
        }
        /// Gets the encoder's configured packet loss percentage.
        ///
        /// Returns the configured loss percentage in the range 0-100, inclusive (default: 0).
        pub fn get_loss_percentage(&self) -> Result<i32> {
            self.get_option(crate::ffi::OPUS_GET_PACKET_LOSS_PERC_REQUEST)
        }
        /// Configures the encoder's expected packet loss percentage.
        ///
        /// Higher values trigger progressively more loss resistant behavior in the encoder at the expense of quality at a given bitrate in the absence of packet loss, but greater quality under loss.
        pub fn set_loss_percentage(&mut self, v: i32) -> Result<()> {
            self.set_option(crate::ffi::OPUS_SET_PACKET_LOSS_PERC_REQUEST, v)
        }
        /// Gets the encoder's configured prediction status.
        ///
        /// - `false` - Prediction enabled (default).
        /// - `true` - Prediction disabled.
        pub fn get_predicition_disabled(&self) -> Result<bool> {
            Ok(self.get_option(crate::ffi::OPUS_GET_PREDICTION_DISABLED_REQUEST)? != 0)
        }
        /// If set to 1, disables almost all use of prediction, making frames almost completely independent.
        ///
        /// This reduces quality.
        pub fn set_predicition_disabled(&mut self, v: bool) -> Result<()> {
            self.set_option(crate::ffi::OPUS_SET_PREDICTION_DISABLED_REQUEST, v.into())
        }
        /// Gets the encoder's configured signal type.
        pub fn get_signal_type(&self) -> Result<SignalType> {
            self.get_option(crate::ffi::OPUS_GET_SIGNAL_REQUEST)?
                .try_into()
                .map_err(Into::into)
        }
        /// Configures the type of signal being encoded.
        ///
        /// This is a hint which helps the encoder's mode selection.
        pub fn set_signal_type(&mut self, v: SignalType) -> Result<()> {
            self.set_option(crate::ffi::OPUS_SET_SIGNAL_REQUEST, v.into())
        }
        /// Determine if variable bitrate (VBR) is enabled in the encoder.
        ///
        /// - `false` - Hard CBR.
        /// - `true` - VBR (default). The exact type of VBR may be retrieved via [`Encoder::get_vbr_constraint`].
        pub fn get_vbr_enabled(&self) -> Result<bool> {
            Ok(self.get_option(crate::ffi::OPUS_GET_VBR_REQUEST)? != 0)
        }
        /// Enables or disables variable bitrate (VBR) in the encoder.
        ///
        /// The configured bitrate may not be met exactly because frames must be an integer number of bytes in length.
        ///
        /// - `false` - Hard CBR.
        /// - `true` - VBR (default). The exact type of VBR may be retrieved via [`Encoder::get_vbr_constraint`].
        pub fn set_vbr_enabled(&mut self, v: bool) -> Result<()> {
            self.set_option(crate::ffi::OPUS_SET_VBR_REQUEST, v.into())
        }
        /// Determine if constrained VBR is enabled in the encoder.
        ///
        /// - `false` - Unconstrained VBR.
        /// - `true` - Constrained VBR (default).
        pub fn get_vbr_constraint(&self) -> Result<bool> {
            Ok(self.get_option(crate::ffi::OPUS_GET_VBR_CONSTRAINT_REQUEST)? != 0)
        }
        /// Enables or disables constrained VBR in the encoder.
        ///
        /// This setting is ignored when the encoder is in CBR mode.
        ///
        /// - `false` - Hard CBR.
        /// - `true` - VBR (default). The exact type of VBR may be retrieved via [`Encoder::get_vbr_constraint`].
        ///
        /// **Warning:** Only the MDCT mode of Opus currently heeds the constraint. Speech mode ignores it completely, hybrid mode may fail to obey it if the LPC layer uses more bitrate than the constraint would have permitted.
        pub fn set_vbr_constraint(&mut self, v: bool) -> Result<()> {
            self.set_option(crate::ffi::OPUS_SET_VBR_CONSTRAINT_REQUEST, v.into())
        }
    }
}

mod ope {
    use super::*;

    /// Encoder options specific to libopusenc.
    ///
    /// Unfortunately, they are not documented yet.
    ///
    /// **Warning:** Some of these options may be unsafe or even cause UB.
    /// They are intended to be used via C macros that don't work with Rust.
    /// Make sure to check if the methods you use match their intended behaviour.
    ///
    /// See [`opusenc.h`](https://gitlab.xiph.org/xiph/libopusenc/-/blob/master/include/opusenc.h) and <https://opus-codec.org/docs/libopusenc_api-0.2/group__encoder__ctl.html>
    impl Encoder {
        pub fn get_decision_delay(&self) -> Result<i32> {
            self.get_option(crate::ffi::OPE_GET_DECISION_DELAY_REQUEST)
        }
        pub fn set_decision_delay(&mut self, v: i32) -> Result<()> {
            self.set_option(crate::ffi::OPE_SET_DECISION_DELAY_REQUEST, v)
        }
        pub fn get_muxing_delay(&self) -> Result<i32> {
            self.get_option(crate::ffi::OPE_GET_MUXING_DELAY_REQUEST)
        }
        pub fn set_muxing_delay(&mut self, v: i32) -> Result<()> {
            self.set_option(crate::ffi::OPE_SET_MUXING_DELAY_REQUEST, v)
        }
        pub fn get_comment_padding(&self) -> Result<i32> {
            self.get_option(crate::ffi::OPE_GET_COMMENT_PADDING_REQUEST)
        }
        pub fn set_comment_padding(&mut self, v: i32) -> Result<()> {
            self.set_option(crate::ffi::OPE_SET_COMMENT_PADDING_REQUEST, v)
        }
        pub fn get_serialno(&self) -> Result<i32> {
            self.get_option(crate::ffi::OPE_GET_SERIALNO_REQUEST)
        }
        pub fn set_serialno(&mut self, v: i32) -> Result<()> {
            self.set_option(crate::ffi::OPE_SET_SERIALNO_REQUEST, v)
        }
        pub fn get_header_gain(&self) -> Result<i32> {
            self.get_option(crate::ffi::OPE_GET_HEADER_GAIN_REQUEST)
        }
        pub fn set_header_gain(&mut self, v: i32) -> Result<()> {
            self.set_option(crate::ffi::OPE_SET_HEADER_GAIN_REQUEST, v)
        }
        pub fn get_nb_streams(&self) -> Result<i32> {
            self.get_option(crate::ffi::OPE_GET_NB_STREAMS_REQUEST)
        }
        pub fn get_nb_coupled_streams(&self) -> Result<i32> {
            self.get_option(crate::ffi::OPE_GET_NB_COUPLED_STREAMS_REQUEST)
        }
    }
}
