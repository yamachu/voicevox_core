const successMarker = "BROWSER_CORE_ABI_OK";

globalThis.Module = {
  noInitialRun: true,
  locateFile: (path, prefix) => new URL(path, prefix || self.location.href).href,
  onRuntimeInitialized: () => {
    try {
      const resultCode = Module._voicevox_browser_initialize_runtime();
      if (resultCode !== 0) {
        throw new Error(`ONNX Runtime initialization returned ${resultCode}`);
      }
      self.postMessage({
        type: "success",
        text: `${successMarker}: Core initialized ONNX Runtime from JavaScript`,
      });
    } catch (error) {
      self.postMessage({ type: "error", message: error.message });
    }
  },
  printErr: (...values) =>
    self.postMessage({ type: "error", message: values.join(" ") }),
  onAbort: (reason) =>
    self.postMessage({ type: "error", message: `Emscripten aborted: ${reason}` }),
};

importScripts("./wasm_browser_api_smoke.js");
