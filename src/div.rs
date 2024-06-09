use std::fs;
use std::io;
use std::process;
use carrot_libs::args;

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
    let mut lines1 = Vec::new();
    let mut lines2 = Vec::new();
    // Read files and check for possible errors
    match fs::read_to_string(opts[0].clone()) {
        Err(e) => {
            eprintln!("{}: Failed to read a file: {:?}!", opts[0], e.kind());
            process::exit(1);
        },
        Ok(result) => {
            todo!("No clue");
            for l in result.lines() {
                lines1.push(l);
            }
        },
    };
    match fs::read_to_string(opts[1].clone()) {
        Err(e) => {
            eprintln!("{}: Failed to read a file: {:?}!", opts[0], e.kind());
            process::exit(1);
        },
        Ok(result) => {
            todo!("No clue");
            for l in result.lines() {
                lines2.push(l);
            }
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
    loop {
        match compare(lines1.clone(), lines2.clone(), left_idx, right_idx) {
            "both_end" => process::exit(0),
            "right_end" => {
                println!("{} !+: {}", left_idx, lines1[left_idx]);
                left_idx += 1;
            }
            "left_end" => {
                println!("{} +!: {}", right_idx, lines2[right_idx]);
                right_idx += 1;
            }
            "equal" => {
                println!("{} ==: {}", left_idx, lines1[left_idx]);
                left_idx += 1;
                right_idx += 1;
            }
            "differ" => {
                // If lines differ, check if this is because user appended/removed some line, or if this is
                // because lines changes completely
                match compare(lines1.clone(), lines2.clone(), left_idx, right_idx) {
                    "both_end" => process::exit(0),
                    "right_end" => {
                        println!("{} !+: {}", left_idx, lines1[left_idx]);
                        left_idx += 1;
                    }
                    "left_end" => {
                        println!("{} +!: {}", right_idx, lines2[right_idx]);
                        right_idx += 1;
                    }
                    "equal" => {
                        println!("{} ==: {}", left_idx, lines1[left_idx]);
                        left_idx += 1;
                        right_idx += 1;
                    }
                    _ => todo!(),
                }
            }
            _ => todo!(),
        }     
    }
}


fn compare(lines1:Vec<&str>, lines2:Vec<&str>, left_idx:usize, right_idx:usize) -> &'static str {
    // If BOTH files ended, quit.
    if lines1.get(left_idx).is_none() && lines2.get(left_idx).is_none() {
        "both_end"
    }
    // If there are no more lines on the right side
    else if lines1.get(left_idx).is_some() && lines2.get(left_idx).is_none() {
        "right_end"
    }
    // If there are no more lines on the left side
    else if lines1.get(left_idx).is_none() && lines2.get(left_idx).is_some() {
        "left_end"
    }
    // If lines are the same, just skip them and check other lines
    else if lines1.get(left_idx) == lines2.get(right_idx) {
        "equal"
    }
    else {
        "differ"
    }
}