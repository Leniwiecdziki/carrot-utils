use carrot_libs::args;
use std::os::raw::c_int;
use std::process;

fn main() {
    let desired_uid = 0;

    // Use C library
    #[link(name = "c")]
    extern "C" {
        // Define seteuid function that takes c_int as an argument
        fn seteuid(num:c_int) -> i32;
        // And geteuid to check the effective uid
        // it does not need any argument
        fn geteuid() -> i32;
    }
    unsafe {
        let retcode = seteuid(0);
        if retcode == -1 {
            eprintln!("Failed to set EUID! Probably, this is not an SUID binary or requested UID is not a proper number!");
            process::exit(0);
        }
        let retcode = geteuid();
        if retcode == -1 {
            eprintln!("Unable to get EUID! Probably, this is not an SUID binary.");
            process::exit(0);
        }
        else if retcode != desired_uid {
            eprintln!("EUID is not correct! Probably, this is not an SUID binary.");
            process::exit(0);
        }
        else {
            println!("OK!");
        }
    };
}