# WebAssembly

Rust-template is capable of running as WebAssembly in the browser. This readme provides instructions to compile and host rust-template.

## Download

An archived WASM binary can be found on the [Releases](https://github.com/zaszi/rust-template/releases) page.

## Usage

Simply serving `rust-template_bg.wasm` and `rust-template.js` (found under WASM in [Releases](https://github.com/zaszi/rust-template/releases) or after building in the `pkg` directory) utilizing your web server of choice is sufficient to host rust-template. Now add a simple HTML script tag in your webpage:

```
<script type="module">
  import init from './rust-template.js';
  init('./rust-template_bg.wasm').then(function (wasm) { wasm.run(); });
</script>
```

## Building

If instead you wish to compile this yourself it is highly recommended to use [wasm-pack](https://github.com/rustwasm/wasm-pack).

Compilation is done as follows:

```
wasm-pack build --target web --release
```

The resulting files are found in the `pkg` directory. At minimum, you will need at least `rust-template_bg.wasm` and `rust-template.js`.
