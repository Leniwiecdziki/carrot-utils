mod libargs;
use std::path::PathBuf;
use std::fs;
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
    for s in swcs {
        if s != "v" {
            eprintln!("Unknown switch: {s}");
            process::exit(1);
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

        // Check if destination exists and detect whether it is a directory or not
        let mut noexists = fs::metadata(&dest).is_err();

        // Create new dest-directory if it doesn't exist and dest must be a directory
        if mustdir && noexists {
            match fs::create_dir_all(&dest) {
                Err(e) => { eprintln!("{}: Was not added because of an error: {:?}!", dest.display(), e.kind()); noexists=true },
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
        let mod_dest = if isdir {
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
                eprintln!("{}: Cannot copy resource because of an error: {:?}!", mod_dest, e.kind());
            };
            if let Err(e) = File::open(&opts[index]).unwrap().read_to_end(&mut buffer2) {
                eprintln!("{}: Cannot copy resource because of an error: {:?}!", mod_dest, e.kind());
            };
        }
        else {
            buffer1=[1].to_vec();
            buffer2=[2].to_vec();
        }
    
            if buffer1 == buffer2 {
                eprintln!("{}: Those files are equal!", opts[index]);
                index += 1;
                continue;
            }
            match fs::rename(&opts[index], &mod_dest) {
                Err(e) => eprintln!("{}: Cannot move file because of an error: {:?}!", opts[index], e.kind()),
                _ => if verbose {println!("{}: Moved successfully.", opts[index])},
            }
        index += 1;
    }
}
