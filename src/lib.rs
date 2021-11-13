use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(x: u32, y: u32) -> u32 {
    x + y
}

#[cfg(test)]
mod tests {}
