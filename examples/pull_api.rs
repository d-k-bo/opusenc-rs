use std::io::Write;

use hound::WavReader;
use opusenc::{Comments, Encoder, MappingFamily, RecommendedTag};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut wav = WavReader::open("examples/speech_orig.wav")?;

    let spec = wav.spec();
    assert_eq!(spec.channels, 1);
    assert_eq!(spec.sample_format, hound::SampleFormat::Int);
    assert_eq!(spec.bits_per_sample, 16);

    let audio_data = wav.samples::<i16>().collect::<hound::Result<Vec<i16>>>()?;

    let mut encoder = Encoder::create_pull(
        Comments::create()
            .add(RecommendedTag::Title, "Opus Speech Samples")?
            .add(RecommendedTag::Artist, "Various Artists")?,
        48_000,
        1,
        MappingFamily::MonoStereo,
    )?;

    let mut output_file = std::fs::File::create("/tmp/speech.opus")?;

    // let's simulate that we are reading the audio data from multiple buffers
    // as if we were reading it from a network stream
    for chunk in audio_data.chunks(audio_data.len() / 4) {
        encoder.write(chunk)?;

        while let Some(page) = encoder.get_page(true) {
            output_file.write_all(page)?;
        }
    }

    encoder.drain()?;

    while let Some(page) = encoder.get_page(true) {
        output_file.write_all(page)?;
    }

    Ok(())
}
