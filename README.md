# Tortoise

A rule-based fractal generator based on turtle graphics &amp; Lindenmayer
systems.

## Building

This project compiles to a WebAssembly target using Rust's
[wasm-bindgen](https://crates.io/crates/wasm-bindgen) crate. For more
information, please check the domain working group's
website [Rust and WebAssembly](https://rustwasm.github.io/). Please make sure to
install the correct dependencies first, namely the Rust toolchain and `wasm-pack`.

In order to compile Rust to WebAssembly so it can be used as a module, execute the
following command in the project's directory.

```bash
wasm-pack build --target=web
```

It is easy to use a utility like [Live Server](https://www.npmjs.com/package/live-server)
during development, to avoid CORS issues and manual reloads.

```bash
live-server --watch=pkg,index.html
```

