use std::fs;
use std::os::unix::fs::{MetadataExt, FileTypeExt, PermissionsExt};
use std::process;
use carrot_libs::kinder;
use carrot_libs::args;

const FIELDS:[&str; 11] = ["name", "type", "size", "perms", "usrown", "grpown", "inode", "hlinks", "atime", "ctime", "mtime"];

fn main() {
    let opts: Vec<String> = args::opts();
    let (swcs, vals) = args::swcs();
    if opts.is_empty() {
        eprintln!("This program requires at least one resource name to present!");
        process::exit(1);
    }

    let mut human = false;
    let mut link = false;
    let mut show_all_fields = true;
    let mut fields_to_show = Vec::new();
    let mut index = 0;
    while index < swcs.len() {
        let s = &swcs[index];
        let v = &vals[index];

        if s != "h" && s != "human"
        && s != "l" && s != "link"
        && s != "f" && s != "field" {
            eprintln!("Unknown switch: {s}");
            process::exit(1);
        }
        if s == "h" || s == "human" {
            human = true;
            if !v.is_empty() { 
                eprintln!("Unsupported value for a switch: {s}={v}");
                process::exit(1);
            };
        }
        if s == "l" || s == "link" {
            link = true;
            if !v.is_empty() { 
                eprintln!("Unsupported value for a switch: {s}={v}");
                process::exit(1);
            };
        }
        if s == "f" || s == "field" {
            show_all_fields = false;
            if v.is_empty() {
                eprintln!("This switch expects a value!");
                process::exit(1);
            }
            // Split all field names requested as a value for switch "f"/"field"
            // and add them to "fields_to_show"
            for value in v.split(',') {
                if !FIELDS.contains(&value.to_lowercase().as_str()) {
                    eprintln!("Unsupported value for a switch: {s}={v}");
                    process::exit(1);
                }
                else {
                    fields_to_show.push(value.to_lowercase().to_string());
                };
            };
        }
        index += 1;
    }

    let mut index = 0;
    while index < opts.len() {
        let current_file= &opts[index];
        // Get metadata from symbolic link or not
        let command_to_match = if link {
            match fs::symlink_metadata(current_file) {
                Err(e) => {
                    eprintln!("{current_file}: Failed to get metadata: {:?}!", e.kind());
                    continue;
                },
                Ok(e) => e,
            }
        }
        else {
            match fs::metadata(current_file) {
                Err(e) => {
                    eprintln!("{current_file}: Failed to get metadata: {:?}!", e.kind());
                    continue;
                },
                Ok(e) => e,
            }
        };

        if fields_to_show.contains(&String::from("name")) || show_all_fields {
            let fieldname = if human { "Name" } else { "name" };
            let value = current_file;
            println!("{fieldname}: {value}");
        };

        if fields_to_show.contains(&String::from("type")) || show_all_fields {
            let fieldname = if human { "Type" } else { "type" };
            let e = &command_to_match;
            let value = if e.is_dir() { "Directory" }
            else if e.is_file() { "File" }
            else if e.is_symlink() { "Symlink" }
            else if e.file_type().is_block_device() { "Block" }
            else if e.file_type().is_char_device() { "Char" }
            else if e.file_type().is_fifo() { "Fifo" }
            else if e.file_type().is_socket() { "Socket" }
            else { "Unknown" };
            println!("{fieldname}: {value}");
        };

        if fields_to_show.contains(&String::from("size")) || show_all_fields {
            let fieldname = if human { "Size" } else { "size" };
            let value = if human {
                kinder::size(command_to_match.size())
            }
            else {
                command_to_match.size().to_string()
            };
            println!("{fieldname}: {value}");
        };

        if fields_to_show.contains(&String::from("perms")) || show_all_fields {
            let fieldname = if human { "Permissions" } else { "perms" };
            // Convert number format
            let converted_perms_mode = format!("{:o}", command_to_match.permissions().mode());
            // Permission mode numbers
            let user = converted_perms_mode.chars().nth_back(2).unwrap();
            let group = converted_perms_mode.chars().nth_back(1).unwrap();
            let others = converted_perms_mode.chars().nth_back(0).unwrap();
            let additional = converted_perms_mode.chars().nth_back(3).unwrap();
            
            let (u,g,o,a) = if human {
                (kinder::perms(user.to_digit(10).unwrap(), true).unwrap(),
                kinder::perms(group.to_digit(10).unwrap(), true).unwrap(),
                kinder::perms(others.to_digit(10).unwrap(), true).unwrap(),
                kinder::perms(additional.to_digit(10).unwrap(), false).unwrap())
            }
            else {
                (user.to_string(), group.to_string(), others.to_string(), additional.to_string())
            };
            println!("{fieldname}: {a}{u}{g}{o}");
        };

        if fields_to_show.contains(&String::from("usrown")) || show_all_fields {
            let fieldname = if human { "Owner (user)" } else { "usrown" };
            let value = command_to_match.uid();
            println!("{fieldname}: {value}");
        };

        if fields_to_show.contains(&String::from("grpown")) || show_all_fields {
            let fieldname = if human { "Owner (group)" } else { "grpown" };
            let value = command_to_match.gid();
            println!("{fieldname}: {value}");
        };

        if fields_to_show.contains(&String::from("inode")) || show_all_fields {
            let fieldname = if human { "Inode" } else { "inode" };
            let value = command_to_match.ino();
            println!("{fieldname}: {value}");
        };

        if fields_to_show.contains(&String::from("hlinks")) || show_all_fields {
            let fieldname = if human { "Hard links" } else { "hlinks" };
            let value = command_to_match.nlink();
            println!("{fieldname}: {value}");
        };

        /*if fields_to_show.contains(&String::from("atime")) || show_all_fields {
            let fieldname = if human { "Accessed" } else { "atime" };
            let time = command_to_match.atime();
            if !human {
                println!("{fieldname}: {time}");
            }
            else {
                let sec = kinder::sec(time);
                let min = kinder::min(time);
                let hour = kinder::hour(time);
                let day = kinder::day(time);
                let month = kinder::month(time);
                let year = kinder::year(time);
                println!("{fieldname}: {day}/{month}/{year} {hour}:{min}:{sec}");
            }
        };
        if fields_to_show.contains(&String::from("ctime")) || show_all_fields {
            let fieldname = if human { "Created" } else { "ctime" };
            let time = command_to_match.ctime();
            if !human {
                println!("{fieldname}: {time}");
            }
            else {
                let sec = kinder::sec(time);
                let min = kinder::min(time);
                let hour = kinder::hour(time);
                let day = kinder::day(time);
                let month = kinder::month(time);
                let year = kinder::year(time);
                println!("{fieldname}: {day}/{month}/{year} {hour}:{min}:{sec}");
            }
        };
        if fields_to_show.contains(&String::from("mtime")) || show_all_fields {
            let fieldname = if human { "Modified" } else { "mtime" };
            let time = command_to_match.mtime();
            if !human {
                println!("{fieldname}: {time}");
            }
            else {
                let sec = kinder::sec(time);
                let min = kinder::min(time);
                let hour = kinder::hour(time);
                let day = kinder::day(time);
                let month = kinder::month(time);
                let year = kinder::year(time);
                println!("{fieldname}: {day}/{month}/{year} {hour}:{min}:{sec}");
            }
        };
        */


        index += 1; 
    }
    if index < opts.len() {
        println!();
    }
}

