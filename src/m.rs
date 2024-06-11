use std::io::ErrorKind;
use std::os::unix::fs::MetadataExt;
use std::process;
use std::fs;
use std::path::PathBuf;
use std::io::Read;
use std::fs::File;
use carrot_libs::args;
use carrot_libs::dir;
use carrot_libs::input;

fn main() {
    let opts = args::opts();
    let (swcs, vals) = args::swcs();
    // Is user moving many files?
    let manysrcs = match opts.len() {
        2 => false,
        3.. => true,
        _ => { eprintln!("This program requires at least two file names!");process::exit(1); },
    };

    let mut verbose = false;
    let mut ask = false;
    let mut overwrite = false;
    let mut link = false;

    let mut index = 0;
    for v in vals {
        if !v.is_empty() {
            eprintln!("None of this program's switches accepts a value!"); process::exit(1); 
        } 
    };
    while index < swcs.len() {
        let s = &swcs[index];

        if s != "o" && s != "overwrite" && s != "v" && s != "verbose" && s != "l" && s != "link"
        && s != "a" && s != "ask" {
            eprintln!("Unknown switch: {s}");
            process::exit(1);
        }
        if s == "o" || s == "overwrite" {
            overwrite = true;
        }
        if s == "v" || s == "verbose" {
            verbose = true;
        }
        if s == "l" || s == "link" {
            link = true;
        }
        if s == "a" || s == "ask" {
            ask = true;
        }
        index += 1;
    };

    if ask && overwrite {
        eprintln!("Switch \"overwrite\" collides with \"ask\"!");
        process::exit(1);
    }

    // Check if destination element exists and it's type
    let command_to_match = if link {
        PathBuf::from(&opts.last().unwrap()).symlink_metadata()
    } else {
        PathBuf::from(&opts.last().unwrap()).metadata()
    };
    let mut dest_type = match &command_to_match {
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                "notfound".to_string()
            }
            else {
                eprintln!("{}: Failed to get destination metadata because of an error: {:?}", &opts[0], e.kind());
                process::exit(1);
            }
        },
        Ok(md) => {
            if md.is_dir() {
                "dir".to_string()
            }
            else {
                "file".to_string()
            }
        },
    };

    // Print error if there is an element that was referenced more than once
    let mut compare = Vec::new();
    for i in &opts {
        if compare.contains(&i) {
            eprintln!("{i}: File referenced more than once!");
            process::exit(1);
        }
        else {
            compare.push(i);
        };
    };

    // If destination is not found AND user requested only a directory OR there are many requests
    if dest_type == "notfound" && (PathBuf::from(&opts[0]).is_dir() || manysrcs) {
        // ... the destination has to be a directory
        dest_type = "dir".to_string();
    }
    // But when there is only a single file request
    else if !manysrcs && dest_type == "notfound" {
        // The destination has to be a file
        dest_type = "file".to_string();
    };

    // Many dirs/files to file
    if manysrcs && dest_type == "file" {
        eprintln!("Cannot move multiple elements to a file!");
        process::exit(1);
    }
    // Dir to file
    else if !manysrcs && PathBuf::from(&opts[0]).is_dir() && (dest_type != "dir" ) {
        eprintln!("Cannot move directory not to a directory!");
        process::exit(1);
    }
    else {
        // This loop will check for all possible errors before actually performing any changes to filesystem.
        for (index, source) in opts[..opts.len()-1].iter().enumerate() {
            // Save the name of destination file
            let mut dest = PathBuf::from(opts.last().unwrap());

            // Check if source file is a directory or not
            let command_to_match = if link {
                PathBuf::from(&opts[index]).symlink_metadata()
            } else {
                PathBuf::from(&opts[index]).metadata()
            };
            let source_is_dir = match &command_to_match {
                Err(e) => {
                    eprintln!("{source}: Failed to get source metadata because of an error: {:?}", e.kind());
                    process::exit(1);
                },
                Ok(md) => {
                    md.is_dir()
                },
            };

            // File to file
            if !source_is_dir && dest_type == "file" {
                // Check if both files are equal
                compare_files(source, &dest);
            }
            // File to dir
            if !source_is_dir && dest_type == "dir" {
                dest = dest.join(PathBuf::from(source).file_name().expect("Failed to get file name from source's path!"));
                compare_files(source, &dest);
            }
            // Dir to dir
            if source_is_dir && dest_type == "dir" {
                // dest = dest.join(source);
            }
        };

        // This loop will FINALLY make changes to filesystem without any checks.
        for (index, source) in opts[..opts.len()-1].iter().enumerate() {
            // Same as in previous loop
            let mut dest = PathBuf::from(opts.last().unwrap());
            // Same as in previous loop
            let command_to_match = if link {PathBuf::from(&opts[index]).symlink_metadata()} 
            else { PathBuf::from(&opts[index]).metadata()};
            let source_is_dir = match &command_to_match {
                Err(e) => {eprintln!("{source}: Failed to get source metadata because of an error: {:?}", e.kind());process::exit(1);},
                Ok(md) => {md.is_dir()},
            };

            if verbose {
                println!("Working on element: {}", &source)
            };

            // File to file
            if !source_is_dir && dest_type == "file" {
                rename(&PathBuf::from(source), &dest, &verbose, &overwrite, &ask)
            }
            // File to dir
            if !source_is_dir && dest_type == "dir" {
                dest = dest.join(PathBuf::from(source).file_name().unwrap());
                rename(&PathBuf::from(source), &dest, &verbose, &overwrite, &ask);
            }
            // Dir to dir
            if source_is_dir && dest_type == "dir" {
                /*
                Example directory A:
                - dogs/1.png
                - dogs/2.png
                - dogs/3.png
                - dogs/4.png

                Example directory B:
                - animals/cats/
                - animals/hamsters/
                - animals/spiders/

                If user tries to copy "dogs" with this command: c dogs/ animals
                Contents of "dogs" will be written directly to "animals", so the effect will look like this:
                - animals/cats/
                - animals/hamsters/
                - animals/spiders/
                - animals/1.png
                - animals/2.png
                - animals/3.png
                - animals/4.png

                But if user tries to copy "dogs" without trailing "/" in it's name with this command: c dogs animals
                It won't write contents of "dogs", but the directory itself, so the effect would look like this:
                - animals/cats/
                - animals/hamsters/
                - animals/spiders/
                - animals/dogs/
                With all doggie photos in it :D
                */ 
                if !source.ends_with('/') {
                    dest = dest.join(source);
                }
                browsedir(&PathBuf::from(source), &dest, &verbose, &overwrite, &ask);
            }
        }

    };
}

// Don't let to move identical files
fn compare_files(source:&String, dest:&PathBuf) {
    let mut buffer1 = Vec::new();
    let mut buffer2 = Vec::new();
    // Check if file is opened properly
    match File::open(source) {
        // Save it's contents to buffer1
        Ok(mut opened_file) => {
            opened_file.read_to_end(&mut buffer1).unwrap();
        },
        // Lack of source file is always a bad sign
        Err(e) => {
            eprintln!("{}: Failed to read a file because of an error: {:?}!", &dest.display(), e.kind());
            process::exit(1);
        },
    };
    // Check if file is not a directory
    if !dest.is_dir() {
        // Check if file is opened properly
        match File::open(dest) {
            // Save it's contents to buffer2
            Ok(mut opened_file) => {
                opened_file.read_to_end(&mut buffer2).unwrap();
            },
            // Lack of source file is okay. If file isn't found - just save "1" to the buffer and ingore the error
            Err(e) => {
                if e.kind() == ErrorKind::NotFound {
                    // Just quit from function
                    return;
                }
            // If there is different kind of error - terminate
                else {
                    eprintln!("{}: Failed to read a file because of an error: {:?}!", &dest.display(), e.kind());
                    process::exit(1);
                };
            },
        };
    }
    else {
        return;
    };

    if buffer1 == buffer2 {
        eprintln!("{source}: Source and destination elements are the same!");
        process::exit(1);
    }
}

fn browsedir(src:&PathBuf, dest:&PathBuf, verbose:&bool, overwrite:&bool, ask:&bool) {
    // Create directory on destination
    if let Err(e) = fs::create_dir_all(dest) {
        eprintln!("{:?}: Failed to create destination directory because of an error: {:?}", dest.display(), e.kind());
        process::exit(1);
    };
    // Contents of a directory that needs to be moved
    let srclist = match dir::browse(&PathBuf::from(src)) {
        Err(e) => {
            eprintln!("{}: Failed to browse a directory: {}", dest.to_string_lossy(), e);
            process::exit(1);
        }
        Ok(ret) => {
            ret
        }
    };

    for element in srclist {
        let stripped_src = element.strip_prefix(src).expect("Copying failed when program tried to strip destination prefix!");
        let copy_here = dest.join(stripped_src);
        if element.is_dir() {
            browsedir(&element, &copy_here, verbose, overwrite, ask);
        }
        else {
            rename(&element, &copy_here, verbose, overwrite, ask);
        };
    }
}

fn rename(source:&PathBuf, dest:&PathBuf, verbose:&bool, overwrite:&bool, ask:&bool) {
    let overwrite = if *ask {
        match input::ask(dest.to_string_lossy()) {
            Err(e) => {
                eprintln!("Can't get user input: {}!", e);
                process::exit(1);
            },
            Ok(e) => e
        }
    }
    else {
        *overwrite
    };

    if overwrite || !dest.exists() {
        // Do usual rename operation if both files are on the same disk
        if source.metadata().unwrap().dev() == dest.metadata().unwrap().dev() {
            match fs::rename(source, dest) {
                Err(e) => eprintln!("{}: Cannot move resource because of an error: {:?}", &source.display(), e.kind()),
                Ok(_) => if *verbose {println!("{}: Moved successfully", &source.display())},
            };
        }
        // If not, copy and remove it from source
        else {
            match fs::copy(source, dest) {
                Err(e) => eprintln!("{}: Cannot copy resource because of an error: {:?}", &source.display(), e.kind()),
                Ok(_) => if *verbose {println!("{}: Copied successfully", &source.display())},
            };
            match fs::remove_dir_all(source) {
                Err(e) => eprintln!("{}: Cannot remove source file(s) because of an error: {:?}", &source.display(), e.kind()),
                Ok(_) => if *verbose {println!("{}: Source file(s) removed successfully", &source.display())},
            };
        }
        
    }
    else {
        eprintln!("{}: Overwriting is disabled! Refusing to use this destination!", dest.display())
    };
}
