mod libargs;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::fs;
use std::env;
use std::process;
use std::fs::File;
use std::io::Read;

fn main() {
    // Import all arguments from command line
    let opts = libargs::opts();
    let (swcs, vals) = libargs::swcs();
    for v in vals {
        if !v.is_empty() {
            eprintln!("None of this program's switches accepts a value."); process::exit(1); 
        } 
    }
    let mut verbose = false;
    let mut ow = false;
    for s in swcs {
        if s != "v" && s != "o" && s != "overwrite" {
            eprintln!("Unknown switch: {s}");
            process::exit(1);
        }
        if s == "o" || s == "overwrite" {
            ow = true;
        }
        else {
            verbose = true;
        }
    }
    // Check how many arguments are available
    // If only two file names are given - the destination can be file or directory
    // If more than two file names are given - the destination must be a directory
    let mustdir = match opts.len() {
        2 => false,
        3.. => true,
        _ => { eprintln!("This program requires at least two file names!");process::exit(1); },
    };

    // Go through every passed option except from the first and last.
    let mut index = 0;
    if verbose {
        println!("Moving to: {}", &opts[opts.len()-1]);
    }

    while index < opts.len() && index != opts.len()-1 {
        // Get destination (the last option)
        let dest:PathBuf = PathBuf::from(&opts[opts.len()-1]);

        // Check if destination exists
        let mut noexists = fs::metadata(&dest).is_err();

        // If overwriting is disabled, refuse moving to a new location if destination exists
        if ow && ! noexists {
            eprintln!("{}: Overwriting is disabled! Refusing to move resource to existing destination!", dest.display());
            process::exit(1);
        }

        // Create new dest-directory if it doesn't exist and dest must be a directory
        if mustdir && noexists {
            match fs::create_dir_all(&dest) {
                Err(e) => { eprintln!("{}: Dicrectory wasn't added because of an error: {:?}!", dest.display(), e.kind()); noexists=true },
                _ => noexists=false,
            }
        }
        let isdir = !noexists && fs::metadata(&dest).unwrap().is_dir();
        // If existing file is not a directory and if we cannot move to a file - print an error
        if ! isdir && mustdir {
            eprintln!("{}: Multiple resources can be moved only to a directory!", dest.display());
            process::exit(1);
        }

        // Moving a file to dir is impossible - move to dir/file instead.
        let mod_dest = if dest == PathBuf::from(".") {
            noexists = true;
            format!("{}/{}", env::current_dir().unwrap().display(), &opts[index])
        }
        else if isdir {
            noexists = true;
            format!("{}/{}", &dest.display(), &opts[index])
        }
        else {
            format!("{}", &dest.display())
        };
        // Test if both files are the same (if destination exists)
        let mut buffer1 = Vec::new();
        let mut buffer2 = Vec::new();
        if ! noexists {
            if let Err(e) = File::open(&opts[index]).unwrap().read_to_end(&mut buffer1) {
                eprintln!("{}: Cannot copy resource because of a problem with source: {:?}!", mod_dest, e.kind());
            };
            if let Err(e) = File::open(&opts[index]).unwrap().read_to_end(&mut buffer2) {
                eprintln!("{}: Cannot copy resource because of a problem with destination: {:?}!", mod_dest, e.kind());
            };
        }
        else {
            buffer1=[1].to_vec();
            buffer2=[2].to_vec();
        }
            if buffer1 == buffer2 {
                eprintln!("{}: Source and destination resources are equal!", opts[index]);
                index += 1;
                continue;
            }
            match fs::rename(&opts[index], &mod_dest) {
                Err(e) => {
                    /*
                    Move operation is nothing else than just changing a link to some inode on disk. This is a nice feature, 
                    because nothing is really moving on disk so the process is quicker.
                    But when can't do this when moving data across different disks!

                    Copy the file to a new path and remove it's original when "CrossedDevices" error appears
                     */

                    // Detect if CrossedDevices appeared (it is a nightly feature of Rust language for now!!!)
                    match e.kind() {
                        /*
                            Method to detect CrossesDevices error is available only in nightly releases of Rust!!!
                            io::ErrorKind::CrossesDevices => {
                         */ 
                        ErrorKind::Other => {
                            // Copy instead of moving
                            match fs::copy(&opts[index], &mod_dest) {
                                Err(e) => eprintln!("{}: Moving failed while copying to another disk because of an error: {}", opts[index], e.kind()),
                                Ok(_) => {
                                    // Remove original file or a directory that we copied
                                    if isdir {
                                        match fs::remove_dir_all(&opts[index]) {
                                            Err(e) => eprintln!("{}: Moving failed while removing older resource because of an error: {}", opts[index], e.kind()),
                                            Ok(_) => { if verbose {println!("{}: Moved successfully.", opts[index])} }
                                        };
                                    }
                                    else {
                                        match fs::remove_file(&opts[index]) {
                                            Err(e) => eprintln!("{}: Moving failed while removing older resource because of an error: {}", opts[index], e.kind()),
                                            Ok(_) => { if verbose {println!("{}: Moved successfully.", opts[index])} }
                                        };
                                    }
                                }
                                
                            }
                        },
                    _ => eprintln!("{}: Cannot move resource because of an error: {:?}!", opts[index], e.kind()),
                    };
                },
                _ => { if verbose {println!("{}: Moved successfully.", opts[index])} },
            };
        index += 1;
    }
}
