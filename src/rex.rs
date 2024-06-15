use carrot_libs::{args,input,system};
use std::io::Write;
use std::{io, os::raw::c_int};
use std::process;
use std::collections::HashMap;
use serde_derive::{Serialize, Deserialize};

// This is where configuration structures for REX program are defined.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Rex {
    allow_users: Vec<String>,
    allow_groups: Vec<String>,
    allow_users_nopass: Vec<String>,
    allow_groups_nopass: Vec<String>,
    deny_users: Vec<String>,
    deny_groups: Vec<String>,
    allow_cmd: HashMap<String,String>,
    allow_cmd_nopass: HashMap<String,String>,
    deny_cmd: HashMap<String,String>,
}
// Default settings for "Rex"
impl std::default::Default for Rex {
    fn default() -> Self {
        Self {
            allow_users: vec![],
            allow_groups: vec![String::from("admin")],
            allow_users_nopass: vec![String::from("root")],
            allow_groups_nopass: vec![],
            deny_users: vec![],
            deny_groups: vec![],
            allow_cmd: HashMap::new(),
            allow_cmd_nopass: HashMap::new(),
            deny_cmd: HashMap::new(),
        }
    }
}
pub const CONFIG_LOCATION_REX:&str = "/etc/rex.toml";

fn main() {
    let args = args::args();
    let opts = args::opts();
    let (swcs, vals) = args::swcs();

    if opts.is_empty() {
        eprintln!("No commands to execute!");
        process::exit(1);
    }

    // Find out, where the first OPTION is defined in arguments
    // excluding argument number 0 which is a program's name
    let mut index_of_first_opt = 0;
    for (i, a) in args[1..].iter().enumerate() {
        // WARNING: enumerate() counts from 0!
        println!("{}: {}", i+1, a);
        if !a.starts_with('-') {
            index_of_first_opt = i+1;
            // Break after the first possible occurence
            break;
        }
    }

    // Find out, where the first SWITCH is defined in arguments
    // excluding argument number 0 which is a program's name
    let mut index_of_first_swc = 0;
    for (i, a) in args[1..].iter().enumerate() {
        if a.starts_with('-') {
            index_of_first_swc = i+1;
            break;
        }
    }

    // The program is built in the way that requires user to enter all switches first!
    // Options have to be defined later.
    // This is bad: rex page /private_content -u=1001 -g=1002 (-u=1001 and -g=1002 will be passed to page)
    // This is ok: rex -u=1001 -g=1002 page /private_content

    // This is caused by the fact, that REX have to send some switches to the executed program itself
    // Otherwise, something like "rex echo -ne hello" would be impossible to run with this tool.

    // That is the reason why switches should be defined before any other option

    let mut desired_uid = 0;
    let mut desired_gid = 0;
    let mut index = 0;

    let there_are_switches_before_options = index_of_first_swc < index_of_first_opt;

    while index < swcs.len() && there_are_switches_before_options {
        let s = &swcs[index];
        let v = &vals[index];

        if s != "u" && s != "user" && s != "g" && s != "group" && s != "b" && s != "both" {
            eprintln!("Unknown switch: {s}!"); process::exit(1);
        }
        if s == "u" || s == "user" {
            desired_uid = match v.parse::<u32>() {
                Ok(e) => e,
                Err(e) => {eprintln!("Failed to convert switch value to a number: {s}={v}: {:?}", e.kind()); process::exit(1);},
            }
        }
        if s == "g" || s == "group" {
            desired_gid = match v.parse::<u32>() {
                Ok(e) => e,
                Err(e) => {eprintln!("Failed to convert switch value to a number: {s}={v}: {:?}", e.kind()); process::exit(1);},
            }
        }
        if s == "b" || s == "both" {
            (desired_uid, desired_gid) = match v.parse::<u32>() {
                Ok(e) => (e,e),
                Err(e) => {eprintln!("Failed to convert switch value to a number: {s}={v}: {:?}", e.kind()); process::exit(1);},
            }
        }
        index += 1;
    }

    // Use C library (bruh...)
    extern "C" {
        // Define seteuid function that takes c_int as an argument
        fn seteuid(num:c_int) -> i32;
        fn setegid(num:c_int) -> i32;
    }
    unsafe {
        // Change effective UID to the user of choice
        let retcode_u = seteuid(desired_uid.try_into().unwrap());
        let retcode_g = setegid(desired_gid.try_into().unwrap());
        if retcode_u == -1 || retcode_g == -1 {
            eprintln!("Failed to change effective ID! Probably, this is not an SUID binary or requested ID is incorrect!");
            process::exit(0);
        }
    };
    // Sanity check - test if the running user/group is what we really want.
    match system::current_user_effective() {
        Ok(e) => {
            if e != desired_uid {
                eprintln!("Failed to change effective UID! Probably, this is not an SUID binary or requested UID is incorrect!");
                process::exit(1);
            }
        },
        Err(e) => {eprintln!("Error: {}", e); process::exit(1);},
    }
    match system::current_group_effective() {
        Ok(e) => {
            if e != desired_gid {
                eprintln!("Failed to change effective GID! Probably, this is not an SUID binary or requested GID is incorrect!");
                process::exit(1);
            }
        },
        Err(e) => {eprintln!("Error: {}", e); process::exit(1);},
    }

    let real_uid = match system::current_user_real() {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Failed to check real UID: {}!", e);
            process::exit(1);
        }
    };

    // If REX is being used to change EUID to a currently used EUID - do not prompt for a password.
    match input::get("Password: ", true) {
        Err(e) => {
            eprintln!("Failed to get user input: {}!", e);
            process::exit(1);
        }
        Ok(ret) => {
            let pass = ret.join(" ");
            match system::password_check(real_uid, pass) {
                Err(e) => {
                    eprintln!("Failed to check password: {e}");
                    process::exit(1);
                }
                Ok(result) => {
                    if result {
                        exec(&args[index_of_first_opt..]);
                    } else {
                        println!("Incorrect password!");
                        process::exit(1);
                    }
                }
            }
        }
        
    }
}

fn exec(args:&[String]) {
    // Flush terminal for commands without newline character at the end
    io::stdout().flush().unwrap();
    if args.len() == 1 {
        process::Command::new(&args[0]).spawn().unwrap().wait_with_output().unwrap();
    } else {
        process::Command::new(&args[0]).args(&args[1..]).spawn().unwrap().wait_with_output().unwrap();
    }
}
