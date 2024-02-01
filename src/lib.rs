// MIT License
// Copyright (c) 2024 largenumbergoeshere


use wasm_bindgen::prelude::*;

// the entrypoint called by wasm_bindgen.
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Make panics work as the should
    std::panic::set_hook(Box::new(|panic_info| {
        let string = format!("{}", panic_info);
        web_sys::wasm_bindgen::throw_str(&string);
    }));

    // run the useful part, propigating errors if needed
    main()?;

    // exit gracefully
    Ok(())
}

/// The main function. do whatever you want here... 
/// Be are that some functions in `std` and `core` will not work as expected or at all. 
///  Some notable examples: `std::thread::sleep` and `println!` - test everything!
fn main() -> Result<(), JsValue> {
    // have an example of the included panic handling
    add_html_to_window("hello world!")?;
    
    panic!("this is an error!");
}

fn add_html_to_window(message: &str) -> Result<(), JsValue> {
    let window = web_sys::window().expect("window");
    let document = window.document().expect("document in window");
    let body = document.body().expect("body in document");

    let val = document.create_element("p")?;
    val.set_inner_html(message);
    body.append_child(&val)?;

    Ok(())
}

