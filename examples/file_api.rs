use hound::WavReader;
use opusenc::{Comments, Encoder, MappingFamily, RecommendedTag};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut wav = WavReader::open("examples/speech_orig.wav")?;

    let spec = wav.spec();
    assert_eq!(spec.channels, 1);
    assert_eq!(spec.sample_format, hound::SampleFormat::Int);
    assert_eq!(spec.bits_per_sample, 16);

    let audio_data = wav.samples::<i16>().collect::<hound::Result<Vec<i16>>>()?;

    let mut encoder = Encoder::create_file(
        "/tmp/speech.opus",
        Comments::create()
            .add(RecommendedTag::Title, "Opus Speech Samples")?
            .add(RecommendedTag::Artist, "Various Artists")?,
        48_000,
        1,
        MappingFamily::MonoStereo,
    )?;

    encoder.write(&audio_data)?;
    encoder.drain()?;

    Ok(())
}
