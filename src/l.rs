use std::path::Path;
use std::process;
use std::fs;
mod libargs;
mod libdir;

fn main() {
    let mut opts = libargs::opts();
    let (swcs, vals) = libargs::swcs();
    // In this program, every option should be a dirname to show
    // If there are no options passed to the program, add a dot
    if opts.is_empty() {
        opts.push(String::from("./"));
    }
    let mut hidden = false;
    let mut rec = false;

    let mut index = 0;
    for v in vals {
        if !v.is_empty() {
            eprintln!("None of this program's switches accepts a value."); process::exit(1); 
        } 
    }
    while index < swcs.len() {
        let s = &swcs[index];

        if s != "h" && s != "hidden" && s != "r" && s != "rec" {
            eprintln!("Unknown switch: {s}");
            process::exit(1);
        }
        if s == "h" || s == "hidden" {
            hidden = true;
        }
        if s == "r" || s == "rec" {
            rec = true;
        }
        index += 1;
    }

    let mut index = 0;
    while index < opts.len() {
        println!("{}:", opts[index]);
        if ! Path::new(&opts[index]).exists() {
            eprintln!("Requested directory does not exist!");
            index += 1;
            if index < opts.len() {println!();}
            continue;
        }
        if ! Path::new(&opts[index]).is_dir() {
            eprintln!("Requested resource is not a directory!");
            index += 1;
            if index < opts.len() {println!();}
            continue;
        }
        let dir_to_list = fs::canonicalize(&opts[index]).unwrap();
        showdir(&dir_to_list, &hidden, &rec);
        index+=1;
        if index < opts.len() {
            println!();
        }
    }

}

fn showdir(dir:&Path, hidden: &bool, rec:&bool) {
    let result = libdir::browse(dir);
    let mut output = Vec::new();
    
    // Add new elements to 'output'
    for r in &result {
        /*
        Example command: "l /home/user/fruits -r" or "l fruits -r" from directory /home/user
        Example directory structure:
            /home/user/fruits/apple
            /home/user/fruits/banana
            /home/user/fruits/pear
            /home/user/fruits/sour/lemon
            /home/user/fruits/sour/orange
        original_request - /home/user/fruits
        fullpath - A full path to found element (/home/user/fruits/banana or /home/user/fruits/sour/lemon)
        justname - Just a name of found element (banana or lemon)
        cutpath - Full path without 'original_request' (banana or sour/lemon)
         */
        let justname = r.file_name().unwrap().to_os_string().into_string().unwrap();
        let cutpath = r.strip_prefix(dir).unwrap();
        if !justname.starts_with('.') || justname.starts_with('.') && *hidden {
            output.push(cutpath);
        }
    }
    output.sort();
    for s in output {
        println!("{}", s.display());
    }

    for r in libdir::browse(dir) {
        if *rec && r.is_dir() {
            showdir(r.as_path(), hidden, rec);
        }
    }
}