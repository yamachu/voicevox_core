import assert from "node:assert/strict";
import { createServer } from "node:http";
import { readFile, stat } from "node:fs/promises";
import { extname, resolve, sep } from "node:path";
import { pathToFileURL } from "node:url";

const root = resolve(process.argv[2] ?? ".");
const playwrightModule = process.env.PLAYWRIGHT_MODULE;
assert(playwrightModule, "PLAYWRIGHT_MODULE must point to the installed Playwright module");
const { chromium } = await import(pathToFileURL(playwrightModule).href);

const contentTypes = new Map([
  [".data", "application/octet-stream"],
  [".html", "text/html; charset=utf-8"],
  [".js", "text/javascript; charset=utf-8"],
  [".wasm", "application/wasm"],
]);

const server = createServer(async (request, response) => {
  try {
    const pathname = decodeURIComponent(new URL(request.url, "http://localhost").pathname);
    const file = resolve(root, `.${pathname}`);
    assert(file === root || file.startsWith(`${root}${sep}`), "invalid request path");
    await stat(file);
    response.writeHead(200, {
      "content-type": contentTypes.get(extname(file)) ?? "application/octet-stream",
    });
    response.end(await readFile(file));
  } catch (error) {
    if (error && typeof error === "object" && "code" in error && error.code === "ENOENT") {
      response.writeHead(404);
      response.end("Not found");
      return;
    }
    console.error(error);
    response.writeHead(500);
    response.end("Internal server error");
  }
});

await new Promise((resolveListen) => server.listen(0, "127.0.0.1", resolveListen));
const address = server.address();
assert(address && typeof address === "object");

const browser = await chromium.launch({ args: ["--no-sandbox"] });
try {
  const page = await browser.newPage();
  const pageErrors = [];
  page.on("pageerror", (error) => pageErrors.push(error.message));
  await page.goto(`http://127.0.0.1:${address.port}/index.html`);
  await page.waitForFunction(
    () => ["passed", "failed"].includes(document.body.dataset.status),
    null,
    { timeout: 120_000 },
  );
  assert.equal(
    await page.locator("body").getAttribute("data-status"),
    "passed",
    `browser smoke failed: ${await page.locator("#output").textContent()}`,
  );
  assert.deepEqual(pageErrors, [], "browser reported uncaught page errors");
  console.log(await page.locator("#output").textContent());
} finally {
  await browser.close();
  await new Promise((resolveClose, rejectClose) =>
    server.close((error) => (error ? rejectClose(error) : resolveClose())),
  );
}
