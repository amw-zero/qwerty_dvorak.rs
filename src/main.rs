use std::collections::HashSet;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::str;

fn open_dict() -> File {
    let path = "/usr/share/dict/words";
    match File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("couldn't open {}: {}", path, Error::description(&e))
    }
}

fn parse_words(dict_contents: &[u8]) -> Vec<&[u8]> {
    dict_contents.split(|&byte| byte == b'\n').collect()
}

fn main() {
    // Parse dictionary file
    let mut dict_file = open_dict();

    let file_size = dict_file.metadata().unwrap().len();
    let mut dict_contents = Vec::with_capacity(file_size as usize);
    dict_file.read_to_end(&mut dict_contents).expect("couldn't read dict file");

    let words = parse_words(&dict_contents);

    // Words with e, q, w, and z will be invalid when converted
    // to Dvorak because those positions are special characters
    let valid_words: Vec<&[u8]> = words.iter().filter(|word|
        !word.iter().any(|c|
            *c == b'q' || *c == b'Q' || *c == b'w' || *c == b'W' ||
            *c == b'e' || *c == b'E' || *c == b'z' || *c == b'Z'
        )
    ).cloned().collect();

    let index: HashSet<&[u8]> = words.iter().cloned().collect();

    // Convert words to Dvorak and see if the converted word is still
    // in the dictionary
    for word in valid_words {
        let converted: Vec<u8> = word.iter().cloned().map(qd_map).collect();
        if index.contains(&*converted) {
            unsafe {
                println!("{} -> {}", str::from_utf8_unchecked(&word),
                                     str::from_utf8_unchecked(&converted));
            }
        }
    }
}

#[inline]
fn qd_map(b: u8) -> u8 { QD_MAP[b as usize] }

static QD_MAP: [u8; 256] = [
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, // 00-0F
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, // 10-1F
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, // 20-2F
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, // 30-3F
    0,    // @
    b'A', // A
    b'X', // B
    b'J', // C
    b'E', // D
    0,    // E
    b'U', // F
    b'I', // G
    b'D', // H
    b'C', // I
    b'H', // J
    b'T', // K
    b'N', // L
    b'M', // M
    b'B', // N
    b'R', // O
    b'L', // P
    0,    // Q
    b'P', // R
    b'O', // S
    b'Y', // T
    b'G', // U
    b'K', // V
    0,    // W
    b'Q', // X
    b'F', // Y
    0,    // Z
    0,    // [
    0,    // \
    0,    // ]
    0,    // ^
    0,    // _
    0,    // `
    b'a', // a
    b'x', // b
    b'j', // c
    b'e', // d
    0,    // e
    b'u', // f
    b'i', // g
    b'd', // h
    b'c', // i
    b'h', // j
    b't', // k
    b'n', // l
    b'm', // m
    b'b', // n
    b'r', // o
    b'l', // p
    0,    // q
    b'p', // r
    b'o', // s
    b'y', // t
    b'g', // u
    b'k', // v
    0,    // w
    b'q', // x
    b'f', // y
    0,    // z
    0,    // )
    0,    // |
    0,    // }
    0,    // -
    0,    // 7F
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, // 80-8F
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, // 90-9F
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, // A0-AF
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, // B0-BF
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, // C0-CF
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, // D0-DF
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, // E0-EF
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, // F0-FF
];
