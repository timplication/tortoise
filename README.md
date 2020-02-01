# Tortoise

![License](https://img.shields.io/github/license/timbaccaert/tortoise?style=flat-square)

A rule-based fractal generator based on turtle graphics &amp; Lindenmayer
systems.

## Screenshot

![Sierpiński's Triangle](https://raw.githubusercontent.com/tbaccaer/tortoise/master/img/sierpinski.png "Sierpiński's Triangle")
> Sierpiński's Triangle approximated using a [Sierpiński arrowhead curve L-system](https://en.wikipedia.org/wiki/Sierpi%C5%84ski_curve#Arrowhead_curve)

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

## License

Tortoise: A rule-based fractal generator based on turtle graphics & Lindenmayer systems.

Copyright (C) 2020 Tim Baccaert <[timbaccaert@protonmail.com](mailto:timbaccaert@protonmail.com)>

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published
by the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program. If not, see
<[https://www.gnu.org/licenses/](https://www.gnu.org/licenses/)>.
