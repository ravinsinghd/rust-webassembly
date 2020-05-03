mod utils;

use rand::Rng;
use std::collections::HashMap;
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

#[wasm_bindgen]
pub fn convert_ascii_to_unicode(data_lines: String) -> String {
    let mut output_string = String::new();
    let ascii_map = initiate_char_map();
    for line in data_lines.lines() {
        let mut temp = String::new();
        for char_in_line in line.chars() {
            let c_code = char_in_line as u8;
            //   println!("{}", c_code);
            let existing_map = ascii_map.get(&c_code);
            match existing_map {
                Some(tamil_map) => {
                    if tamil_map.is_combined == true && tamil_map.isPrefix == true {
                        for char_in_line in tamil_map.value.chars() {
                            temp.push(char_in_line);
                        }
                    // println!("is_combined {} {}", true, temp)
                    } else {
                        if temp.len() > 0 {
                            // println!("temp {} ", temp);
                            // println!("output_string {} ", output_string);
                            for char_in_line in tamil_map.value.chars() {
                                output_string.push(char_in_line);
                            }
                            // println!("temp {} ", temp);
                            // println!("output_string {} ", output_string);
                            for char_in_line in temp.chars() {
                                output_string.push(char_in_line);
                            }
                            // println!("temp {} ", temp);
                            // println!("output_string {} ", output_string);
                            temp.clear();
                        } else {
                            for char_in_line in tamil_map.value.chars() {
                                output_string.push(char_in_line);
                            }
                        }
                    }
                }
                None => {}
            }
        }
        // println!("{}", output_string);
    }
    return output_string;
}

pub struct Tamil {
    is_combined: bool,
    value: String,
    isPrefix: bool,
}
pub fn initiate_char_map() -> HashMap<u8, Tamil> {
    let mut ascii_map: HashMap<u8, Tamil> = HashMap::new();
    ascii_map.insert(
        1,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        2,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        3,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        4,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        5,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        6,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        7,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        8,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        9,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        10,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        11,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        12,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        13,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        14,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        15,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        16,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        17,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        18,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        19,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        20,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        21,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        22,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        23,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        24,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        25,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        26,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        27,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        28,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        29,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        30,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        31,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        32,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: " ".to_string(),
        },
    );
    ascii_map.insert(
        33,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        34,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        35,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        36,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        37,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        38,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        39,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        40,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "(".to_string(),
        },
    );
    ascii_map.insert(
        41,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: ")".to_string(),
        },
    );
    ascii_map.insert(
        42,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        43,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        44,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "இ".to_string(),
        },
    );
    ascii_map.insert(
        45,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        46,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: ".".to_string(),
        },
    );
    ascii_map.insert(
        47,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        48,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "0".to_string(),
        },
    );
    ascii_map.insert(
        49,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "1".to_string(),
        },
    );
    ascii_map.insert(
        50,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "2".to_string(),
        },
    );
    ascii_map.insert(
        51,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "3".to_string(),
        },
    );
    ascii_map.insert(
        52,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "4".to_string(),
        },
    );
    ascii_map.insert(
        53,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "5".to_string(),
        },
    );
    ascii_map.insert(
        54,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "6".to_string(),
        },
    );
    ascii_map.insert(
        55,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "7".to_string(),
        },
    );
    ascii_map.insert(
        56,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "8".to_string(),
        },
    );
    ascii_map.insert(
        57,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "9".to_string(),
        },
    );
    ascii_map.insert(
        58,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        59,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "்".to_string(),
        },
    );
    ascii_map.insert(
        60,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        61,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        62,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: ",".to_string(),
        },
    );
    ascii_map.insert(
        63,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        64,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        65,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "யு".to_string(),
        },
    );
    ascii_map.insert(
        66,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "டீ".to_string(),
        },
    );
    ascii_map.insert(
        67,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "ஊ".to_string(),
        },
    );
    ascii_map.insert(
        68,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "னு".to_string(),
        },
    );
    ascii_map.insert(
        69,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "நு".to_string(),
        },
    );
    ascii_map.insert(
        70,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "கு".to_string(),
        },
    );
    ascii_map.insert(
        71,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "பு".to_string(),
        },
    );
    ascii_map.insert(
        72,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "ர்".to_string(),
        },
    );
    ascii_map.insert(
        73,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "ஜ".to_string(),
        },
    );
    ascii_map.insert(
        74,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "து".to_string(),
        },
    );
    ascii_map.insert(
        75,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "மு".to_string(),
        },
    );
    ascii_map.insert(
        76,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "டு".to_string(),
        },
    );
    ascii_map.insert(
        77,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "ஆ".to_string(),
        },
    );
    ascii_map.insert(
        78,
        Tamil {
            isPrefix: true,
            is_combined: true,
            value: "ே".to_string(),
        },
    );
    ascii_map.insert(
        79,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "மு".to_string(),
        },
    );
    ascii_map.insert(
        80,
        Tamil {
            isPrefix: false,
            is_combined: true,
            value: "ீ".to_string(),
        },
    );
    ascii_map.insert(
        81,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "ஞ".to_string(),
        },
    );
    ascii_map.insert(
        82,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "சு".to_string(),
        },
    );
    ascii_map.insert(
        83,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "ளு".to_string(),
        },
    );
    ascii_map.insert(
        84,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "வு".to_string(),
        },
    );
    ascii_map.insert(
        85,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "ரு".to_string(),
        },
    );
    ascii_map.insert(
        86,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "ஏ".to_string(),
        },
    );
    ascii_map.insert(
        87,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "று".to_string(),
        },
    );
    ascii_map.insert(
        88,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "ஓ".to_string(),
        },
    );
    ascii_map.insert(
        89,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "லு".to_string(),
        },
    );
    ascii_map.insert(
        90,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "ணு".to_string(),
        },
    );
    ascii_map.insert(
        91,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        92,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        93,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        94,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        95,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        96,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        97,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "ய".to_string(),
        },
    );
    ascii_map.insert(
        98,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "டி".to_string(),
        },
    );
    ascii_map.insert(
        99,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "உ".to_string(),
        },
    );
    ascii_map.insert(
        100,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "ன".to_string(),
        },
    );
    ascii_map.insert(
        101,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "ந".to_string(),
        },
    );
    ascii_map.insert(
        102,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "க".to_string(),
        },
    );
    ascii_map.insert(
        103,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "ப".to_string(),
        },
    );
    ascii_map.insert(
        104,
        Tamil {
            isPrefix: false,
            is_combined: true,
            value: "ா".to_string(),
        },
    );
    ascii_map.insert(
        105,
        Tamil {
            isPrefix: true,
            is_combined: true,
            value: "ை".to_string(),
        },
    );
    ascii_map.insert(
        106,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "த".to_string(),
        },
    );
    ascii_map.insert(
        107,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "ம".to_string(),
        },
    );
    ascii_map.insert(
        108,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "ட".to_string(),
        },
    );
    ascii_map.insert(
        109,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        110,
        Tamil {
            isPrefix: true,
            is_combined: true,
            value: "ெ".to_string(),
        },
    );
    ascii_map.insert(
        111,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "ழ".to_string(),
        },
    );
    ascii_map.insert(
        112,
        Tamil {
            isPrefix: false,
            is_combined: true,
            value: "ி".to_string(),
        },
    );
    ascii_map.insert(
        113,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "ங".to_string(),
        },
    );
    ascii_map.insert(
        114,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "ச".to_string(),
        },
    );
    ascii_map.insert(
        115,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "ள".to_string(),
        },
    );
    ascii_map.insert(
        116,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "வ".to_string(),
        },
    );
    ascii_map.insert(
        117,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "ர".to_string(),
        },
    );
    ascii_map.insert(
        118,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "எ".to_string(),
        },
    );
    ascii_map.insert(
        119,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "ற".to_string(),
        },
    );
    ascii_map.insert(
        120,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "ஒ".to_string(),
        },
    );
    ascii_map.insert(
        121,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "ல".to_string(),
        },
    );
    ascii_map.insert(
        122,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "ண".to_string(),
        },
    );
    ascii_map.insert(
        123,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        124,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        125,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        126,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        127,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        128,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        129,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        130,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        131,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        132,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        133,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        134,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        135,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        136,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        137,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        138,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        139,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        140,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        141,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        142,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        143,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        144,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        145,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        146,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        147,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        148,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        149,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        150,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        151,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        152,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        153,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        154,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        155,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        156,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        157,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        158,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        159,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        160,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        161,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        162,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        163,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        164,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        165,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        166,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        167,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        168,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        169,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        170,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        171,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        172,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        173,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        174,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        175,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        176,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        177,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        178,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        179,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        180,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        181,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        182,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        183,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        184,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        185,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        186,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        187,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        188,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        189,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        190,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        191,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        192,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        193,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        194,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        195,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        196,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        197,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        198,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        199,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        200,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        201,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        202,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        203,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        204,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        205,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        206,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        207,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        208,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        209,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        210,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        211,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        212,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        213,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        214,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        215,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        216,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        217,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        218,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        219,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        220,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        221,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        222,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        223,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        224,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        225,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        226,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        227,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        228,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        229,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        230,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        231,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        232,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        233,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        234,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        235,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        236,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        237,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        238,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        239,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        240,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        241,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        242,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        243,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        244,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        245,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        246,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        247,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        248,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        249,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        250,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        251,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        252,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        253,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        254,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    ascii_map.insert(
        255,
        Tamil {
            isPrefix: false,
            is_combined: false,
            value: "அ".to_string(),
        },
    );
    return ascii_map;
}
