#![allow(unused_variables)]
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
        let cursor_y = crossterm::cursor::position().expect("Failed to obtain cursor position!").0;
        let mut input_index = cursor_y;
        loop {
            // Go to raw mode to get more control over terminal
            crossterm::terminal::enable_raw_mode().unwrap();

            // Flush on start and end of the loop
            flush();

            let event = event::read().unwrap();
            match event {
                // CTRL+Z: Quit
                Key(KeyEvent {code: KeyCode::Char('z'), modifiers: KeyModifiers::CONTROL, ..}) => {
                    crossterm::terminal::disable_raw_mode().expect("Cannot quit from raw terminal mode!");
                    input = "exit 1".to_string();
                    break;
                },

                // ANY OTHER: Show it on keyboard and add it to "input" variable
                Key(KeyEvent {code: KeyCode::Char(c), ..}) => {
                    input_index += 1;
                    input.push(c);
                },
                
                // ARROWS: Cursor movement
                Key(KeyEvent {code: KeyCode::Left, ..}) => {
                    if input_index > cursor_y {
                        input_index -= 1;
                        print!("{}", crossterm::cursor::MoveToColumn(input_index));
                        continue;
                    }
                    else {
                        print!("\x07");
                        continue;
                    };
                    
                },
                Key(KeyEvent {code: KeyCode::Right, ..}) => {
                    if input_index < (input.len()+2).try_into().unwrap() {
                        input_index += 1;
                        print!("{}", crossterm::cursor::MoveToColumn(input_index));
                        continue;
                    }
                    else {
                        print!("\x07");
                        continue;
                    };
                },

                // ENTER: Quickly append newline character to "input" and stop waiting for input by breaking out of the loop
                Key(KeyEvent {code: KeyCode::Enter, ..}) => {
                    input.push('\n');
                    crossterm::terminal::disable_raw_mode().unwrap();
                    break;
                },
                // BACKSPACE: Remove character on cursor
                Key(KeyEvent {code: KeyCode::Backspace, ..}) => {
                    if input_index-cursor_y != input.len().try_into().unwrap() {
                        input.remove((input_index-cursor_y).into());
                        crossterm::terminal::disable_raw_mode().unwrap();
                    }
                    else {
                        input.pop();
                    }
                },

                _ => {
                    input = "exit 1".to_string();
                    break;
                },
            };
            // Show what's in input
            print!("\r");
            print!("{}", Clear(ClearType::CurrentLine));
            print!("{}{}", prompt, input);
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