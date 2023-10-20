// Using wasm-bindgen to communicate between Rust and JavaScript
// wasm-bindgem provide a bridge between the types of JavaCript and Rust
use wasm_bindgen::prelude::*;


// Calling external functions in JavaScript from Rust
#[wasm_bindgen] // wasm_bindgen will get the functions
extern {
    pub fn alert(s: &str); // importing the alert function with one string argument 's'
}


#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
