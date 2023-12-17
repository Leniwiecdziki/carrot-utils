mod libargs;
mod libdir;
mod libfileinfo;
mod lib2human;
mod lib2machine;
use std::process;
use std::path::Path;
use std::path::PathBuf;
use std::collections::HashMap;

// TODO: atime, ctime, mtime
// TODO: Recursive search
// FIXME: NOT Logic

fn main() {
    let opts = libargs::opts();
    let (swcs, vals) = libargs::swcs();
    if opts.is_empty() {
        eprintln!("This program requires at least one directory name!");
        process::exit(1);
    }
    if swcs.is_empty() {
        eprintln!("This program requires at least one search filter!");
        process::exit(1);
    }

    let mut reqall = false;
    let mut not = false;
    let mut swcs_index = 0;
    while swcs_index < swcs.len() {
        let s = &swcs[swcs_index];
        let v = &vals[swcs_index];

        if s != "not" && s != "reqall"
        && s != "name" && s != "iname" && s != "ftype"
        && s != "links" && s != "size"
        && s != "read" && s != "noread" && s != "write" && s != "nowrite" && s != "exec" && s != "noexec" 
        && s != "uid" && s != "gid" && s != "uowner" && s != "gowner" && s != "nouowner" && s != "nogowner"
        && s != "atime" && s != "ctime" && s != "mtime" 
        && s != "lvl"
        {
            eprintln!("Unknown switch: {s}");
            process::exit(1);
        }

        if (s=="name"||s=="iname"||s=="type"||s=="links"||s=="size"||s=="uid"||s=="gid"||s=="uowner"||s=="gowner"||
        s=="atime"||s=="ctime"||s=="mtime"||s=="lvl"||s=="logic") && v.is_empty()
        {
            eprintln!("Switch requires a value: {s}");
            process::exit(1);
        }
        if (s=="or"||s=="s"||s=="nosort"||s=="read"||s=="noread"||s=="write"||s=="nowrite"||s=="exec"||s=="noexec"||
        s=="nouwoner"||s=="nogowner") && !v.is_empty() 
        {
            eprintln!("Unsupported value for a switch: {s}={v}");
            process::exit(1);
        }
        if s == "reqall" {
            reqall = true;
        }
        if s == "not" {
            not = true;
        }
        swcs_index += 1;
    }

    let mut swcs_index = 0;
    let mut opts_index = 0;

    while opts_index < opts.len() {
        // List of files in requested directory
        let mut entries = Vec::new();

        if !PathBuf::from(&opts[opts_index]).exists() {
            eprintln!("{}: Resource can't be found", &opts[opts_index]);
            process::exit(1);
        }
        if !PathBuf::from(&opts[opts_index]).is_dir() {
            eprintln!("{}: Resource is not a directory", &opts[opts_index]);
            process::exit(1);
        }

        for file in libdir::browse(&PathBuf::from(&opts[opts_index])) {
            entries.push(file);
        }

        /* 
        Send a list of files in currently requested directory, switch and it's value to "showdir".
        This should return a number of switch, that is being used now and a list of files, that are matching a condition
        */
        let mut table: HashMap<usize, Vec<PathBuf>> = HashMap::new();
        while swcs_index < swcs.len() {
            table.insert( 
                swcs_index, showdir(entries.to_owned(), &swcs[swcs_index], &vals[swcs_index], 0, &not) 
            );
            swcs_index += 1;
        };
        opts_index += 1;

        // OR LOGIC: Join all tables and deduplicate
        if !reqall {
            let table = table.into_values();
            let mut output = Vec::new();
            for list in table {
                for i in list {
                    output.push(i);
                }
            }
            output.sort();
            output.dedup();
            for i in output {
                println!("{}", i.into_os_string().into_string().unwrap());
            }
        }
        // AND LOGIC: Get list of all available files and slowly remove those, that are missing in at least one condition
        else {
            let table = table.into_values();
            for list in table {
                let mut file_index = 0;
                while file_index < entries.len() {
                    if list.contains(&entries[file_index]) {
                        file_index += 1;
                    }
                    else {
                        entries.remove(file_index);
                    }
                }
            }
            for i in entries {
                println!("{}", i.into_os_string().into_string().unwrap())
            }
        }
        
    };

}

fn showdir(mut entries:Vec<PathBuf>, s:&str, v:&str, mut lvl:i32, not:&bool) -> Vec<PathBuf> {
    lvl += 1;

    let mut index = 0;
    while index < entries.len() {
    let r = &entries[index];
        if (!correct(s, v, r, lvl) && !*not) || (correct(s, v, r, lvl) && *not) {
            entries.remove(index);
        }
        else {
            index += 1;
        }
    };

    /* Append contents of a subdirectory if needed
    for r in &entries {
        if r.is_dir() {
            showdir(entries.to_owned(), s, v, lvl, not);
        }
    }
    */
    entries
}

// Function that checks if switch matches value

fn correct(s:&str, v:&str, r:&Path, lvl:i32) -> bool {
    let justname = r.file_name().unwrap().to_str().unwrap();
    s=="not" || s=="reqall" ||
    s=="name" && v == justname ||
    s=="iname" && v.to_lowercase() == justname.to_lowercase() ||
    s=="ftype" && v.to_lowercase() == libfileinfo::ftype(r).unwrap().to_lowercase() ||
    s=="size" && v.to_lowercase().ends_with('+') && lib2machine::size(v) < libfileinfo::size(r).unwrap() ||
    s=="size" && v.to_lowercase().ends_with('-') && lib2machine::size(v) > libfileinfo::size(r).unwrap() ||
    s=="size" && v.to_lowercase() == lib2human::size(libfileinfo::size(r).unwrap()).to_lowercase() ||
    s=="read" && lib2human::perms(libfileinfo::perms(r).unwrap().1, true).to_lowercase().contains('r') ||
    s=="read" && lib2human::perms(libfileinfo::perms(r).unwrap().2, true).to_lowercase().contains('r') ||
    s=="read" && lib2human::perms(libfileinfo::perms(r).unwrap().3, true).to_lowercase().contains('r') ||
    s=="write" && lib2human::perms(libfileinfo::perms(r).unwrap().1, true).to_lowercase().contains('w') ||
    s=="write" && lib2human::perms(libfileinfo::perms(r).unwrap().2, true).to_lowercase().contains('w') ||
    s=="write" && lib2human::perms(libfileinfo::perms(r).unwrap().3, true).to_lowercase().contains('w') ||
    s=="exec" && lib2human::perms(libfileinfo::perms(r).unwrap().1, true).to_lowercase().contains('x') ||
    s=="exec" && lib2human::perms(libfileinfo::perms(r).unwrap().2, true).to_lowercase().contains('x') ||
    s=="exec" && lib2human::perms(libfileinfo::perms(r).unwrap().3, true).to_lowercase().contains('x') ||
    s=="noread" && !lib2human::perms(libfileinfo::perms(r).unwrap().1, true).to_lowercase().contains('r') ||
    s=="noread" && !lib2human::perms(libfileinfo::perms(r).unwrap().2, true).to_lowercase().contains('r') ||
    s=="noread" && !lib2human::perms(libfileinfo::perms(r).unwrap().3, true).to_lowercase().contains('r') ||
    s=="nowrite" && !lib2human::perms(libfileinfo::perms(r).unwrap().1, true).to_lowercase().contains('w') ||
    s=="nowrite" && !lib2human::perms(libfileinfo::perms(r).unwrap().2, true).to_lowercase().contains('w') ||
    s=="nowrite" && !lib2human::perms(libfileinfo::perms(r).unwrap().3, true).to_lowercase().contains('w') ||
    s=="noexec" && !lib2human::perms(libfileinfo::perms(r).unwrap().1, true).to_lowercase().contains('x') ||
    s=="noexec" && !lib2human::perms(libfileinfo::perms(r).unwrap().2, true).to_lowercase().contains('x') ||
    s=="noexec" && !lib2human::perms(libfileinfo::perms(r).unwrap().3, true).to_lowercase().contains('x') ||
    s=="uid" && *v == libfileinfo::uid(r).unwrap().to_string() ||
    s=="gid" && *v == libfileinfo::gid(r).unwrap().to_string() ||
    s=="links" && v.ends_with('+') && libfileinfo::hlinks(r).unwrap() > v.split('+').collect::<Vec<&str>>()[0].parse::<u64>().unwrap() ||
    s=="links" && v.ends_with('-') && libfileinfo::hlinks(r).unwrap() < v.split('-').collect::<Vec<&str>>()[0].parse::<u64>().unwrap() ||
    s=="links" && *v == libfileinfo::hlinks(r).unwrap().to_string() ||
    s=="lvl" && v.ends_with('+') && lvl > v.split('+').collect::<Vec<&str>>()[0].parse::<i32>().unwrap() ||
    s=="lvl" && v.ends_with('-') && lvl < v.split('-').collect::<Vec<&str>>()[0].parse::<i32>().unwrap() ||
    s=="lvl" && lvl == v.parse::<i32>().unwrap()
}