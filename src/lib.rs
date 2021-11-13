use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn muh_len(input: &[u8]) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {}
