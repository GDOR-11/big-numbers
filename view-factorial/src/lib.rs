use wasm_bindgen::prelude::*;
use convert_base::Convert;

#[wasm_bindgen]
extern {
    pub fn alert(string: &str);
}

#[wasm_bindgen]
pub unsafe fn base256_to_string(base256: &[u8], base: u32) -> String {
    alert("function called");
    let mut convert = Convert::new(256, base as u64);
    alert("base converter created");
    let mut custom_base: Vec<u8> = convert.convert(base256);
    alert("base conversion done");
    for digit in &mut custom_base {
        *digit += if *digit < 10 { 48 } else { 87 };
    }
    alert("finished mapping from digit to ASCII representation");
    String::from_utf8_unchecked(custom_base)
}
