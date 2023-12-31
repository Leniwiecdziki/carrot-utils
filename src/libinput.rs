#![allow(unused_variables)]
use std::process;
use std::io;
use std::io::Write;
use crossterm::terminal::{Clear, ClearType};
use crossterm::event::{self, Event::Key, KeyEvent, KeyCode, KeyModifiers};

fn flush() {
    io::stdout().flush().expect("Failed to flush terminal output!")
}

pub fn get(prompt:String) -> Vec<String> {
    // FOR ALL COMMENTS BELLOW: Assume, that user typed this command into a shell: af file then ad dir
    // This variable contains full line typed by the user (List 1.: 'af file then ad dir')
    let mut input = String::new();

    // This list contains arguments passed by the user and with all built-in commands separated 
    // (List 1.: 'af', 'file') (List 2.: 'then') (List 3.: 'ad', 'dir')
    let words: Vec<String> = Vec::new();
    
    // Print a prompt
    print!("{prompt}");

    // Flush stdout to print the prompt
    io::stdout().flush().expect("Cannot flush output!");
        // Read line into "input"
        // Process each character written on keyboard
        let initial_cur_pos = crossterm::cursor::position().expect("Failed to obtain cursor position!").0;
        let mut actual_cur_pos = initial_cur_pos;
        loop {
            // Go to raw mode to get more control over terminal
            crossterm::terminal::enable_raw_mode().unwrap();

            // Flush on start and end of the loop
            flush();

            let event = event::read().unwrap();
            match event {
                // CTRL+Z: Quit
                Key(KeyEvent {code: KeyCode::Char('z'), modifiers: KeyModifiers::CONTROL, ..}) => {
                    // Disable raw mode and quit
                    crossterm::terminal::disable_raw_mode().expect("Cannot quit from raw terminal mode!");
                    println!();
                    process::exit(1);
                },

                // ANY OTHER: Show it on keyboard and add it to "input" variable
                Key(KeyEvent {code: KeyCode::Char(c), ..}) => {
                    // Insert a char in "input" on position where cursor is located
                    input.insert((actual_cur_pos-initial_cur_pos).into(), c);
                    // Move cursor to right as we type
                    actual_cur_pos += 1;
                },
                
                // ARROWS: Cursor movement
                Key(KeyEvent {code: KeyCode::Left, ..}) => {
                    if actual_cur_pos > initial_cur_pos {
                        // Move cursor to left
                        actual_cur_pos -= 1;
                    }
                    else {
                        print!("\x07");
                        continue;
                    };
                    
                },
                Key(KeyEvent {code: KeyCode::Right, ..}) => {
                    if actual_cur_pos < (input.len()+2).try_into().unwrap() {
                        // Move cursor to right
                        actual_cur_pos += 1;
                    }
                    else {
                        print!("\x07");
                        continue;
                    };
                },
                Key(KeyEvent {code: KeyCode::Home, ..}) => {
                    // Move cursor back to the prompt
                    actual_cur_pos=initial_cur_pos;
                }
                Key(KeyEvent {code: KeyCode::End, ..}) => {
                    // Move where "input" is reaching it's end
                    actual_cur_pos=(input.len() as u16)+initial_cur_pos;
                }

                // ENTER: Quickly append newline character to "input" and stop waiting for input by breaking out of the loop
                Key(KeyEvent {code: KeyCode::Enter, ..}) => {
                    input.push('\n');
                    crossterm::terminal::disable_raw_mode().unwrap();
                    break;
                },
                // BACKSPACE: Remove character on cursor
                Key(KeyEvent {code: KeyCode::Backspace, ..}) => {
                    if actual_cur_pos > initial_cur_pos {
                        // Delete from "input" where cursor is located
                        if actual_cur_pos-initial_cur_pos != input.len().try_into().unwrap() {
                            input.remove((actual_cur_pos-initial_cur_pos-1).into());
                        }
                        else {
                            input.pop();
                        };
                        // Move cursor
                        actual_cur_pos -= 1;
                    }
                    else {
                        print!("\x07");
                    };
                    
                },
                // OTHER
                _ => {
                    // Bell!
                    print!("\x07");
                },
            };
            // Move to start of the column
            print!("\r");
            // Clear everything on that line
            print!("{}", Clear(ClearType::CurrentLine));
            // Show prompt and contents of input
            print!("{}{}", prompt, input);
            // Move cursor to position defined in "actual_cur_pos"
            print!("{}", crossterm::cursor::MoveToColumn(actual_cur_pos)); 
            // Flush on start and end of the loop
            flush();
        };
        // Quit from raw mode when we're out of the loop
        print!("\n\r");
        crossterm::terminal::disable_raw_mode().unwrap();
        /*
            Character division helps to find individual arguments (words)
            Expected output: ('af' 'file' 'then' 'ad' 'dir')
        */
        input.split_whitespace().map(str::to_string).collect()
}