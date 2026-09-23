use voicevox_core::{
    AccelerationMode, StyleId,
    blocking::{Onnxruntime, OpenJtalk, Synthesizer, VoiceModelFile},
};

fn main() -> anyhow::Result<()> {
    let model_path = std::env::args_os()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("pass the sample ONNX path as the first argument"))?;
    let runtime = Onnxruntime::init_once()?;
    let synthesizer = Synthesizer::builder(runtime)
        .text_analyzer(OpenJtalk::new("")?)
        .acceleration_mode(AccelerationMode::Cpu)
        .cpu_num_threads(1)
        .build()?;
    let voice_model = VoiceModelFile::open(model_path)?;
    synthesizer.load_voice_model(&voice_model).perform()?;
    let wav = synthesizer
        .tts("これはテストです", StyleId::new(302))
        .perform()?;
    anyhow::ensure!(
        wav.starts_with(b"RIFF") && wav.get(8..12) == Some(b"WAVE"),
        "synthesis did not produce WAV audio"
    );

    println!(
        "Synthesized the fixed-label sample into {} bytes of WAV audio.",
        wav.len()
    );
    Ok(())
}
