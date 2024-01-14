use std::fs;
use std::collections::HashMap;
use std::io::{self, BufRead, IsTerminal};
use crossterm::{self, execute, terminal, cursor, event};
use crossterm::event::{KeyCode, KeyModifiers, KeyEvent, Event::Key};
use std::process;
mod libargs;

fn prepare_console() {
    // Enter an alternate screen
    execute!(io::stdout(), terminal::EnterAlternateScreen).expect("Cannot change to alternate screen!");
    // Hide cursor
    execute!(io::stdout(), cursor::Hide).expect("Cannot hide a cursor!");
    // Move cursor to the top
    execute!(io::stdout(), cursor::MoveTo(0,0)).expect("Cannot hide a cursor!");
}
fn unprepare_console() {
    // Leave an alternate screen
    execute!(io::stdout(), terminal::LeaveAlternateScreen).expect("Cannot quit from alternate screen!");
    // Hide cursor
    execute!(io::stdout(), cursor::Show).expect("Cannot show a cursor!");
    // Exit raw mode
    terminal::disable_raw_mode().expect("Cannot disable raw mode!");
}

fn main() {
    let opts = libargs::opts();
    let (swcs, vals) = libargs::swcs();
    if !swcs.is_empty() || !vals.is_empty() {
        eprintln!("This program does not need any switches nor values!");
        process::exit(1);
    }

    // Show error when there are no files requested in options by user BUT not when something is piped to our sweet program
    let stdin = io::stdin();
    if opts.is_empty() || !stdin.is_terminal() {
        eprintln!("Type the name of elements to preview!");
        process::exit(1);
    }

    // Show piped stuff
    if !stdin.is_terminal() {
        todo!();
        execute!(io::stdout(), crossterm::terminal::EnterAlternateScreen).expect("Cannot change to alternate screen!");
        for line in stdin.lock().lines() {
            println!("{}", line.expect("Could not read from standard input"));
        }
        
    };
    // Show stuff requested as options
    let mut index = 0;
    while index < opts.len() {
        match fs::read_to_string(&opts[index]) {
            Err(e) => { 
                eprintln!("{}: Cannot preview the file: {:?}!", opts[index], e.kind());
                index += 1;
            },
            Ok(f) => {
                prepare_console();
                // Make a list of lines in a file and associate them with line numbers
                let mut lines = HashMap::new();
                let mut idx = 0;
                for line in f.lines() {
                    let terminal_width = terminal::size().unwrap().0 as usize ;
                    // Make line shorter when it is longer than our terminal
                    if line.len() > terminal_width {
                        // How many times do we need to cut it?
                        let how_many_iterations = line.len() / terminal_width;

                        let mut split_count = 1;
                        while split_count != how_many_iterations+1 {
                            // If iteration was already ran, remove unneeded characters from line
                            let previous_split_count = split_count-1;
                            let n = previous_split_count * terminal_width;
                            let shorter_line = line[n..].split_at(terminal_width);
                            lines.insert(idx, shorter_line.0);
                            idx += 1;
                            if split_count == how_many_iterations {
                                lines.insert(idx, shorter_line.1);
                                idx += 1;
                            };
                            split_count +=1;
                        };
                    } else {
                        lines.insert(idx, line);
                        idx += 1;
                    };
                };
                very_funny(lines);
            },
        };
        index += 1;
    }

}

fn very_funny(content:HashMap<usize, &str>) {
    let terminal_height = terminal::size().expect("Failed to check terminal height!").1.into();
    let lines_count = content.len();

    let mut start = 0;
    let end = terminal_height;

    terminal::enable_raw_mode().expect("Cannot enable raw mode!");
    // Show lines from START (which is 1 by default or something else when user wants to)
    // to end (which is always a terminal height)
    let mut index = start;
    while index < end-1 {
        print_line(content.get_key_value(&index));
        if index != end {
            println!();
        };
        index += 1;
    }
    print_line(content.get_key_value(&index));
    loop {
        let event = event::read().expect("Keyboard event cannot be read!");
        match event {
            // CTRL+C, CTRL+Z, Q: Quit
            Key(KeyEvent {code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL, ..}) |
            Key(KeyEvent {code: KeyCode::Char('z'), modifiers: KeyModifiers::CONTROL, ..}) |
            Key(KeyEvent {code: KeyCode::Char('q'), ..}) => {
                break;
            },
            // Scroll one line down
            Key(KeyEvent {code: KeyCode::Down, ..}) |
            Key(KeyEvent {code: KeyCode::PageDown, ..}) => {
                if index < lines_count-1+end-1 {
                    execute!(io::stdout(), terminal::ScrollUp(1)).unwrap();
                    execute!(io::stdout(), cursor::MoveToRow(end.try_into().unwrap())).unwrap();
                    start += 1;
                    index += 1;
                    print_line(content.get_key_value(&index));
                }
            },
            // Scroll one line up
            Key(KeyEvent {code: KeyCode::Up, ..}) |
            Key(KeyEvent {code: KeyCode::PageUp, ..}) => {
                if start > 0 {
                    execute!(io::stdout(), terminal::ScrollDown(1)).unwrap();
                    execute!(io::stdout(), cursor::MoveToRow(0)).unwrap();
                    start -= 1;
                    index -= 1;
                    print_line(content.get_key_value(&start));
                };
            },
            _ => {
            },
        };
    };
    unprepare_console();
}

fn print_line(line:Option<(&usize, &&str)>) {
    match line {
        Some(value) => {
            // Print line from text file
            print!("{}\r", value.1);
        },
        // If file ended, just show the void...
        None => print!("\r"),
    };
    execute!(io::stdout(), cursor::MoveToColumn(0)).unwrap();
}