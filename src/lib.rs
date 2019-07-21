mod utils;

use rand::Rng;
use wasm_bindgen::prelude::*;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn greet(mut number_of_elements: i32) -> Vec<i32> {
    let mut data = Vec::new();
    while number_of_elements > 0 {
        let random_value = rand::thread_rng().gen_range(1, 101);
        data.push(random_value);
        number_of_elements = number_of_elements - 1;
    }
    return data;
}