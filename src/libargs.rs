#![allow(dead_code)]
use std::env;
pub fn args() -> Vec<String>{
    // Import every argument
    let allargs:Vec<String> = env::args().collect();
    allargs
}

pub fn swcs() -> (Vec<String>, Vec<String>) {
    let a = args();
    // Create mutable vector which will store switches and values later
    let mut s = vec![];
    let mut v = vec![];
    // Go trough every argument
    let mut index = 1;
    while index < a.len() {
        // If it starts with '-', argument is a 'switch'
        let chars = &a[index].chars().collect::<Vec<char>>();
        if chars[0] == '-' {
            // If everything is OK - add argument to swcs and cut first letter (a '-' in this case)
            if a[index].contains('=') {
                let split: Vec<_> = (a[index][1..]).split('=').collect();
                s.push(String::from(split[0]));
                v.push(String::from(split[1]));
            }
            else {
                s.push(String::from(&a[index][1..]));
                v.push(String::from(""));
            };
        }
        index+=1;
    }
    (s, v)
}

pub fn opts() -> Vec<String> {
    let a = args();
    // Create mutable vector which will store switches and options later
    let mut o = vec![];
    // Go trough every argument
    let mut index = 1;
    while index < a.len() {
        // If it starts with '-', argument is a 'switch'
        let s = &a[index].chars().collect::<Vec<char>>();
        if s[0] != '-' {
            o.push(String::from(&a[index]));
        }
        index+=1;
    }
    o
}
