const output = document.querySelector("#output");
const worker = new Worker(new URL("./worker.js", import.meta.url));

worker.addEventListener("message", ({ data }) => {
  if (data.type === "log") {
    output.textContent += `\n${data.text}`;
  } else if (data.type === "success") {
    document.body.dataset.status = "passed";
    output.textContent += `\n${data.text}`;
    worker.terminate();
  } else if (data.type === "error") {
    document.body.dataset.status = "failed";
    output.textContent += `\n${data.message}`;
    worker.terminate();
  }
});

worker.addEventListener("error", (event) => {
  document.body.dataset.status = "failed";
  output.textContent += `\nWorker error: ${event.message}`;
  worker.terminate();
});
