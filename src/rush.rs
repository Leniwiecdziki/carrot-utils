use std::io;
use std::io::Write;
use std::process;
use std::env;
mod libargs;

/*
    TODO
    There are three types of commands in RUSH
    - Classic commands: When you try to run something like 'git' or 'htop', it will be executed by system with all it's arguments.
    - Built-in commands: They also get their arguments as usual BUT they will be executed by shell
    - SUPER COMMANDS: They are used to operate on output, exit code, or anything else from previous or next commands

    Let's assume that we have a following script:
    if my_app do echo it works! end
    In this example, super command "if" has to find out if "my_app" succeeded. 
    If so, every command from "do" to "end" will be executed
 */ 
const SUPER_COMMANDS:[&str; 2] = ["then", "if"];

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
        // Always set $? (return code of previous command) to zero on start-up
        env::set_var("?", "0");
        loop {
            // All commands
            let commands = getinput();
            // Commands separated by built-in keywords
            for (index, command) in commands.into_iter().enumerate() {
                // Check whether the first argument is a keyword or not
                match command[0].as_str() {
                    "gt" => gt(&command),
                    "help" | "?" => help(),
                    "exit" | "quit" | "bye" => exit(&command),
                    "then" | "then2" | "then3" => runcommand(&command[1..]),
                    "if" => todo!("IF"),
                    _ => runcommand(&command),
                };
            }
        };
    }
}

fn getinput() -> Vec<Vec<String>> {
    // FOR ALL COMMENTS BELLOW: Assume, that user typed this command into a shell: af file then ad dir
    // This variable contains full line typed by the user (List 1.: 'af file then ad dir')
    let mut input = String::new();

    // This list contains all commands passed by the user 
    let mut commands: Vec<Vec<String>> = Vec::new();

    // This list contains arguments passed by the user and with all built-in commands separated 
    // (List 1.: 'af', 'file') (List 2.: 'then') (List 3.: 'ad', 'dir')
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
                        Expected output: ('af' 'file' 'then' 'ad' 'dir')
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
                        This will be used to separate SUPER COMMANDS from anything else
                        Expected output: ('af' 'file'), ('then'), ('ad' 'dir')
                     */ 
                    let mut command = Vec::new();
                    let mut index = 0;
                    // println!("DEBUG: I got these words: {:?}", words);
                    while index < words.len() {
                        // println!("DEBUG: Analising word: {}", words[index]);
                        // If built-in keyword appears
                        if SUPER_COMMANDS.contains(&words[index].as_str()) {
                            // println!("DEBUG: Phrase '{}' is a keyword", words[index]);
                            // println!("DEBUG: It's index is: {index}");
                            // Separate keyword from PREVIOUSLY collected words
                            // Expected output: ('af' 'file'), ('then' 'ad' 'dir')
                            let (before_keyword, right) = words.split_at(index);
                            // Convert everything to a vector
                            let (before_keyword, right) = (before_keyword.to_vec(), right.to_vec());
                            println!("DEBUG: Words before keyword: {:?}", before_keyword);

                            // Separate keyword from NEXT words, that are not collected yet
                            // Expected output: ('af' 'file'), ('then'), ('ad' 'dir')
                            let (keyword, after_keyword) = {
                                let (keyword, after_keyword) = right.split_at(1);
                                (keyword.to_vec(), after_keyword.to_vec())
                            };
                            // println!("DEBUG: Keyword: {:?}", keyword);
                            // println!("DEBUG: Words after keyword: {:?}", after_keyword);

                            // Send previous words to "commands"
                            // Example: ('af' 'file')
                            commands.push(before_keyword.to_vec());
                            // Send keyword to "commands" exclusively
                            // Example: ('then')
                            commands.push(keyword.to_vec());
                            // We no longer need to deal with ('af' 'file') and ('then')
                            words = after_keyword.to_vec();
                            // Start over with new words
                            // Example: ('ad' 'dir')
                            index = 0;
                        }
                        // If there is not built-in command 
                        else {
                            command.push(words[index].clone());
                            index += 1;
                            if index == words.len() {
                                // println!("DEBUG: It's so dark and alone here... No keywords!");
                                // println!("DEBUG: I'm adding this command to 'commands' list: {:?}", words);
                                commands.push(words.clone());
                            }
                        }
                    } 
                    // dbg!(&commands);
                    commands
                },
            }
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
    };
}

fn runcommand(args:&[String]) {
    if args.is_empty() || args[0].is_empty() {
        print!("");
    }
    else if let Err(e) = process::Command::new(&args[0]).args(&args[1..]).status() { 
        eprintln!("{}: Command execution failed because of an error: {}", args[0], e.kind()) 
    }
    io::stdout().flush().unwrap();
}