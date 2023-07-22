# opusenc-rs

[![Build Status](https://github.com/d-k-bo/opusenc-rs/workflows/CI/badge.svg)](https://github.com/d-k-bo/opusenc-rs/actions?query=workflow%3ACI)
[![Crates.io](https://img.shields.io/crates/v/opusenc)](https://crates.io/crates/opusenc)
[![Documentation](https://img.shields.io/docsrs/opusenc)](https://docs.rs/opusenc)
[![License: BSD-3-Clause](https://img.shields.io/crates/l/opusenc)](LICENSE)

<!-- cargo-rdme start -->

High-level bindings for
[libopusenc](https://opus-codec.org/docs/libopusenc_api-0.2/index.html).

## Example

```rust
let audio_data: Vec<i16> = {
    let mut file = std::fs::File::open("/dev/urandom")?;
    let mut buf = vec![0; 60 * 48_000 * 2 * 2];
    file.read_exact(&mut buf)?;
    buf.chunks_exact(2)
        .map(|a| i16::from_ne_bytes([a[0], a[1]]))
        .collect()
};

let mut encoder = Encoder::create_file(
    "/tmp/noise.opus",
    Comments::create()
        .add(RecommendedTag::Title, "Random Noise")?
        .add(RecommendedTag::Artist, "/dev/urandom")?,
    48_000,
    2,
    MappingFamily::MonoStereo,
)?;

encoder.write(&audio_data)?;
encoder.drain()?;

```

## Encoder options

This crate provides a `encoder-options` feature which enables reading and changing encoder options.

**Warning:** Some of these options might not work with opusenc, may be unsafe or even cause UB.
They are intended to be used via C macros that don't work with Rust.
Make sure to check if the methods you use match their intended behaviour.

<!-- cargo-rdme end -->

## License

This project is licensed under the BSD-3-Clause License.

See [LICENSE](LICENSE) for more information.
