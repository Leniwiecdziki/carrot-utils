use std::fs;
use std::io::Read;
use std::process;
use std::io::{self, IsTerminal};
use std::time;
use std::thread;
use carrot_libs::args;

fn main() {
    let opts = args::opts();
    let (swcs, vals) = args::swcs();

    let mut index = 0;
    let mut show_all_contents_on_start = true;
    let mut sleep_time = 0.0;
    while index < swcs.len() {
        let s = &swcs[index];
        let v = &vals[index];

        if s != "f" && s != "full" && s != "l" && s != "last" && s != "s" && s != "sleep" {
            if !v.is_empty() { eprintln!("{s}: This switch doesn't accept a value!"); process::exit(1); };
            eprintln!("Unknown switch: {s}!"); process::exit(1);
        }

        if s == "f" || s == "full" {
            if !v.is_empty() { eprintln!("{s}: This switch doesn't accept a value!"); process::exit(1); };
            show_all_contents_on_start = true;
        }
        if s == "l" || s == "last" {
            show_all_contents_on_start = false;
        }
        if s == "s" || s == "sleep" {
            sleep_time = match v.parse::<f32>() {
                Ok(e) => e,
                Err(e) => {eprintln!("Failed to convert switch value to a floating point number: {s}={v}: {:?}", e); process::exit(1);},
            }
        }
        index+=1;
    }

    // This program is able to seek for updates only in one source at a time
    // so accept only one file in options OR something from pipe
    
    // If no options were passed and nothing is piped
    if opts.is_empty() && io::stdin().is_terminal() {
        eprintln!("This program requires file or pipe source to read from!");
        process::exit(1);
    }
    // If some file is requested but also something is piped
    if (!opts.is_empty() && !io::stdin().is_terminal()) || opts.len() > 1 {
        eprintln!("This program does not support reading from more than one source!");
        process::exit(1);
    }

    let mut text_len_outside_loop = 0;
    
    // Something is being piped?
    if !io::stdin().is_terminal() {
        // This program is useless when working with pipes...
        loop {
            let mut buffer = [0,1,2,3];
            while let Ok(n_bytes) = io::stdin().read(&mut buffer) {
                if n_bytes == 0 {break};

                // Convert UTF-8 codes to readable string
                let text = core::str::from_utf8(&buffer).unwrap();
                // Print pipe contents
                print!("{}", text);
                buffer.fill(0);

                // Wait for some time specified in -s/-sleep
                let sleep_time = time::Duration::from_secs_f32(sleep_time);
                thread::sleep(sleep_time);
            }
        }
    }
    else {
        let source = &opts[0];
        if show_all_contents_on_start {
            match fs::read_to_string(source) {
                Err(e) => {
                    eprintln!("{}: Failed to read from source: {:?}!", source, e.kind());
                    process::exit(1);
                },
                Ok(e) => {
                    print!("{e}");
                }
            }
        }
        loop {
            match fs::read(source) {
                Ok(read) => {
                    // Convert UTF-8 codes to readable string
                    let text = core::str::from_utf8(&read).unwrap();
                    // Check number of lines in source
                    let text_len_inside_loop = text.lines().count();
                    // If the file is longer, print the last line and update text_len
                    // Note: When text_len_outside_loop is equal to zero, this typically means that our
                    // program just started or the file that we're reading is empty.
                    // In this case - don't print anything.
                    if text_len_outside_loop < text_len_inside_loop {
                        if text_len_outside_loop != 0 {
                            println!("{}", text.lines().last().unwrap());
                        }
                        text_len_outside_loop = text_len_inside_loop;
                    }
    
                    // Wait for some time specified in -s/-sleep
                    let sleep_time = time::Duration::from_secs_f32(sleep_time);
                    thread::sleep(sleep_time);
                },
                Err(e) => {
                    eprintln!("{}: Failed to read from source: {:?}!", source, e.kind());
                    process::exit(1);
                }
            }
        }
    }

    
}
