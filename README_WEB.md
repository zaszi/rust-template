# WebAssembly

Rust-template is capable of running as WebAssembly in the browser. This readme provides instructions to compile and host rust-template.

## Download

An archived WASM binary can be found on the [Releases](https://github.com/zaszi/rust-template/releases) page.

## Usage

Simply serving `rust-template_bg.wasm` and `rust-template.js` (found under WASM in [Releases](https://github.com/zaszi/rust-template/releases) or after building in the `pkg` directory) utilizing your web server of choice is sufficient to host rust-template. You will still need to make a single call with javascript in your web page as follows:

```
import init from './rust-template.js';
init('./rust-template_bg.wasm').then(function (wasm) { wasm.run(); });
```

Which can then be called from within a simple HTML canvas:

```
<canvas id="canvas" width="640" height="480"></canvas>
<script type="module" src="run.js"></script>
```

## Building

If instead you wish to compile this yourself it is highly recommended to use [wasm-pack](https://github.com/rustwasm/wasm-pack).

Compilation is done as follows:

```
wasm-pack build --target web --release
```

The resulting files are found in the `pkg` directory. At minimum, you will need at least `rust-template_bg.wasm` and `rust-template.js`.
