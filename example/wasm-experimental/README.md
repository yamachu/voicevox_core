# WebAssembly (browser) example — experimental

VOICEVOX Core を `wasm32-unknown-emscripten` 向けにビルドし、ブラウザの Web Worker 上で音声合成するサンプルです。
Worker で Core を初期化した後、ページのテキストボックスに入力した任意の文字列を合成し、生成された WAV をメインスレッドに転送して `<audio>` で再生します。

## 公式 ort rc.15 の検証

この実験ブランチは [既存の wasm PoC](https://github.com/VOICEVOX/voicevox_core/pull/1447) の変更を、
[最新 main の検証時点のコミット](https://github.com/VOICEVOX/voicevox_core/commit/8b5d850fd8b2bd9f52ccefa882be12ebaf400552) に移植しています。
`ort` と `ort-sys` は crates.io の公式 `2.0.0-rc.15` を使い、ort の fork は使いません。
Environment の明示的な保持、Session 作成時の Environment 指定、Execution Provider の名前・feature、
非網羅的な TensorElementType への対応を行っています。
Open JTalk は既存 PoC と同じ wasm 対応 fork を引き続き使います。

Rust 1.96.0、Emscripten 4.0.8、ONNX Runtime 1.23.2 の静的ライブラリで、
以下の wasm ビルド（最終リンクを含む）とネイティブ側のコンパイルを確認しました。
ブラウザ上での音声合成の動作確認は別途必要です。

```sh
cargo +1.96.0 check --locked --package voicevox_core --features load-onnxruntime
cargo +1.96.0 tree --locked --package voicevox_core_wasm \
  --target wasm32-unknown-emscripten --invert ort
```

## 構成

| ファイル | 役割 |
| --- | --- |
| `index.html` | ログ表示と音声プレイヤー |
| `main.js` | Worker を起動し、入力されたテキストを送信、受け取った WAV を検証して再生 |
| `worker.js` | 辞書と `sample.vvm` を Emscripten FS に書き込んで Core を初期化し、メッセージごとに合成 |
| `wasm_browser_api_example.{js,wasm}` | ビルド成果物（手順 3 でコピー） |
| `sample.vvm` | 音声モデル（手順 4 で作成） |
| `open_jtalk_dic_utf_8-1.11/` | Open JTalk 辞書（手順 4 で配置） |

## 必要なもの

- Rust 1.96.0（`wasm32-unknown-emscripten` ターゲット）
- [Emscripten SDK](https://emscripten.org/docs/getting_started/downloads.html) 4.0.8
- WebAssembly 向けにビルドした ONNX Runtime（`libonnxruntime_webassembly.a`）

以下では次の変数を使います。

- `EMSDK_PATH`: emsdk のディレクトリ
- `VOICEVOX_CORE_PATH`: voicevox_core リポジトリのルート
- `ONNXRUNTIME_DOWNLOAD_DIR`: ONNX Runtime の静的ライブラリがあるディレクトリ

## 手順

### 1. ツールチェーンの準備

```sh
rustup toolchain install 1.96.0 --profile minimal --target wasm32-unknown-emscripten

cd "${EMSDK_PATH}"
./emsdk install 4.0.8
./emsdk activate 4.0.8
source ./emsdk_env.sh
```

### 2. ビルド

検証に使った静的ライブラリは [このビルド](https://github.com/yamachu/onnxruntime-builder/actions/runs/35830958810)
の `onnxruntime-wasm-static` artifact です（artifact の保持期限内のみダウンロード可能）。
`ONNXRUNTIME_DOWNLOAD_DIR` には展開されたパッケージの `lib` ディレクトリを指定します。

```sh
gh run download 35830958810 --repo yamachu/onnxruntime-builder \
  --name onnxruntime-wasm-static --dir ./onnxruntime-wasm-static
export ONNXRUNTIME_DOWNLOAD_DIR="$(pwd)/onnxruntime-wasm-static/onnxruntime-wasm-static-1.23.2/lib"
```

```sh
cd "${VOICEVOX_CORE_PATH}"

export CARGO_TARGET_WASM32_UNKNOWN_EMSCRIPTEN_LINKER=emcc
export CARGO_TARGET_WASM32_UNKNOWN_EMSCRIPTEN_RUSTFLAGS="\
  -C target-feature=+simd128 \
  -C link-arg=-sALLOW_MEMORY_GROWTH=1 \
  -C link-arg=-sINITIAL_MEMORY=1073741824 \
  -C link-arg=-sSTACK_SIZE=8388608 \
  -C link-arg=-sEXPORTED_RUNTIME_METHODS=FS,HEAPU8,ccall \
  -C link-arg=-sEXPORTED_FUNCTIONS=_main,_voicevox_browser_initialize,_voicevox_browser_synthesize,_voicevox_browser_wav_pointer,_voicevox_browser_wav_length \
  -L native=${ONNXRUNTIME_DOWNLOAD_DIR}"

cargo +1.96.0 build --locked \
  --package voicevox_core_wasm \
  --bin wasm_browser_api_example \
  --features link-onnxruntime \
  --target wasm32-unknown-emscripten \
  --profile c-api
```

### 3. 成果物のコピー

```sh
cp target/wasm32-unknown-emscripten/c-api/wasm_browser_api_example.{js,wasm} \
  example/wasm-experimental/
```

### 4. 音声モデルと辞書の用意

`model/sample.vvm` はディレクトリなので、中身を zip にまとめて配置します。

```sh
(cd model/sample.vvm && zip -r ../../example/wasm-experimental/sample.vvm .)
```

Open JTalk 辞書を `open_jtalk_dic_utf_8-1.11` ディレクトリごと配置します。
Worker は辞書の各ファイルを個別に `fetch` し、Emscripten FS の `/open_jtalk_dic_utf_8-1.11` に書き込みます。

### 5. 実行

Worker と `fetch` を使うため、`file://` ではなく HTTP サーバー経由で開きます。

```sh
cd example/wasm-experimental
python3 -m http.server 8000
```

<http://localhost:8000/> を開くと辞書とモデルの読み込みが始まり、完了すると「合成」ボタンが押せるようになります。
テキストと Style ID を指定して「合成」を押すと、完了後にプレイヤーが表示されます。
ブラウザの自動再生制限でブロックされた場合は、再生ボタンを押してください。

`body` の `data-status` 属性は `loading` → `ready` → (合成ごとに) `running` → `passed` / `failed` と変化するため、自動テストの判定に使えます。
