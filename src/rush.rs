use std::io;
use std::io::Write;
use std::ops::Index;
use std::os::unix::process::ExitStatusExt;
use std::process;
use std::env;
use std::collections::HashMap;
mod libargs;
mod libinput;

/*
There are three types of commands in RUSH
- Classic commands: When you try to run something like 'git' or 'htop', it will be executed by system with all it's arguments.
- Built-in commands: They also get their arguments as usual BUT they will be executed by shell
- SUPER COMMANDS: They are used to operate on output, exit code, or anything else from previous or next commands

Let's assume that we have a following script:
if my_app do echo it works! end
In this example, super command "if" has to find out if "my_app" succeeded. 
If so, every command from "do" to "end" will be executed
 */ 
const SUPER_COMMANDS:[&str; 3] = ["then", "if", "do"];

/*
This struct will be used as a template for "return" variable.
"return" helps this shell to find commands that reported success on exit or not
You'll find more about it later
 */
struct CommandStatus {
    code: Option<i32>,
    success: bool,
    signal: Option<i32>,
    core_dumped: bool 
}

fn main() {
    let opts = libargs::opts();
    let (swcs, vals) = libargs::swcs();

    // Refuse to run when switches have been passed
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
            // Get commands
            let commands = getinput(String::from("> "));
            /*
            This variable contains all required information about statuses reported by all invoked commands
            For example: When user runs this snippet of code:
            if ad file do
                echo 'Directory added successfully' 
            end
            Rush will check if "return.success" from "commands[1]" is true (because "ad file" is our second command here)  
             */
            let mut returns = HashMap::new();
            // Commands separated by built-in keywords
            for (index, command) in commands.clone().into_iter().enumerate() {
                // Check whether the first argument is a keyword or not
                match command[0].as_str() {
                    "gt" => gt(&command, index, &mut returns),
                    "help" | "?" => help(),
                    "exit" | "quit" | "bye" => exit(&command, index, &mut returns),
                    "then" => report_success(index, &mut returns),
                    "if" => supercmd_if(index, &commands),
                    "do" => supercmd_do(index, &commands, &returns),
                    _ => runcommand(&command, index, &mut returns),
                };
            }
        };
    }
}

fn getinput(prompt:String) -> Vec<Vec<String>> {
    // This list contains all commands passed by the user 
    let mut commands: Vec<Vec<String>> = Vec::new();
    let mut words = libinput::get(prompt);
        /*
        This will be used to separate SUPER COMMANDS from anything else
        Expected output: ('af' 'file'), ('then'), ('ad' 'dir')
        */ 
        let mut command = Vec::new();
        let mut index = 0;
        while index < words.len() {
            // If built-in keyword appears
            if SUPER_COMMANDS.contains(&words[index].as_str()) {
                // Separate keyword from PREVIOUSLY collected words
                // Expected output: ('af' 'file'), ('then' 'ad' 'dir')
                let (before_keyword, right) = words.split_at(index);
                // Convert everything to a vector
                let (before_keyword, right) = (before_keyword.to_vec(), right.to_vec());

                // Separate keyword from NEXT words, that are not collected yet
                // Expected output: ('af' 'file'), ('then'), ('ad' 'dir')
                let (keyword, after_keyword) = {
                    let (keyword, after_keyword) = right.split_at(1);
                    (keyword.to_vec(), after_keyword.to_vec())
                };

                // Send previous words to "commands"
                // Example: ('af' 'file')
                if !before_keyword.is_empty() {
                    // Do not append anything if there is emptyness before keyword!
                    commands.push(before_keyword.to_vec());
                }
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
                };
            };
        };
    commands
}

fn report_success(index:usize, returns:&mut HashMap<usize, CommandStatus>) {
    let command_status = CommandStatus {code: Some(0),success: true,signal: None,core_dumped: false};
    returns.insert(index, command_status);
}
fn report_failure(index:usize, returns:&mut HashMap<usize, CommandStatus>) {
    let command_status = CommandStatus {code: Some(1),success: false,signal: None,core_dumped: false};
    returns.insert(index, command_status);
}

fn gt(args:&Vec<String>, index:usize, returns:&mut HashMap<usize, CommandStatus>) {
    if args.len() == 1 {
        eprintln!("Give me a directory path to go!");
        report_failure(index, returns);
    }
    else if args.len() > 2 {
        eprintln!("Cannot go to multiple directories simultaneously!");
        report_failure(index, returns)
    }
    else {
        match env::set_current_dir(&args[1]) { 
            Err(e) => {
                eprintln!("{}: Cannot go into this directory because of an error: {}", args[1], e.kind());
                report_failure(index, returns);
            },
            Ok(_) => {
                report_success(index, returns);
            }
        };
    };
}

fn help() {
    todo!("Help!");
}

fn exit(args:&Vec<String>, index:usize, returns:&mut HashMap<usize, CommandStatus>) {
    if args.len() == 1 {
        report_failure(index, returns);
        process::exit(0)
    }
    else if args.len() > 2 {
        report_failure(index, returns);
        eprintln!("Cannot exit with multiple exit codes!");
    }
    else {
        match args[1].parse::<i32>() {
            Err(e) => {
                eprintln!("Cannot exit with this code because of an error: {:?}", e.kind());
                report_failure(index, returns);
            },
            Ok(code) => { report_success(index, returns); process::exit(code); },
        }
    };
}

fn runcommand(args:&[String], index:usize, returns:&mut HashMap<usize, CommandStatus>) {
    if args.is_empty() || args[0].is_empty() {
        print!("");
    }
    match process::Command::new(&args[0]).args(&args[1..]).status() { 
        Err(e) => eprintln!("{}: Command execution failed because of an error: {}", args[0], e.kind()),
        Ok(process) => {
            let command_status = CommandStatus {
                code: process.code(),
                success: process.success(),
                signal: process.signal(),
                core_dumped: process.core_dumped()
            };
            returns.insert(index, command_status);
        },
    }
    io::stdout().flush().unwrap();
}

/*
This function only tests if syntax of commands requested by user are correct
It checks if "do" is present and if there are some other super commands except from accepted ones like "then", "and" or "or"
 */
fn supercmd_if(index_of_if:usize, commands: &[Vec<String>]) {
    for (idx, command) in commands[index_of_if..].iter().enumerate() {
        if idx > 0 {
            // Scan every command until you find "do"
            let cur_cmd = &command[0];
            let acceptable_keywords:[&str; 3] = ["and", "or", "then"];

            let found_do = cur_cmd == "do";
            // Break immediately when there is an "do"
            if found_do { break; };

            // Print error when "do" is still not found and there is an super command that isn't accepted
            // In this case, super commands that shouldn't appear between "if" and "do" like "match"
            if SUPER_COMMANDS.contains(&cur_cmd.as_str()) && !acceptable_keywords.contains(&cur_cmd.as_str()) {
                eprintln!("Syntax error! Found operator \"{cur_cmd}\" instead of desired \"do\"!"); 
                process::exit(1);
            }

            // If there are no available commands and "if" is still not found - print and error
            if idx+index_of_if == commands.len()-1 && !found_do {
                eprintln!("Syntax error! Operator \"or\" not found!"); process::exit(1);
            };
        };
    };
}

fn supercmd_do(index_of_do:usize, commands: &[Vec<String>], returns: &HashMap<usize, CommandStatus>) {
    // Initial variables
    let mut index_of_if = 0;
    // Check syntax
    let mut idx = index_of_do-1;
    while idx < index_of_do {
        // Check comments in supercmd_do()
        let cur_cmd = &commands[idx][0];
        let acceptable_keywords:[&str; 3] = ["and", "or", "then"];

        let found_if = cur_cmd == "if";
        if found_if { index_of_if = idx+1;break; };

        if SUPER_COMMANDS.contains(&cur_cmd.as_str()) && !acceptable_keywords.contains(&cur_cmd.as_str()) {
            eprintln!("Syntax error! Found operator \"{cur_cmd}\" instead of desired \"if\"!"); 
            process::exit(1);
        }

        if idx == 0 && !found_if {
            eprintln!("Syntax error! Operator \"if\" wasn't found!"); process::exit(1);
        };
        idx -= 1;
    }
    // Check status of every command after "if" and before "do"
    let mut index = index_of_if;
    println!("Starting point: {index}");
    while index < index_of_do {
        println!("There is a command {:?} with index {index} and status {:?}", commands[index], returns.get(&index).unwrap().success );
        index += 1;
    }

}