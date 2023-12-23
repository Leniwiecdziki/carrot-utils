use std::io;
use std::io::Write;
use std::process;
use std::env;
mod libargs;

fn main() {
    let opts = libargs::opts();
    let (swcs, vals) = libargs::swcs();

    // Refuse to run when switches were passed
    if ! swcs.is_empty() {
        eprintln!("This program does not support any switches and values!");
        process::exit(1);
    };
    if ! vals.is_empty() {
        eprintln!("This program does not support any switches and values!");
        process::exit(1);
    };

    /*
        This shell will work in two modes:
        File mode - read lines from a file provided by the user via arguments
        Input mode - read lines from stdin
     */
    let mode = if ! opts.is_empty() {
        "file"
    }
    else {
        "input"
    };

    if mode == "file" {
        todo!("File mode is not ready yet!");
    }
    else if mode == "input" {
        loop {
            // All commands
            let commands = getinput();
            // Commands separated by built-in keywords
            for (index, command) in commands.into_iter().enumerate() {
                // Check whether the first argument is a keyword or not
                match command[0].as_str() {
                    "help" | "?" => help(),
                    "exit" | "quit" | "bye" => exit(&command),
                    "gt" => gt(&command),
                    "then" => runcommand(&command[1..]),
                    _ => runcommand(&command),
                };
            }
        };
    }
}

fn getinput() -> Vec<Vec<String>> {
    // FOR ALL COMMENTS BELLOW: Assume, that user typed this command into a shell: say hello then say goodbye
    // This variable contains full line typed by the user (List 1.: 'say hello then say goodbye')
    let mut input = String::new();

    // This list contains all commands passed by the user 
    let mut commands: Vec<Vec<String>> = Vec::new();

    // This list contains arguments passed by the user (List 1.: 'say', 'hello') (List 2.: 'echo', 'goodbye')
    let mut words: Vec<String> = Vec::new();
    
    // Print a prompt
    print!("> ");
    // Flush stdout to print the prompt
    match io::stdout().flush() {
        Err(e) => {
            eprintln!("Shell crashed with the following error: {}", e.kind());
            process::exit(1);
        },
        // Read line into "input"
        Ok(_) => {
            match io::stdin().read_line(&mut input) {
                Err(e) => {
                    eprintln!("Shell crashed with the following error: {}", e.kind());
                    process::exit(1);
                },
                Ok(_) => {
                    /*
                        Character division helps to find individual arguments (words)
                        Expected output: ('say' 'hello' 'then' 'say' 'goodbye')
                     */
                    let mut word = String::new();
                    for c in input.chars() {
                        if c.is_whitespace() {
                            words.push(word.clone());
                            word.clear();
                        }
                        else {
                            word.push(c);
                        };
                    };
                    /*
                        Arguments division helps to find individual commands
                        Expected output: ('say' 'hello'), ('then' 'say' 'goodbye')
                     */
                    let mut command = Vec::new();
                    let words_lenght = words.len();
                    for (index, w) in words.into_iter().enumerate() {
                        // Append command to "commands" list if there are no more arguments available... 
                        if index == words_lenght-1 {
                            command.push(w);                      // Quickly append the last word to a command
                            if index != 0 {
                                // If there's not previous command, do not append anything!
                                commands.push(command.clone());   // Send a command to "commands"
                            }
                            command.clear();                      // Clear a list to start looking over for new, inline commands
                        }
                        // ...or when there is a built-in keyword
                        else if w == "then" {
                            if index != 0 {
                                // If there's not previous command, do not append anything!
                                commands.push(command.clone());   // Send previous command to "commands"
                            }
                            command.clear();                      // Clear list to start over
                            command.push(w);                      // Append a keyword to a fresh list
                        }
                        else {
                            command.push(w);
                        };
                    };
                    commands
                },
            }
        }
    }
}

fn help() {
    todo!("Help!");
}

fn exit(args:&Vec<String>) {
    if args.len() == 1 {
        process::exit(0)
    }
    else if args.len() > 2 {
        eprintln!("Cannot exit with multiple exit codes!");
    }
    else {
        match args[1].parse::<i32>() {
            Err(e) => eprintln!("Cannot exit with this code because of an error: {:?}", e.kind()),
            Ok(code) => process::exit(code),
        }
    }
}

fn gt(args:&Vec<String>) { 
    if args.len() == 1 {
        eprintln!("Give me a directory path to go!");
    }
    else if args.len() > 2 {
        eprintln!("Cannot go to multiple directories simultaneously!");
    }
    else if let Err(e) = env::set_current_dir(&args[1]) { 
            eprintln!("{}: Cannot go into this directory because of an error: {}", args[1], e.kind());
    };
}

fn runcommand(args:&[String]) {
    process::Command::new(&args[0]).args(&args[1..]).status().expect("Execution failed");
    io::stdout().flush().unwrap();
}