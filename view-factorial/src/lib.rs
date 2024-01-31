use wasm_bindgen::prelude::*;
use convert_base::Convert;

#[wasm_bindgen]
extern {
    pub fn alert(string: &str);
}

fn shift_slice_left(slice: &mut [u32]) -> bool {
    let mut last_carry = false;
    let mut next_carry;
    for num in slice.iter_mut().rev() {
        (*num, next_carry) = num.overflowing_shl(1);
        *num += last_carry as u32;
        last_carry = next_carry;
    }
    last_carry
}
fn shift_slices_left(left: &mut [u32], right: &mut [u32]) {
    shift_slice_left(left);
    left[left.len() - 1] |= shift_slice_left(right) as u32;
}

#[wasm_bindgen]
/*pub unsafe fn base256_to_string(base256: &[u8], base: u32) -> String {
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
}*/
pub unsafe fn base256_to_string(base256: &mut [u32], base: u32) -> String {
    let digit_groups = (32 * base256.len()).div_ceil(3).div_ceil(8);
    let mut digits = vec![0; digit_groups];

    alert(&format!("{digit_groups}"));

    for i in 0..base256.len() * 32 {
        if i % 100000 == 0 { alert(&format!("{}", i as f64 / base256.len() as f64 / 32f64)); }
        shift_slices_left(&mut digits, base256);

        // there are 8 digits in each digit group
        for digit_group in &mut digits {
            // the bitmask specifies the place in the digit group where the digit is
            let mut digit_bitmask = 0x1111;
            for i in 0..8 {
                // get the digit
                let digit = (*digit_group & digit_bitmask) >> (4 * i);
                // if digit >= 5 { digit += 3; }
                if digit >= 5 {
                    *digit_group += 5 << (4 * i);
                }
                // shift the bitmask to get the next digit
                digit_bitmask <<= 4;
            }
        }
    }

    alert("digits calculated!");

    let mut digits = unsafe {
        std::slice::from_raw_parts(
            digits.as_ptr() as *const u8,
            digits.len() * 8
        )
    };

    alert("conversion from u32 to u8 successful!");

    let Some(idx) = digits.iter().position(|&num| num != 0) else {
        return String::from("0");
    };

    alert(&format!("first non-zero index is {idx}!"));

    digits = unsafe {
        std::slice::from_raw_parts(
            digits.as_ptr().add(idx),
            digits.len() - idx
        )
    };

    alert("leading zeroes successfully removed!");

    String::from_utf8_unchecked(
        digits
            .iter()
            .map(|&digit| digit + if digit < 10 { 48 } else { 87 })
            .collect(),
    )
}
