use carrot_libs::{args,input,system};
use std::os::raw::c_int;
use std::process;
use std::io;
use std::io::Write;
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
    let opts = args::opts();
    let (swcs, vals) = args::swcs();

    if opts.is_empty() {
        eprintln!("No commands to execute!");
        process::exit(1);
    }
    let mut desired_uid = 0;
    let mut desired_gid = 0;
    let mut index = 0;
    while index < swcs.len() {
        let s = &swcs[index];
        let v = &vals[index];

        if s != "u" && s != "user" && s != "g" && s != "group" && s != "b" && s != "both" {
            eprintln!("Unknown switch: {s}!"); process::exit(1);
        }
        if s == "u" || s == "user" {
            desired_uid = match v.parse::<u32>() {
                Ok(e) => e,
                Err(e) => {eprintln!("Cannot parse value for a switch: {s}={v}: {:?}", e.kind()); process::exit(1);},
            }
        }
        if s == "g" || s == "group" {
            desired_gid = match v.parse::<u32>() {
                Ok(e) => e,
                Err(e) => {eprintln!("Cannot parse value for a switch: {s}={v}: {:?}", e.kind()); process::exit(1);},
            }
        }
        if s == "b" || s == "both" {
            (desired_uid, desired_gid) = match v.parse::<u32>() {
                Ok(e) => (e,e),
                Err(e) => {eprintln!("Cannot parse value for a switch: {s}={v}: {:?}", e.kind()); process::exit(1);},
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
                        io::stdout().flush().unwrap();
                        if opts.len() == 1 {
                            process::Command::new(&opts[0]).spawn().unwrap().wait_with_output().unwrap();
                        } else {
                            process::Command::new(&opts[0]).args(&opts[1..]).spawn().unwrap().wait_with_output().unwrap();
                        }
                        // Flush stdout
                    } else {
                        println!("What a miss!");
                        process::exit(1);
                    }
                }
            }
        }
        
    }
}
