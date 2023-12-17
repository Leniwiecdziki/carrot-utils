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
    let mut dbg = false;
    let mut index = 0;
    let mut start = false;
    let mut end = false;
    let mut lines_to_show: usize = 0;
    while index < swcs.len() {
        let s = &swcs[index];
        let v = &vals[index];

        if s != "r" && s != "rev" && s != "c" && s != "count" && s != "d" && s != "debug" && 
        s != "s" && s != "start" && s != "e" && s != "end" {
            eprintln!("Unknown change: {s}");
            process::exit(1);
        }
        if s == "r" || s == "rev" {
            rev = true;
            if !v.is_empty() { eprintln!("Unsupported value for a change: {s}={v}"); process::exit(1); }
        }
        if s == "c" || s == "count" {
            showcount = true;
            if !v.is_empty() { eprintln!("Unsupported value for a change: {s}={v}"); process::exit(1); }
        }
        if s == "d" || s == "debug" {
            dbg = true;
            if !v.is_empty() { eprintln!("Unsupported value for a change: {s}={v}"); process::exit(1); }
        }
        if s == "s" || s == "start" {
            if v.is_empty() { eprintln!("This change requires a value: {s}={v}"); process::exit(1); }
            match v.parse::<usize>() {
                Err(e) =>
                {
                    eprintln!("Could not parse a value for a switch {}={} because of an error: {:?}", s, v, e.kind());
                    process::exit(1);
                },
                Ok(e) =>
                {
                    start=true;
                    lines_to_show=e;
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
                    lines_to_show=e;
                }
            }
        }
        if end & start {
            eprintln!("Switches 'start' and 'end' are colliding.");
            process::exit(1);
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
            Ok(e) => { 
                let mut output = Vec::new();
                let mut linecount = 1.try_into().unwrap();

                for line in e.lines() {
                    output.push(line)
                }

                // Show lines from a file in normal/reverse order
                // FIXME: Show Nth lines from end
                if rev {
                    for line in output.iter().rev() {
                        if 
                        start & (linecount <= lines_to_show) || 
                        end & (linecount >= lines_to_show) || 
                        !start & !end 
                        {
                            showit(showcount, dbg, linecount, line);
                        }
                        linecount += 1;
                    }
                }
                else {
                    for line in output {
                        if 
                        start & (linecount <= lines_to_show) || 
                        end & (linecount >= lines_to_show) || 
                        !start & !end 
                        {
                            showit(showcount, dbg, linecount, line);
                        }
                        linecount += 1;
                    }
                }

                index += 1; 
                if index < opts.len() { println!(); } 
                }
        }
    }
}

fn showit(showcount:bool, dbg:bool , linecount: usize, line:&str) {
    // This string contains "linecount: " if showcount is enabled
    let printline = if showcount {
        format!("{linecount}: ")
    }
    else {
        String::new()
    };
    if dbg {
        println!("{printline}{:#?}", line);
    }
    else {
        println!("{printline}{}", line);
    }

}
