use std::fs;
use std::process;
mod libargs;

fn main() {
    let opts = libargs::opts();
    let (swcs, vals) = libargs::swcs();

    if opts.is_empty() {
        eprintln!("This program requires at least one file name!");
        process::exit(1);
    }

    let mut rev = false;
    let mut showcount = false;
    let mut index = 0;
    let mut start = false;
    let mut end = false;
    let mut start_from_line: usize = 0;
    while index < swcs.len() {
        let s = &swcs[index];
        let v = &vals[index];

        if s != "r" && s != "rev" && s != "c" && s != "count" && 
        s != "s" && s != "start" && s != "e" && s != "end" {
            eprintln!("Unknown switch: {s}");
            process::exit(1);
        }
        if s == "r" || s == "rev" {
            rev = true;
            if !v.is_empty() { eprintln!("Unsupported value for a switch: {s}={v}"); process::exit(1); }
        }
        if s == "c" || s == "count" {
            showcount = true;
            if !v.is_empty() { eprintln!("Unsupported value for a switch: {s}={v}"); process::exit(1); }
        }
        if s == "s" || s == "start" {
            if v.is_empty() { eprintln!("This switch requires a value: {s}={v}"); process::exit(1); }
            match v.parse::<usize>() {
                Err(e) =>
                {
                    eprintln!("Could not parse a value for a switch {}={} because of an error: {:?}", s, v, e.kind());
                    process::exit(1);
                },
                Ok(e) =>
                {
                    start=true;
                    start_from_line=e;
                }
            }
        }
        if s == "e" || s == "end" {
            if v.is_empty() { eprintln!("This change requires a value: {s}={v}"); process::exit(1); }
            match v.parse::<usize>() {
                Err(e) =>
                {
                    eprintln!("Could not parse a value for a switch {}={} because of an error: {:?}", s, v, e.kind());
                    process::exit(1);
                },
                Ok(e) =>
                {
                    end=true;
                    start_from_line=e;
                }
            }
        }
        index += 1; 
    }

    let mut index = 0;
    while index < opts.len() {
        match fs::read_to_string(&opts[index]) {
            Err(e) => { 
                eprintln!("{}: Cannot preview the file because of an error: {:?}!", opts[index], e.kind());
                index += 1;
                },
            Ok(f) => { 
                // Create a vector that stores entire file in it
                let mut contents = Vec::new();
                for line in f.lines() {
                    contents.push(line)
                }
                
                // Count lines
                let lines_in_file = contents.len();
                
                // Error handling for "start" and "end" switches
                if end & start {
                    eprintln!("Switches 'start' and 'end' are colliding.");
                    process::exit(1);
                };
                if (start_from_line > lines_in_file || start_from_line < 1) && (start || end) {
                    eprintln!("Line number out of range!");
                    process::exit(1);
                }

                // By default, this program will print out file contents from the first line
                // But when "rev" is enabled, it should be printed from the end.

                // FIXME: Mixing "start" or "end" with "rev" breaks the app
                let start_counting_from = if !rev {
                    1
                }
                else {
                    lines_in_file
                };
                // If user decides to not count lines from first - add "start" or "end" values
                let mut counter = if !start && !end {
                    start_counting_from
                }
                else if start && !end {
                    start_counting_from+start_from_line-1
                }
                else if !start && end {
                    let end_from_line = start_from_line-1;
                    lines_in_file-end_from_line
                }
                else {
                    1
                };
                
                // Print line by line
                while counter <= lines_in_file && counter != 0  {
                    // Create a counter object that will be displayed beside text when "count" switch is used
                    let counter_print = if showcount {
                        format!("{counter}: ")
                    }
                    else {
                        String::new()
                    };
                    println!("{}{}", counter_print, contents[counter-1]);
                    // Reverse line counting
                    if !rev {
                        counter += 1;
                    }
                    else {
                        counter -= 1;
                    };
                }

                index += 1; 
                if index < opts.len() { println!(); } 
                }
        }
    }
}
