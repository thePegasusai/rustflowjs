# you can use the wasm-bindgen crate to create a Rust function that can be called from JavaScript. Here's an example of a Rust function that takes a Tensor and returns its sum:

use wasm_bindgen::prelude::*;
use js_sys::Float32Array;

#[wasm_bindgen]
pub fn sum(tensor: &Float32Array) -> f32 {
    let mut sum = 0.0;
    for i in 0..tensor.length() {
        sum += tensor.get_index(i).unwrap();
    }
    sum
}
