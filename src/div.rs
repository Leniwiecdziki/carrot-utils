use std::fs;
use std::io;
use std::process;
use carrot_libs::args;
use std::collections::BTreeMap;

fn main() {
    let opts = args::opts();
    let (swcs, vals) = args::swcs();
    for v in vals {
        if !v.is_empty() {
            eprintln!("None of this program's switches accepts a value."); process::exit(1); 
        } 
    }
    let mut status = false;
    let mut show_exact_lines = false;
    for s in swcs {
        if s != "s" && s != "status"
        && s != "i" && s != "ignore"
        && s != "e" && s != "exact" {
            eprintln!("Unknown switch: {s}");
            process::exit(1);
        }
        if s == "s" || s == "status" {
            status = true;
        }
        if s == "i" || s == "ignore" {
            show_exact_lines = false;
        }
        if s == "e" || s == "exact" {
            show_exact_lines = true;
        }
    }

    if opts.len() != 2 {
        eprintln!("Type the name of two files to compare!");
        process::exit(1);
    }

    // Save file lines to vectors
    let mut lines = BTreeMap::new();

    // Read files and check for possible errors
    let file1 = fs::read_to_string(opts[0].clone());
    let file2 = fs::read_to_string(opts[1].clone());

    match (file1, file2) {
        (Ok(ref res1), Ok(ref res2)) => {
            for l in res1.lines() {
                lines.insert(l, String::from(""));
            }
        },
        _ => {
            eprintln!("{}: Failed to read a file: {:?}!", opts[0], e.kind());
            process::exit(1);
        },
    };

    // Compare lines
    let mut left_idx = 0;
    let mut right_idx = 0;

    // This is a table of all symbols:
    // =    Lines match
    // ~    Line is updated
    // +    Line is appended
    // -    Line is removed
    // !    Line does not exist

    // Compare lines
    for l in lines1 {

    }
}


fn is_on_other_side(lines:Vec<&str>, idx:usize) -> bool {
    for (i,l) in lines[idx..].iter().enumerate() {
        let haha = idx+i;
        if *l == lines[haha] && haha != lines.len() {
            return true;
        } else {
            return false;
        }
    }
    // Bro, return something, just in case (skull emoji)
    false
}
