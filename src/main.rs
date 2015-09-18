use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;

fn qwerty_to_dvorak_hash() -> HashMap<char, char> {
    let mut q_to_d = HashMap::new();
    q_to_d.insert('a', 'a');
    q_to_d.insert('b', 'x');
    q_to_d.insert('c', 'j');
    q_to_d.insert('d', 'e');
    q_to_d.insert('f', 'u');
    q_to_d.insert('g', 'i');
    q_to_d.insert('h', 'd');
    q_to_d.insert('i', 'c');
    q_to_d.insert('j', 'h');
    q_to_d.insert('k', 't');
    q_to_d.insert('l', 'n');
    q_to_d.insert('m', 'm');
    q_to_d.insert('n', 'b');
    q_to_d.insert('o', 'r');
    q_to_d.insert('p', 'l');
    q_to_d.insert('r', 'p');
    q_to_d.insert('s', 'o');
    q_to_d.insert('t', 'y');
    q_to_d.insert('u', 'g');
    q_to_d.insert('v', 'k');
    q_to_d.insert('x', 'q');
    q_to_d.insert('y', 'f');
    q_to_d
}

fn convert_words_to_dvorak(s: &str, dvorak_conversion_hash: &HashMap<char, char>) {
    let full_dict: Vec<&str> = s.split_whitespace().collect();
    for line in &full_dict {
        let lower = line.to_lowercase();
        if lower.contains("e") || lower.contains("q") || lower.contains("w")
        || lower.contains("z") {
            continue;
        }

        let converted = to_dvorak(&lower, dvorak_conversion_hash);
        let needle: &str = &converted;
        if full_dict.contains(&needle) {
            println!("qwerty: {} | dvorak: {}", lower, converted)
        }
    }
}

fn to_dvorak(qwerty: &str, conversion_hash: &HashMap<char, char>) -> String {
    let mut converted = String::from("");
    for c in String::from(qwerty).chars() {
        match conversion_hash.get(&c) {
            Some(dvorak) => converted.push(*dvorak),
            None => panic!("no conversion for {}", c)
        }
    }
    converted
}

fn main() {
    let path = Path::new("/usr/share/dict/words");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   Error::description(&why)),
        Ok(_) => {
            let q_to_d = qwerty_to_dvorak_hash();
            convert_words_to_dvorak(&s, &q_to_d)
        }
    }
}