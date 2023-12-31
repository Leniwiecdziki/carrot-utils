use std::path::Path;
use std::path::PathBuf;
use std::process;
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
    let mut sort = true;
    let mut rec = false;

    let mut index = 0;
    for v in vals {
        if !v.is_empty() {
            eprintln!("None of this program's switches accepts a value."); process::exit(1); 
        } 
    }
    while index < swcs.len() {
        let s = &swcs[index];

        if s != "h" && s != "hidden" 
        && s != "r" && s != "rec"
        && s != "n" && s != "nosort" {
            eprintln!("Unknown switch: {s}");
            process::exit(1);
        }
        if s == "h" || s == "hidden" {
            hidden = true;
        }
        if s == "r" || s == "rec" {
            rec = true;
        }
        if s == "n" || s == "nosort" {
            sort = false;
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
        // This is a name of directory that was requested
        let original_request = &opts[index];
        let dir_to_list = &opts[index];

        showdir(&PathBuf::from(original_request), &PathBuf::from(dir_to_list), &hidden, &rec, &sort);
        index+=1;
        if index < opts.len() {
            println!();
        }
    }

}

fn showdir(original_request:&Path, dir:&Path, hidden: &bool, rec:&bool, sort:&bool) {
    let result = libdir::browse(dir);
    let mut sorted_result = Vec::new();
    
    // For every element found in directory
    let mut index = 0;
    while index < result.len() {
        let element = &result[index];
        // Get it's name without any trailing directory
        let filename_without_og = element.strip_prefix(original_request).unwrap();
        // Get it's name without a directory name specified as a part of argument
        let filename_without_any_dir = element.file_name().unwrap().to_os_string().into_string().unwrap();

        // Do not show hidden files
        let is_hidden = filename_without_any_dir.starts_with('.');
        if !is_hidden || *hidden {
            if *sort {
                sorted_result.push(filename_without_og);
            }
            else {
                println!("{}", filename_without_og.display());
            };
            // Descent into another directory if "rec" is enabled
            if element.is_dir() && *rec {
                showdir(original_request, &original_request.join(filename_without_og), hidden, rec, sort);
            };
        };
        index += 1;
    };

    if *sort {
        sorted_result.sort();
        for i in sorted_result  {
            println!("{}", i.display());
        };
    };
}