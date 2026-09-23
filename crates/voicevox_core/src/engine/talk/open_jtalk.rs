#[cfg(all(target_os = "emscripten", feature = "wasm-open-jtalk-link-smoke"))]
include!("open_jtalk_wasm.rs");

#[cfg(not(target_os = "emscripten"))]
include!("open_jtalk_native.rs");

#[cfg(all(target_os = "emscripten", not(feature = "wasm-open-jtalk-link-smoke")))]
compile_error!("Emscripten builds currently require the wasm-open-jtalk-link-smoke feature.");
