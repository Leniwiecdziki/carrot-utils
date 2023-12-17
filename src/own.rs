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
use libc::lchown;

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

        if s != "r" && s != "rec" && s != "v" {
            eprintln!("Unknown switch: {s}");
            process::exit(1);
        }
        if s == "r" || s == "rec" {
            rec = true;
        }
        if s == "v" {
            verbose = true;
        }
        index += 1;
    }
    // If there are no arguments passed to the program, print an error
    if opts.is_empty() {
        eprintln!("This program requires ownership IDs to set!");
        process::exit(1);
    }
    if opts.len() < 2 {
        eprintln!("This program requires at least one resource name!");
        process::exit(1);
    }

    let mut index = 1;
    let split: Vec<_> = opts[0].split(',').collect();

    while index < opts.len() {
        // Convert uid and gid to a number (u32)
        let user =
            if split[0] == "n" {
                match libfileinfo::uid(&PathBuf::from(&opts[index])) {
                    Err(e) => 
                    {
                        eprintln!("{}: Could not retrieve ownership of a resource because of an error: {:?}", &opts[index], e.kind());
                        index += 1;
                        continue;
                    },
                    Ok(e) =>
                    {
                        e
                    }
                }
            }
            else if split[0] != "n" {
                match split[0].parse::<u32>() {
                    Err(e) => 
                    {
                        eprintln!("Could not parse user ID because of an error: {:?}", e.kind());
                        process::exit(1);
                    }
                    Ok(e) =>
                    {
                        e
                    }
                }
            }
            else {
                eprintln!("Could not parse user ID because of an unknown error");
                process::exit(1);
            };
        let group =
            if split.len() == 1 || split[1] == "n" && split.len() == 2  {
                match libfileinfo::gid(&PathBuf::from(&opts[index])) {
                    Err(e) => 
                    {
                        eprintln!("{}: Could not retrieve ownership of a resource because of an error: {:?}", &opts[index], e.kind());
                        index += 1;
                        continue;
                    },
                    Ok(e) =>
                    {
                        e
                    }
                }
            }
            else if split[1] != "n" && split.len() == 2 {
                match split[1].parse::<u32>() {
                    Err(e) => 
                    {
                        eprintln!("Could not parse user ID because of an error: {:?}", e.kind());
                        process::exit(1);
                    }
                    Ok(e) =>
                    {
                        e
                    }
                }
            }
            else {
                eprintln!("Could not parse group ID because of an unknown error");
                process::exit(1);
            };
        if verbose {
            println!("Setting ownership mode: {user} {group}",);
        }
        if !PathBuf::from(&opts[index]).is_dir() || PathBuf::from(&opts[index]).is_dir() & !rec {
            changeown(&opts[index], &user, &group, &verbose);
        }
        else if PathBuf::from(&opts[index]).is_dir() & rec  {
            changeown(&opts[index], &user, &group, &verbose);
            browsedir(&PathBuf::from(&opts[index]), &user, &group, &rec, &verbose);
        }

        index += 1;
    }
}

fn changeown(path:&str, user:&u32, group:&u32, verbose:&bool) {
    // See manpage: chown (2)
    // Rust doesn't currently support changing file ownership
    unsafe {
        // Workaround for "Temporary CString as ptr"
        let a = CString::new(path).unwrap();
        let ret = lchown(
            a.as_ptr(), *user, *group
        );
        if ret == 0 {
            if *verbose {
                println!("Successfully changed ownership: {}", path);
            }
        }
        else {
            eprintln!("Failed to set ownership: {}", path);
        }
    }
}

fn browsedir(path:&Path, user:&u32, group:&u32, rec:&bool, verbose:&bool) {
    // List where all found files will be stored
    let result = libdir::browse(path);

    // Add new elements to 'result'
    for r in &result {
        changeown(r.to_str().unwrap(), user, group, verbose);
        if rec & r.is_dir() {
            changeown(r.to_str().unwrap(), user, group, verbose);
            browsedir(r, user, group, rec, verbose)
        }
    }
}