use voicevox_core::blocking::Onnxruntime;

#[unsafe(no_mangle)]
pub extern "C" fn voicevox_browser_initialize_runtime() -> i32 {
    match Onnxruntime::init_once() {
        Ok(_) => 0,
        Err(error) => {
            eprintln!("{error:#}");
            1
        }
    }
}

fn main() {}
