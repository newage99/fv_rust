use std::num::ParseIntError;
use std::str;

pub fn make_even(mut s: String) -> String {
    if s.len() % 2 == 1 {
        s = "0".to_string() + &s.to_string();
    }
    return s;
}

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub fn output(hex: String, mut i: i32) -> i32 {
    match &decode_hex(&hex) {
        Ok(dh) => match str::from_utf8(dh) {
            Ok(v) => {
                if format!("{:?}", v).len() < 7 {
                    if i % 10 == 0 {
                        println!("{:?} {:?} \t", hex, v);
                    } else {
                        print!("{:?} {:?} \t", hex, v);
                    }
                    i += 1;
                }
            }
            _ => {}
        },
        _ => {}
    }

    return i;
}

