#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn fix16_from_int(a: i32) -> fix16_t {
    a * fix16_one
}

pub fn fix16_to_int(a: fix16_t) -> i32 {
    if a >= 0 {
        (a + (fix16_one >> 1)) / fix16_one
    } else {
        (a - (fix16_one >> 1)) / fix16_one
    }
}

pub fn fix16_from_float(a: f32) -> fix16_t {
    let temp = a * fix16_one as f32;
    if temp.is_sign_positive() {
        (temp + 0.5) as fix16_t
    } else {
        (temp - 0.5) as fix16_t
    }
}

pub fn fix16_to_float(a: fix16_t) -> f32 {
    a as f32 / fix16_one as f32
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let x = fix16_from_int(0xBB);
        let y = fix16_from_float(2934.02234);
        unsafe {
            assert_eq!(fix16_to_int(fix16_add(x, fix16_one)), 0xBC);
            assert_eq!(fix16_to_float(fix16_add(y, fix16_one)), 2935.02234);
        }
    }
}
