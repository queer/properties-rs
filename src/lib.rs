// #[cfg(test)]
// mod tests {
// #[test]
// fn it_works() {
// }
// }
// TODO: Tests

extern crate regex;

use std::collections::BTreeMap;
use std::io::{BufRead, BufReader};
use std::fs::File;

#[macro_export]
macro_rules! map(
    { $T: ident, $($key: expr => $ value: expr),* } => {
        {
            let mut m = $T::new();
            $(
                m.insert($key, $value);
            )*
            m
        }
    };
);

#[macro_export]
macro_rules! btreemap(
    { $($key: expr => $value: expr),* } => {
        {
            use std::collections::BTreeMap;

            map!(BTreeMap,
                $(
                    $key => $value
                )*
            )
        }
    };
);

// #[macro_export]
// macro_rules! hashmap(
// { $($key: expr => $value: expr),* } => {
// {
// use std::collections::HashMap;
//
// map!(HashMap,
// $(
// $key => $value
// )*
// )
// }
// };
// );

pub fn parse(file: &str) -> BTreeMap<String, Vec<String>> {
    let config_file = BufReader::new(File::open(file)
        .expect(format!("Opening {} failed!", file).as_str()));

    let mut map: std::collections::BTreeMap<String, Vec<String>> = btreemap!();

    // Process lines, add k/v pairs to map
    for x in config_file.lines() {
        match x {
            Ok(line) => {
                if line.starts_with("#") {
                    println!("[INFO] Found comment line, skipping...");
                    continue;
                }
                if line == "" {
                    println!("[INFO] Blank line, skipping...");
                    continue;
                }
                let mut split = line.splitn(2, "=");
                let key = split.next().unwrap().to_string();
                let value = split.next().unwrap().to_string();
                // Map each key to a vec of values
                if map.contains_key(&key) {
                    // Unwrap is safe because we already know it contains the key

                    map.get_mut(&key).unwrap().push(value);
                } else {
                    map.insert(key, vec![value]);
                }
            }
            Err(_) => {
                break;
            }
        }
    }

    // Valid key name filter
    let filter = regex::Regex::new("[/ \\[\\]\\{\\}<>,\\|\\*\\&\\^%\\(\\)=]").unwrap();
    // Processed raw lines, now verify validity
    for (key, value) in map.iter() {
        if filter.is_match(key) {
            panic!("[ERROR] Key '{}' contains invalid characters!", key);
        }
    }

    map
}

