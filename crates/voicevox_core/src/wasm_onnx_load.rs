fn main() -> anyhow::Result<()> {
    let model_path = std::env::var_os("VOICEVOX_SAMPLE_ONNX")
        .ok_or_else(|| anyhow::anyhow!("VOICEVOX_SAMPLE_ONNX must point to a sample ONNX model"))?;
    let model_bytes = std::fs::read(model_path)?;
    let (input_count, output_count) = voicevox_core::wasm_onnx_load_smoke::load_model(model_bytes)?;
    anyhow::ensure!(input_count > 0, "the sample ONNX model has no inputs");
    anyhow::ensure!(output_count > 0, "the sample ONNX model has no outputs");

    println!("Loaded sample ONNX with {input_count} inputs and {output_count} outputs.");
    Ok(())
}
