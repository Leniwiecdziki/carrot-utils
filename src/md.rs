use std::fs;
use std::process;
use std::path::PathBuf;
mod lib2human;
mod lib2machine;
mod libargs;
mod libfileinfo;

fn main() {
    let opts: Vec<String> = libargs::opts();
    let (swcs, vals) = libargs::swcs();
    if opts.is_empty() {
        eprintln!("This program requires at least one resource name to present!");
        process::exit(1);
    }

    let mut hread = false;
    let mut index = 0;
    for v in vals {
        if !v.is_empty() {
            eprintln!("None of this program's switches accepts a value."); process::exit(1); 
        } 
    }
    while index < swcs.len() {
        let s = &swcs[index];

        if s != "h" && s != "human" {
            eprintln!("Unknown switch: {s}");
            process::exit(1);
        }
        if s == "h" || s == "human" {
            hread = true;
        }
        index += 1;
    }

    let mut index = 0;
    while index < opts.len() {
        if fs::symlink_metadata(&opts[index]).is_ok() {
            let current_file=&PathBuf::from(&opts[index]);
            println!("Name: {}", current_file.display());
            if libfileinfo::ftype(current_file).unwrap() == "Symlink" {
                println!("Path: {}", fs::read_link(current_file).unwrap().display());
            }
            else {
                println!("Path: {}", fs::canonicalize(current_file).unwrap().display());
            }
            ;
            if hread {
                println!("Size: {}", lib2human::size(libfileinfo::size(current_file).unwrap()));
            }
            else {
                println!("Size: {}", libfileinfo::size(current_file).unwrap());
            };
            if hread {
                let p = libfileinfo::perms(current_file).unwrap();
                let a = lib2human::perms(p.0, false);
                let u = lib2human::perms(p.1, true);
                let g = lib2human::perms(p.2, true);
                let o = lib2human::perms(p.3, true);
                println!("Permissions: {a}{u}{g}{o}");
            }
            else {
                let p = libfileinfo::perms(current_file).unwrap();
                println!("Permissions: {}{}{}{}", p.0, p.1, p.2, p.3);
            }
            println!("Owner (user): {}", libfileinfo::uid(current_file).unwrap());
            println!("Owner (group): {}", libfileinfo::gid(current_file).unwrap());
            println!("Inode: {}", libfileinfo::inode(current_file).unwrap());
            println!("Hard links: {}", libfileinfo::hlinks(current_file).unwrap());
            let atime = libfileinfo::atime(current_file).unwrap();
            let ctime = libfileinfo::ctime(current_file).unwrap();
            let mtime = libfileinfo::mtime(current_file).unwrap();
            if hread {
                println!("Accessed: {}/{}/{} {}:{}:{}", 
                lib2human::day(atime), lib2human::month(atime), lib2human::year(atime), lib2human::hour(atime), lib2human::min(atime), lib2human::sec(atime));
            }
            else {
                println!("Accessed: {atime}");
            };
            if hread {
                println!("Created: {}/{}/{} {}:{}:{}", 
                lib2human::day(ctime), lib2human::month(atime), lib2human::year(atime), lib2human::hour(atime), lib2human::min(atime), lib2human::sec(atime));
            }
            else {
                println!("Created: {ctime}");
            };
            if hread {
                println!("Modified: {}/{}/{} {}:{}:{}", 
                lib2human::day(mtime), lib2human::month(atime), lib2human::year(atime), lib2human::hour(atime), lib2human::min(atime), lib2human::sec(atime));
            }
            else {
                println!("Modified: {mtime}");
            };
        }
        else {
            eprintln!("{}: Requested resource cannot be found!", opts[index]);
        };
        index += 1; 
    }
        if index < opts.len() {
            println!();
        }
}

