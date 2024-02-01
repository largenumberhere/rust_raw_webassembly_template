# Raw rust Web-Assembly template
A template for writing minimal wasm in rust. 
No frameworks, no packages, no javascript, no nodejs, no nothing except some plumbing to compile and load rust inside a webpage. This is ***not batteries included***, you will have to do everything manually.

Contains wasm-bingen to build on wasm's extremely minimal type system, faucilitate passing object to and from javascript and web-sys for access to the browser's builtin javascript functions. It has minimal type safety but it does mean you have access to the bare minumum to start manipulating the [DOM](https://en.wikipedia.org/wiki/Document_Object_Model) to start making sensible 'rustic' abstractions.
Inspired by https://tung.github.io/posts/rust-and-webassembly-without-a-bundler/

### Setup:
0. Dependencies:
- A broweser
- cargo
- cargo-generate
- wasm-bindgen-cli
1. Use [cargo generate](https://github.com/cargo-generate/cargo-generate?tab=readme-ov-file) on this repository to copy this to your computer
2. Important: Rename the project in cargo.toml, ***this template will ingore any information you give to cargo-generate***, you are welcome to make your own that does so automatically if you want this behaviour to change.
3. Create your own script/infastructure to compile/deploy your project. An example powershell script is included `test.ps1` to base yours off. It can be used as-is but, however it may not work for your configutation - no guarentees are made.
Here's some important commands you may need to use:
- `cargo +nightly install wasm-bindgen-cli`
- `cargo build --release --target=wasm32-unknown-unknown`
- `wasm-bindgen target/wasm32-unknown-unknown/release/no_modules.wasm --out-dir .  --target web --no-typescript`


4. Be aware of the MIT licence bellow. You may be legally obligated to correctly attribute this template, especially if you're using it as part of a commercial product.

Strongly reccomended:
- Update or delete the `README.md` and to make your project look more professional
- Update `LICENCE`
- Update `.gitignore`
- Write in the main function
- Be are that some important standard functions and macros will not work as expected or at all! Some notable examples: `std::thread::sleep` and `println!` - test everything and make typesafe abstractions! 

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