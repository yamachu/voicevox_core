use std::{mem::MaybeUninit, ptr::NonNull};

use voicevox_core_c_api::{VoicevoxOnnxruntime, voicevox_onnxruntime_init_once};

#[unsafe(no_mangle)]
pub extern "C" fn voicevox_browser_initialize_runtime() -> i32 {
    let mut runtime = MaybeUninit::<&'static VoicevoxOnnxruntime>::uninit();
    let output = NonNull::new(runtime.as_mut_ptr()).expect("MaybeUninit pointer is non-null");
    let result = unsafe { voicevox_onnxruntime_init_once(output) };
    if result as i32 == 0 {
        unsafe {
            runtime.assume_init();
        }
    }
    result as i32
}

fn main() {}
