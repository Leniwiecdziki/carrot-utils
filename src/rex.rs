use carrot_libs::{args,input,system};
use std::os::raw::c_int;
use std::process;

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
    match system::current_user() {
        Ok(e) => {
            if e != desired_uid {
                eprintln!("Failed to change effective UID! Probably, this is not an SUID binary or requested UID is incorrect!");
                process::exit(1);
            }
        },
        Err(e) => {eprintln!("Error: {}", e); process::exit(1);},
    }
    match system::current_group() {
        Ok(e) => {
            if e != desired_gid {
                eprintln!("Failed to change effective GID! Probably, this is not an SUID binary or requested GID is incorrect!");
                process::exit(1);
            }
        },
        Err(e) => {eprintln!("Error: {}", e); process::exit(1);},
    }

    let pass = input::get("Password: ".to_string(), true).join(" ");
    match system::password_check(desired_uid, &pass) {
        Err(e) => {
            eprintln!("Failed to check passwords: {e}");
            process::exit(1);
        }
        Ok(result) => {
            if result {
                println!("Passwords match!");
            } else {
                println!("Passwords mismatch!");
                process::exit(1);
            }
        }
    }
}
