use opusenc::{Comments, Encoder, MappingFamily, RecommendedTag};
use std::io::{Read, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let audio_data: Vec<i16> = {
        let mut file = std::fs::File::open("/dev/urandom")?;
        let mut buf = vec![0; 60 * 48_000 * 2 * 2];
        file.read_exact(&mut buf)?;
        buf.chunks_exact(2)
            .map(|a| i16::from_ne_bytes([a[0], a[1]]))
            .collect()
    };

    let mut encoder = Encoder::create_pull(
        Comments::create()
            .add(RecommendedTag::Title, "Random Noise")?
            .add(RecommendedTag::Artist, "/dev/urandom")?,
        48_000,
        2,
        MappingFamily::MonoStereo,
    )?;

    encoder.write(&audio_data)?;
    encoder.drain()?;

    let mut output_file = std::fs::File::create("/tmp/noise.opus")?;

    while let Some(page) = encoder.get_page(true) {
        output_file.write_all(page)?;
    }

    Ok(())
}
