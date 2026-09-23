use voicevox_core::blocking::Onnxruntime;

fn main() -> anyhow::Result<()> {
    let model_path = std::env::args_os()
        .nth(1)
        .unwrap_or_else(|| "/sample.onnx".into());

    Onnxruntime::init_once()?;
    let model = std::fs::read(model_path)?;
    let session = ort::session::Session::builder()?.commit_from_memory(&model)?;
    anyhow::ensure!(
        !session.inputs().is_empty() && !session.outputs().is_empty(),
        "the ONNX model has no inputs or outputs"
    );

    println!(
        "BROWSER_ONNX_SMOKE_OK: {} inputs, {} outputs",
        session.inputs().len(),
        session.outputs().len()
    );
    Ok(())
}
