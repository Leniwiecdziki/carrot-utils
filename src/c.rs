mod libargs;
mod libdir;
use std::process;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::io::Read;
use std::fs::File;

fn main() {
    let opts = libargs::opts();
    let (swcs, vals) = libargs::swcs();
    match opts.len() {
        2 => false,
        3.. => true,
        _ => { eprintln!("This program requires at least two file names!");process::exit(1); },
    };

    let mut verbose = false;
    let mut ow = false;
    let mut index = 0;
    for v in vals {
        if !v.is_empty() {
            eprintln!("None of this program's switches accepts a value."); process::exit(1); 
        } 
    }
    while index < swcs.len() {
        let s = &swcs[index];

        if s != "o" && s != "overwrite" && s != "v" {
            eprintln!("Unknown switch: {s}");
            process::exit(1);
        }
        if s == "o" || s == "overwrite" {
            ow = true;
        }
        if s == "v" {
            verbose = true;
        }
        index += 1;
    }

    let dest = &opts[opts.len()-1];
    let mut destexists = fs::symlink_metadata(PathBuf::from(&dest)).is_ok();
    let mut destdir = if destexists { fs::symlink_metadata(PathBuf::from(&dest)).unwrap().is_dir() } else {false};

    let mut index = 0;
    while index < opts.len()-1 {
        let src = &opts[index];
        if verbose {
            println!("Copying to: {dest}");
        }
        /*
        There are multiple copying scenarios that we need to test:
        1. Both resources are the same? - Refuse to copy
        2. Is user copying directory to a file? - Prevent it from happening

        3. Is user copying file to a directory? - Copy under that directory, not as directory itself
        4. Is user copying directory to another directory (existing)? - Split them and overwrite/not overwrite
        5. Is user copying directory to another directory (nonexisting)? - Create it and copy recursively

        And also, there is 'overwrite' switch.
        By default, overwriting is blocked but if user permits it - just do it!
        */

        let exists = fs::symlink_metadata(PathBuf::from(&src)).is_ok();
        let isdir = if exists { fs::symlink_metadata(PathBuf::from(&src)).unwrap().is_dir() } else {false};

        if !exists {
            eprintln!("{}: Resource does not exist!", &src);
            index += 1;
            continue;
        }

        if let (Err(_), Err(_)) = (fs::canonicalize(src), fs::canonicalize(dest)) {
            eprintln!("Cannot get metadata from resources");
            index += 1;
            continue;
        }
        // Create a directory before copying to it
        (destexists, destdir) = 
        if exists && isdir && !destexists && !destdir {
            fs::create_dir_all(PathBuf::from(&dest)).unwrap();
            (true, true)
        }
        else if exists && !isdir && !destexists && !destdir {
            //fs::write(PathBuf::from(&dest), "").unwrap();
            (false, false)
        }
        else if exists && !isdir && destexists && !destdir {
            (true, false)
        }
        else {
            (true, true)
        };

        // Get fullpaths
        let s = fs::canonicalize(src).unwrap();
        let d = fs::canonicalize(dest).unwrap();

        // Test 1.
        // If src and dest exist, check if they are the same
        let mut buffer1 = Vec::new();
        let mut buffer2 = Vec::new();
        if exists && destexists && !isdir && !destdir {
            if let Err(e) = File::open(&s).unwrap().read_to_end(&mut buffer1) {
                eprintln!("{}: Cannot copy resource because of a problem with source: {:?}!", &src, e.kind());
                index += 1;
                continue;
            };
            if let Err(e) = File::open(&d).unwrap().read_to_end(&mut buffer2) {
                eprintln!("{}: Cannot copy resource because of a problem with destination: {:?}!", &src, e.kind());
                index += 1;
                continue;
            };
        }
        else {
            buffer1=[1].to_vec();
            buffer2=[2].to_vec();
        }
        if buffer1 == buffer2 {
            eprintln!("{}: Source and destination resources are equal!", &src);
            index += 1;
            continue;
        }

        // Test 2.
        // Check if we're copying directory to a file
        if isdir && exists && destexists && !destdir {
            eprintln!("{}: Copying directory to a file is not possible!", &src);
            index += 1;
            continue;
        }

        // Test 3.
        // File to file
        if !isdir && exists && destexists && !destdir {
            copy(&s, &d, &ow, &verbose);
            index += 1;
            continue;
        }

        // Test 4.
        // Directory to directory
        if exists && isdir && destexists && destdir {
            browsedir(&s, &d, &ow, &verbose);
            index += 1;
            continue;
        }

        // Test 5.
        // File to directory
        let mod_dest =  if exists && !isdir && destexists && destdir {
            PathBuf::from(format!("{}/{}", d.display(), s.file_name().unwrap().to_str().unwrap()))
        }
        else {
            d.clone()
        };
        copy(&s, &mod_dest, &ow, &verbose);

    index += 1;
    }
    
}

fn browsedir(src:&Path, dest:&Path, ow:&bool, verbose:&bool) {
    // List where all found files will be stored
    let mut srclist = libdir::browse(&PathBuf::from(src));
    let mut destlist = Vec::new();

    for r in srclist.clone() {
        // let justname = r.file_name().unwrap().to_os_string().into_string().unwrap();
        let cutpath = r.strip_prefix(dest).unwrap();
        let newdir = format!("{}/{}", dest.display(), cutpath.display());
        if r.is_dir() {
            fs::create_dir_all(PathBuf::from(newdir)).unwrap();
        }
        else {
            srclist.push(PathBuf::from(format!("{}/{}", src.display(), cutpath.display())));
            destlist.push(PathBuf::from(format!("{}/{}", dest.display(), cutpath.display())));
        }
    }
    // Remove folder names from srclist because destlist doesn't has them either
    // and they need to have the same contents
    srclist.retain(|i| !i.is_dir());

    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.zip
    for iterator in srclist.iter().zip(destlist) {
        let (s, d) = iterator;
        if !s.is_dir() {
            copy(s, &d, ow, verbose);
        }
    }

    // Append contents of a subdirectory if needed
    for r in libdir::browse(src) {
        if r.is_dir() {
            browsedir(r.as_path(), dest, ow, verbose);
        }
    }
}

fn copy(src:&Path, dest:&Path, ow:&bool, verbose:&bool) {
    if !ow && fs::symlink_metadata(PathBuf::from(&dest)).is_ok() {
        eprintln!("{}: Overwriting has been disabled! Refusing to copy as {}.", &src.display(), &dest.display());
    }
    else {
        match fs::copy(src, dest) {
            Err(e) => eprintln!("{}: Cannot copy element because of an error: {}", &dest.display(), e.kind()),
            Ok(_) => if *verbose {println!("{}: Copied successfully", &dest.display())},
        };
    };
}