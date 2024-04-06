use std::io::{self, Read};
use std::io::IsTerminal;
use std::fs;
use std::process;
use carrot_libs::args;

fn main() {
    let opts = args::opts();
    let (swcs, vals) = args::swcs();

    // This sets the number of columns to create.
    // If the value is None - the terminal width will be used instead
    let mut width = crossterm::terminal::size().unwrap().0;
    let mut index = 0;
    while index < swcs.len() {
        let s = &swcs[index];
        let v = &vals[index];

        if s != "w" && s != "width"
        {
            eprintln!("Unknown switch: {s}");
            process::exit(1);
        }
        // Make sure, that value is set!
        if v.is_empty() { 
            eprintln!("This switch requires a value: {s}!"); process::exit(1); 
        }
        
        if s=="w"||s=="width" {
            // Parse value as a number
            let v = match v.parse::<u16>() {
                Err(e) => {eprintln!("Failed to parse a numeric value because of an error: {:?}!", e.kind());process::exit(1);},
                Ok(res) => res,
            };
            width = v;
        }
        index += 1;
    }

    // Show error when there are no files requested as options by user and nothing is piped to the program
    if opts.is_empty() && io::stdin().is_terminal() {
        eprintln!("Type the name of elements to use!");
        process::exit(1);
    }

    // If something is piped to our program, show it.
    if !io::stdin().is_terminal() {
        let mut contents_of_stdin = String::new();
        io::stdin().lock().read_to_string(&mut contents_of_stdin).expect("Failed to retrieve contents of stdin!");
        columnize(contents_of_stdin, width);
    }

    if opts.is_empty() {
        process::exit(0);
    }
    // Show stuff requested as options
    let mut index = 0;
    while index < opts.len() {
        match fs::read_to_string(&opts[index]) {
            Err(e) => { 
                eprintln!("{}: Cannot preview the file: {:?}!", opts[index], e.kind());
                index += 1;
            },
            Ok(f) => {
                columnize(f, width);
            },
        };
        index += 1;
    };
}

fn columnize(text:String, width:u16) {
    // Find out which word is the longest in our input
    let mut lenght_of_the_longest_word = 0;
    for word in text.split_whitespace() {
        if word.len() > lenght_of_the_longest_word {
            // Adding one to word.len() adds super space between columns
            lenght_of_the_longest_word = word.len();
        };
    }

    // This defines how many times the longest (and any other) word 
    // would fit in the defined width
    let max_words_per_row = width/lenght_of_the_longest_word as u16;

    if max_words_per_row == 0 {
        eprintln!("The text in the column is too long to fit in the desired width!");
        process::exit(1);
    }

    let mut idx = 1;
    for word in text.split_whitespace() {
        print!("{word}");
        // End row by printing the new line when maximum allowed number of words has been shown
        if idx%max_words_per_row==0 {
            idx=0;
            println!();
        }
        // Filling gap is a "space" character repeated as many times
        // as it it needed to make an elegant column

        // TIP: We don't need that filling when the number of max words per row is reached.
        // We don't have to make empty space before another word - it simply does not
        // exist on the current row.
        else {
            let filling_gap = " ".repeat(lenght_of_the_longest_word-word.len());
            print!("{filling_gap} ");
        }
        idx += 1;
    }
    // Print the final newline
    println!();
}