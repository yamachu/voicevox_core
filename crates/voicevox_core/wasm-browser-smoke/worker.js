const successMarker = "BROWSER_ONNX_SMOKE_OK";

globalThis.Module = {
  arguments: ["/sample.onnx"],
  locateFile: (path, prefix) => new URL(path, prefix || self.location.href).href,
  print: (...values) => {
    const text = values.join(" ");
    self.postMessage({ type: "log", text });
    if (text.includes(successMarker)) {
      self.postMessage({ type: "success", text });
    }
  },
  printErr: (...values) =>
    self.postMessage({ type: "error", message: values.join(" ") }),
  onAbort: (reason) =>
    self.postMessage({ type: "error", message: `Emscripten aborted: ${reason}` }),
};

importScripts("./wasm_onnx_smoke.js");
