# Rust Raw Webassembly Template
A template for writing minimal wasm in rust. 
No frameworks, no packages, no javascript, no nodejs, no NPM... no nothing, except the bare minumum plumbing to compile and load rust inside a webpage. This is ***not batteries included***, you will have to do everything manually.

Inspired by https://tung.github.io/posts/rust-and-webassembly-without-a-bundler/

### Setup:
1. Dependencies:
- A broweser
- [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [cargo-generate](https://github.com/cargo-generate/cargo-generate)
- wasm-bindgen-cli - `cargo install wasm-bindgen-cli`
2. Use [cargo generate](https://github.com/cargo-generate/cargo-generate?tab=readme-ov-file) on this repository to copy this to your computer
3. Important: Rename the project in cargo.toml, ***this template will ingore any information you give to cargo-generate***, you are welcome to make your own that does so automatically if you want this behaviour to change.
4. Create your own script/infastructure to compile/deploy your project. An example powershell script is included `test.ps1` to base yours off. It can be used as-is but, however it may not work for your configutation - no guarentees are made.
Here's some important commands you may need to use:
- `cargo +nightly install wasm-bindgen-cli`
- `cargo build --release --target=wasm32-unknown-unknown`
- `wasm-bindgen target/wasm32-unknown-unknown/release/no_modules.wasm --out-dir .  --target web --no-typescript`
You will need to update them accordingly if you change folder structure, give the project a different name. You may also need to update `index.html` file changes.
5. Be aware of the MIT licence bellow. You may be legally obligated to correctly attribute this template, especially if you're using it as part of a commercial product.

### Strongly reccomended:
- Update or delete the `README.md` and to make your project look more professional
- Update `LICENCE`
- Update `.gitignore`
- Write in the main function
- Be are that some important standard functions and macros will not work as expected or at all! Some notable examples: `std::thread::sleep` and `println!` - test everything and make typesafe abstractions!
- Understand why it works and create your own
- Update the project - the source contains ``

### Somewhat-Technical explanation
At time of writing, the `wasm-bindgen` documentation contains outdated and incomplete information on correctly using it to create a project with minimal dependencies. After a couple days of searching and tweaking, I was able to finally work out how to use rust within web assembly without any framework or npm nonsense. The biggest light-bulb moment came from the article '[Rust and web assembly without a bundler](https://tung.github.io/posts/rust-and-webassembly-without-a-bundler/)' on 'Tung's Word Box'
This template is the outcome of this tinkering. I felt like it would be selfish to not throw it out into the world. Even if it saves one person a little headache, it has served its purpose. If not, I'm sure I'll be using it myself anyway.

The included libraries and tools are very important.
- Wasm-bindgen-cli generates 'glue' to connect the browser's javascript-first engine to the wasm runtime. It also builds on the minimal [type system of wasm](https://webassembly.github.io/spec/core/syntax/types.html) This allows you to pass information to and from javascript functions without having to manually convert everything non-trivial to and from bytes.
- wasm-bingen is the dependency required in rust projects to use wasm-bindgen-cli tool.
- Web-sys contains bindings to access the browser's builtin javascript functions. It has almost zero type safety, but it does mean you have access to the bare minumum to start manipulating the [DOM](https://en.wikipedia.org/wiki/Document_Object_Model) to start making sensible 'rustic' abstractions.

This project also includes a single html file in `templates` called `index.html`. This is the web-page that will run the generated wasm. It contains a simple script to fetch and run wasm's generated javascript wrapper library and display clear error indicators if anything fails.


## LICENCE:
```
MIT License

Copyright (c) 2024 largenumbergoeshere

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```