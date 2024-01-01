use std::path::Path;
use std::path::PathBuf;
use std::process;
use std::ffi::CString;
mod libargs;
mod lib2human;
mod lib2machine;
mod libfileinfo;
mod libdir;
extern crate libc;
use libc::chmod;

//#[derive(Debug)]-
struct ModesTable {
    additional:u32, user: u32, group: u32, other: u32
}

fn main() {
    let opts = libargs::opts();
    let (swcs, vals) = libargs::swcs();

    let mut rec = false;
    let mut verbose = false;
    let mut index = 0;
    for v in vals {
        if !v.is_empty() {
            eprintln!("None of this program's switches accepts a value."); process::exit(1); 
        } 
    }
    while index < swcs.len() {
        let s = &swcs[index];

        if s != "r" && s != "rec" && s != "v" && s != "verbose" {
            eprintln!("Unknown switch: {s}");
            process::exit(1);
        }
        if s == "r" || s == "rec" {
            rec = true;
        }

        if s == "v" || s == "verbose" {
            verbose = true;
        }
        index += 1;
    }
    // If there are no arguments passed to the program, print an error
    if opts.is_empty() {
        eprintln!("This program requires permission's mode to set!");
        process::exit(1);
    }
    if opts.len() < 2 {
        eprintln!("This program requires at least one resource name!");
        process::exit(1);
    }

    let mut index = 1;
    let split: Vec<_> = opts[0].split(',').collect();

    while index < opts.len() {
        let m = checkmode(&split, PathBuf::from(&opts[index]));
        let m_all = format!("{}{}{}{}", m.additional, m.user, m.group, m.other);
        if m_all.contains('8') {
            eprintln!("Incorrect permission mode!");
            process::exit(1);
        }
        if verbose {
            println!("Setting permission mode: {m_all}");
        }
        
        if !PathBuf::from(&opts[index]).is_dir() || PathBuf::from(&opts[index]).is_dir() & !rec {
            changemode(&opts[index], &m_all, &verbose);
        }
        else if PathBuf::from(&opts[index]).is_dir() & rec  {
            changemode(&opts[index], &m_all, &verbose);
            browsedir(&PathBuf::from(&opts[index]), &String::from(&m_all), &rec, &verbose);
        }
        index += 1;
    }
}

fn checkmode(input:&Vec<&str>, file:PathBuf) -> ModesTable {
    let prev_perms = libfileinfo::perms(&file).unwrap();

    match input.len() {
        0 => { eprintln!("No modes!"); process::exit(1); },
        1 =>
            ModesTable {
                user:
                    if input[0] != "n" { lib2machine::perms(input[0], true) }
                    else { prev_perms.1 },
                group: prev_perms.2,
                other: prev_perms.3,
                additional: prev_perms.0,
            },
        2 => 
            ModesTable {
                user:
                    if input[0] != "n" { lib2machine::perms(input[0], true) }
                    else { prev_perms.1 },
                group:
                    if input[1] != "n" { lib2machine::perms(input[1], true) }
                    else { prev_perms.2 },
                other: prev_perms.3,
                additional: prev_perms.0,
            },

        3 => 
            ModesTable {
                user:
                    if input[0] != "n" { lib2machine::perms(input[0], true) }
                    else { prev_perms.1 },
                group:
                    if input[1] != "n" { lib2machine::perms(input[1], true) }
                    else { prev_perms.2 },
                other: 
                    if input[2] != "n" { lib2machine::perms(input[2], true) }
                    else { prev_perms.3 },
                additional: prev_perms.0,
            },

        4 => 
            ModesTable {
                user:
                    if input[0] != "n" { lib2machine::perms(input[0], true) }
                    else { prev_perms.1 },
                group:
                    if input[1] != "n" { lib2machine::perms(input[1], true) }
                    else { prev_perms.2 },
                other: 
                    if input[2] != "n" { lib2machine::perms(input[2], true) }
                    else { prev_perms.3 },
                additional: 
                    if input[3] != "n" { lib2machine::perms(input[3], false) }
                    else { prev_perms.0 },
            },

        _ => 
            { eprintln!("Too many modes!"); process::exit(1); },
    }
}

fn changemode(path:&str, mode: &str, verbose:&bool) {
    // See manpage: chmod (2)
    // For unknown reasons, PermissionsExt doesn't do anything, so I'm using libc
    unsafe {
        // Workaround for "Temporary CString as ptr"
        let a = CString::new(path).unwrap();
        let ret = chmod(
            a.as_ptr(), 
            u32::from_str_radix(mode, 8).unwrap()
        );
        if ret == 0 {
            if *verbose {
                println!("{path}: Succeeded");
            }
        }
        else {
            eprintln!("{path}: Failed");
        }
    }
}

fn browsedir(path:&Path, mode:&String, rec:&bool, verbose:&bool) {
    // List where all found files will be stored
    let result = libdir::browse(path);

    // Add new elements to 'result'
    for r in &result {
        changemode(r.to_str().unwrap(), mode.as_str(), verbose);
        if rec & r.is_dir() {
            changemode(r.to_str().unwrap(), mode.as_str(), verbose);
            browsedir(r, mode, rec, verbose)
        }
    }
}