use wasm_bindgen::prelude::*; 

///Initialize WASM (optional setup)
#[wasm_bindgen(start)]
pub fn main_js() {
    // This runs automatically when loaded in JS
}

///Simple function to test Rust <-> JS
#[wasm_bindgen]
pub fn suggest_discard(hand: &str) -> String {
    format!("Best discard for hand '{}': 5-man", hand)
}
