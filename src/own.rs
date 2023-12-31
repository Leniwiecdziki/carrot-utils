use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use std::fs;
use std::ffi::CString;
mod libargs;
mod lib2human;
mod lib2machine;
mod libdir;
use libc::lchown;

fn main() {
    let opts = libargs::opts();
    let (swcs, vals) = libargs::swcs();

    let mut rec = false;
    let mut link = false;
    let mut verbose = false;
    let mut index = 0;
    for v in vals {
        if !v.is_empty() {
            eprintln!("None of this program's switches accepts a value."); process::exit(1); 
        } 
    }
    while index < swcs.len() {
        let s = &swcs[index];

        if s != "r" && s != "rec"
        && s != "l" && s != "link"
        && s != "v" && s != "verbose" {
            eprintln!("Unknown switch: {s}");
            process::exit(1);
        }
        if s == "r" || s == "rec" {
            rec = true;
        }
        if s == "l" || s == "link" {
            link = true;
        }
        if s == "v" || s == "verbose" {
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
    let ids: Vec<_> = opts[0].split(',').collect();
    let uid = ids[0].trim();
    let gid = ids[1].trim();

    // Something is not quiet right if user requested more than two ID's
    if ids.len() > 2 {
        eprintln!("Maximally two ID's can be requested!");
        process::exit(1);
    }

    while index < opts.len() {
        let command_to_match = if link {
            fs::symlink_metadata(&opts[index])
        }
        else {
            fs::metadata(&opts[index])
        };
        // Convert ID's requested by user in arguments to a numbers of type u32
        let user =
            // If user set uid to "-", just use previous ownership ID
            if uid == "-" {
                match &command_to_match {
                    Err(e) => {
                        eprintln!("{}: Could not get current resource ownership ID: {:?}", &opts[index], e.kind());
                        index += 1;
                        continue;
                    },
                    Ok(file) => {
                        file.uid()
                    }
                }
            }
            else {
                match uid.parse::<u32>() {
                    Err(e) => {
                        eprintln!("Could not parse UID because of an error: {:?}", e.kind());
                        process::exit(1);
                    }
                    Ok(file) => {
                        file
                    }
                }
            };
        let group =
            if ids.len() == 1 || gid == "-" && ids.len() == 2  {
                match &command_to_match {
                    Err(e) => {
                        eprintln!("{}: Could not check current UID because of an error: {:?}", &opts[index], e.kind());
                        index += 1;
                        continue;
                    },
                    Ok(file) => {
                        file.gid()
                    }
                }
            }
            else {
                match gid.parse::<u32>() {
                    Err(e) => {
                        eprintln!("{}: Could not check current GID because of an error: {:?}", &opts[index], e.kind());
                        process::exit(1);
                    }
                    Ok(e) =>{
                        e
                    }
                }
            };
        if verbose {
            println!("Setting ownership mode: {user} {group}");
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
                println!("{}: Successfully changed ownership.", path);
            }
        }
        else {
            eprintln!("{}: Failed to set ownership!", path);
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