use carrot_libs::{args,input,system};
use std::process;
use std::fs;
use std::io;
use std::path::Path;
use serde_derive::{Serialize, Deserialize};

// This sctructure defined typical list of all users
#[derive(Serialize, Deserialize, Debug)]
struct UsersList {
    users: Vec<User>,
}

// And this is a structure of one, particular user
#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: u32,
    name: String,
    description: String,
    groups: Vec<u32>,
    password: String,
    password_change_date: i64,
    password_expiration_date: i64,
    can_change_password: bool,
    creation_date: i64,
    locked: bool,
    lock_date: i64,
    profile_dir: String,
    shell: String,
}

// Default settings for "UsersList"
impl ::std::default::Default for UsersList {
    fn default() -> Self {
        Self {
            users: Vec::from([
                User {
                    id:0,
                    name:String::from("root"),
                    groups: Vec::from([0]),
                    description: String::from(""),
                    password:String::from(""),
                    password_change_date: chrono::offset::Utc::now().timestamp(),
                    password_expiration_date: 0,
                    can_change_password: true,
                    creation_date: chrono::offset::Utc::now().timestamp(),
                    locked: false,
                    lock_date: 0,
                    profile_dir: String::from("/root/"),
                    shell: String::from("/bin/rush"),
                }
            ])
        }
    }
}
const CONFIG:&str = "/etc/users.toml";

fn main() {
    let opts = args::opts();
    let (swcs, vals) = args::swcs();

    if opts.len() == 1 && opts[0] == "id" {
        let current_uid = match system::current_user() {
            Ok(e) => e,
            Err(e) => {eprintln!("Error: {}", e); process::exit(1);},
        };
        println!("{current_uid}");
        process::exit(0);
    }
    if opts.len() < 2 {
        eprintln!("Missing action and user ID/name!");
        process::exit(1);
    }
    if opts.len() > 2 {
        eprintln!("Only one user can be modified at the same time!");
        process::exit(1);
    }

    // What do to with the user
    let action = opts[0].clone();
    // Which user to edit? - this can sometimes be a user name or ID
    let request = opts[1].clone();

    // Open users.toml for configuration
    let cfg:UsersList = match confy::load_path(CONFIG) {
        Err(e) => {
            eprintln!("Failed to open configuration. Probably, you don't have sufficient permissions: {}", e);
            process::exit(1);
        },
        Ok(e) => {
            e
        }
    };

    // Check current user ID
    let current_uid = match system::current_user() {
        Ok(e) => e,
        Err(e) => {eprintln!("Error: {}", e); process::exit(1);},
    };
    // Are we running as root?
    let running_as_root = match system::isroot() {
        Ok(e) => e,
        Err(e) => {eprintln!("Error: {}", e); process::exit(1);},
    };
    // If we're not running as root:
    // check if we can change our password
    // resolve user name
    let mut output_chpass = false;
    let mut output_name = "".to_string();
    if running_as_root {
        output_chpass = true;
    } 
    else {
        for u in &cfg.users {
            if u.id == current_uid {
                output_chpass = u.can_change_password;
                output_name.clone_from(&u.name);
            }
        }
    };
    let request_is_number = request.parse::<u32>().is_ok();
    let request_points_to_current_user = if request_is_number {
        request.parse::<u32>().unwrap() == current_uid
    } else {
        request == output_name
    };

    // VERY IMPORTANT:
    // NO ONE but the root user should have the permission to use this program!
    // The only exception are:
    // 1. Usage of "update" action with "-pass" BUT ONLY if "can_change_password" is set to "true" and "request" points to currently logged in user.
    // 2. Usage of "list" but only when "request" points to currently logged in user
    // 3. Usage of "whois" or "isthere"
    if !running_as_root && (

    (action == "add" || action == "del") ||
    (action == "update" &&
    ( !output_chpass || !request_points_to_current_user || swcs.len()>1 || (swcs.len()==1 && !swcs.contains(&"pass".to_string())) )
    ) ||
    (action == "list" && !request_points_to_current_user)

    )
    {
        eprintln!("You are not permitted to perform this operation!"); process::exit(1);
    }


    // Define some default settings for a user
    let mut id = system::getpref_or_exit("default_user_pref", "minimal_uid").parse::<u32>().unwrap();
    let mut name = "".to_string();
    let mut description = "".to_string();
    let mut password = "".to_string();
    let password_change_date = chrono::offset::Utc::now().timestamp();
    let mut password_expiration_date = 0_i64;
    let mut can_change_password = system::getpref_or_exit("default_user_pref", "can_change_password").parse::<bool>().unwrap();
    let creation_date = chrono::offset::Utc::now().timestamp();
    let mut locked = system::getpref_or_exit("default_user_pref", "locked").parse::<bool>().unwrap();
    let mut lock_date = 0_i64;
    let mut profile_dir = system::getpref_or_exit("default_user_pref", "profile_dir");
    let mut shell = system::getpref_or_exit("default_user_pref", "shell");
    
    let mut index = 0;
    while index < swcs.len() {
        let s = swcs[index].clone();
        let v = vals[index].clone();

        if v.is_empty() && (s=="id"||s=="name"||s=="desc"||s=="expire"||s=="chpass"||s=="lock"||s=="lockdate"||s=="profile"||s=="shell") {
            eprintln!("This switch requires a value: {s}!"); process::exit(1); 
        }
        if action != "add" && action != "del" && action != "update" && (s=="id"||s=="desc"||s=="pass"||s=="expire"||s=="chpass"||s=="lock"||s=="lockdate"||s=="profile"||s=="shell") {
            eprintln!("Action \"{action}\" does not accept this switch: {s}!"); process::exit(1); 
        }
        if action != "update" && s=="name" {
            eprintln!("Action \"{action}\" does not accept this switch: {s}!"); process::exit(1); 
        }

        if s == "id" {
            id = match v.parse::<u32>() {
                Err(e) => {
                    eprintln!("Requested ID cannot be parsed: {:?}!", e.kind());
                    process::exit(1);
                }
                Ok(e) => e,
            };
        }
        else if s == "name" {name.clone_from(&v)}
        else if s == "desc" {description.clone_from(&v)}
        else if s == "pass" {
            // If password is not passed in a value
            if v.is_empty() {
                // Get passwords
                let pass_probe1 = input::get("Password: ", true);
                let pass_probe2 = input::get("Password (once again): ", true);
                // Catch all possible errors
                if pass_probe1.is_err() || pass_probe2.is_err() {
                    eprintln!("Failed to get user input!");
                    process::exit(1);
                };
                let min_allowed_pass_len = system::getpref_or_exit("default_user_pref", "password_minimum_len").parse::<u64>().unwrap();
                let max_allowed_pass_len = system::getpref_or_exit("default_user_pref", "password_maximum_len").parse::<u64>().unwrap();
                if (pass_probe1.clone().unwrap().len() as u64) < min_allowed_pass_len ||
                (pass_probe2.clone().unwrap().len() as u64) < min_allowed_pass_len 
                {
                    eprintln!("System admin requires your password to have at least {} characters!", min_allowed_pass_len);
                    process::exit(1);
                }

                if (pass_probe1.clone().unwrap().len() as u64) > max_allowed_pass_len ||
                (pass_probe2.clone().unwrap().len() as u64) > max_allowed_pass_len 
                {
                    eprintln!("System admin requires your password to have up to {} characters!", max_allowed_pass_len);
                    process::exit(1);
                }

                if pass_probe1.clone().unwrap() == pass_probe2.unwrap() {
                    password = system::encrypt(&pass_probe1.clone().unwrap().join(" "));
                } 
                else {
                    eprintln!("Passwords do not match! Exiting.");
                    process::exit(1);
                }
            }
            // If password is being passed by the user in a value
            else {
                eprintln!("Setting up passwords this way is insecure!");
                eprintln!("Try using -pass without any value.");
                password = system::encrypt(&v);
            }
        }
        else if s == "expire" {
            password_expiration_date = match v.parse::<i64>() {
                Err(e) => {
                    eprintln!("Requested password expiration date cannot be parsed: {:?}!", e.kind());
                    process::exit(1);
                }
                Ok(e) => e,
            };
        }
        else if s == "chpass" {
            can_change_password = match v.parse::<bool>() {
                Err(e) => {
                    eprintln!("Requested password modifcation status cannot be parsed: {:?}!", e);
                    process::exit(1);
                }
                Ok(e) => e,
            };
        }
        else if s == "lock" {
            locked = match v.parse::<bool>() {
                Err(e) => {
                    eprintln!("Requested lock status cannot be parsed: {:?}!", e);
                    process::exit(1);
                }
                Ok(e) => e,
            };
        }
        else if s == "lockdate" {
            lock_date = match v.parse::<i64>() {
                Err(e) => {
                    eprintln!("Requested account lock date cannot be parsed: {:?}!", e.kind());
                    process::exit(1);
                }
                Ok(e) => e,
            };
        }
        else if s == "profile" {profile_dir.clone_from(&v)}
        else if s == "shell" {shell.clone_from(&v)}
        else {
            eprintln!("Unknown switch: {s}");
        }
        index += 1;
    }

    // Get the action and do what is needed
    match action.as_str() {
        "init" => {
            confy::store_path(CONFIG, UsersList::default()).unwrap();
        }
        "add" => {
            // If the requested content is a number - typically this means that user wants to
            // create a user giving only ID. This is impossible to do.
            // use: userutil add someone -id=69 instead of userutil add 69
            let request_is_number = request.parse::<i64>().is_ok();
            if request_is_number {
                eprintln!("User ID is not an accepted option while adding!");
                process::exit(1);
            }
            // Check if user name is already reserved
            if isthere(&request, &cfg.users) {
                eprintln!("The user name \"{}\" is already reserved!", request);
                process::exit(1);
            }

            // Check if user ID is already reserved
            while isthere(&id.to_string(), &cfg.users) {
                // Break if UID was implicitly set by the user
                if swcs.contains(&"id".to_string()) {
                    eprintln!("The user ID \"{}\" is already reserved!", id);
                    process::exit(1);
                }
                // Find closest unused uid
                else {
                    id += 1;
                }
            }
            
            // Copy current user config
            let mut copy = cfg.users.clone();
            // Append a new user
            copy.push( User {
                        id,
                        name: request.clone(),
                        groups: Vec::from([]),
                        description,
                        password, password_change_date, password_expiration_date, can_change_password,
                        creation_date,
                        locked, lock_date, profile_dir: profile_dir.clone(), shell,
                    } );
            // Create a new config object
            let newconfig = UsersList {
                users: copy,
            };
            // Add new contents
            confy::store_path(CONFIG, newconfig).unwrap();

            // Create profile directory
            let profile_dir = format!("{}/{}", profile_dir.clone(), request.clone());
            if let Err(e) = fs::create_dir_all(profile_dir.clone()) {
                eprintln!("{}: Failed to create user profile directory!: {}", profile_dir, e.kind());
                eprintln!("Created a user without a home directory!");
                process::exit(1);
            };
            let def_prof = system::getpref_or_exit("default_user_pref", "default_profile_dir");

            if let Err(e) = copy_dir_all(def_prof, profile_dir.clone()) {
                eprintln!("{}: Copying from default profile to a user profile failed: {}!", profile_dir, e.kind());
                eprintln!("Created a user with empty home directory!");
                process::exit(1);
            };
        },
        "del" => {
            // This is pretty much self explanatory
            // I described similiar code in matching case above.
            // Check if user is already added
            if !isthere(&request, &cfg.users) {
                eprintln!("User with name \"{}\" does not exist!", request);
                process::exit(1);
            }
            if !isthere(&id.to_string(), &cfg.users) {
                eprintln!("User with ID \"{}\" does not exist!", request);
                process::exit(1);
            }
            // Copy current user config
            let mut copy = cfg.users.clone();
            // Find and remove a user with the name or ID that is exact to the requested one 
            let mut i = 0;
            while i < copy.len() {
                if copy[i].name == request || copy[i].id.to_string() == request {
                    copy.remove(i);
                } else {
                    i+=1;
                }
            }
            // Create a new config object
            let newconfig = UsersList {
                users: copy,
            };
            // Add new contents
            confy::store_path(CONFIG, newconfig).unwrap();

        },
        "update" => {
            // Check if user is already added
            if !isthere(&request, &cfg.users) {
                eprintln!("User with name \"{}\" does not exist!", request);
                process::exit(1);
            }
            if !isthere(&id.to_string(), &cfg.users) {
                eprintln!("User with ID \"{}\" does not exist!", request);
                process::exit(1);
            }
            // Copy current user config
            let mut copy = cfg.users.clone();
            let mut user_to_update = None;
            // Find and a user with the name or ID that is exact to the requested one 
            // Remove that match from users list
            let mut i = 0;
            while i < copy.len() {
                if copy[i].name == request || copy[i].id.to_string() == request {
                    user_to_update = Some(copy[i].clone());
                    copy.remove(i);
                } else {
                    i+=1;
                }
            }
            if user_to_update.is_none() {
                eprintln!("This program contradicts itself! User was found in one part of the program and not in the other. This is a bug.");
                process::exit(1);
            }
            // If user supplied some switch, use the value from switch
            // if not, use values that are already defined for him/her/whatever
            let id = if swcs.contains(&"id".to_string()) { 
                id
            } else { 
                user_to_update.clone().unwrap().id
            };
            let name = if swcs.contains(&"name".to_string()) {
                name
            } else {
                user_to_update.clone().unwrap().name
            };
            // left groups unchanged
            let description = if swcs.contains(&"desc".to_string()) {
                description
            } else {
                user_to_update.clone().unwrap().description
            };
            let password = if swcs.contains(&"pass".to_string()) {
                password
            } else {
                user_to_update.clone().unwrap().password
            };
            // change the table bellow only if -pass is supplied
            let password_change_date = if swcs.contains(&"pass".to_string()) {
                chrono::offset::Utc::now().timestamp()
            } else {
                user_to_update.clone().unwrap().password_change_date
            };
            let password_expiration_date = if swcs.contains(&"expire".to_string()) {
                password_expiration_date
            } else {
                user_to_update.clone().unwrap().password_expiration_date
            };
            let can_change_password = if swcs.contains(&"chpass".to_string()) {
                can_change_password
            } else {
                user_to_update.clone().unwrap().can_change_password
            };
            // left creation date unchanged
            let locked = if swcs.contains(&"lock".to_string()) {
                locked
            } else {
                user_to_update.clone().unwrap().locked
            };
            let lock_date = if swcs.contains(&"lockdate".to_string()) {
                lock_date
            } else {
                user_to_update.clone().unwrap().lock_date
            };
            let profile_dir = if swcs.contains(&"profile".to_string()) {
                profile_dir
            } else {
                user_to_update.clone().unwrap().profile_dir
            };
            let shell = if swcs.contains(&"shell".to_string()) {
                shell
            } else {
                user_to_update.clone().unwrap().shell
            };
            // Append a new user
            copy.push( User {
                        id,
                        name,
                        groups: user_to_update.clone().unwrap().groups,
                        description,
                        password, password_change_date, password_expiration_date, can_change_password,
                        creation_date: user_to_update.clone().unwrap().creation_date,
                        locked, lock_date, profile_dir, shell,
                    } );
            // Create a new config object
            let newconfig = UsersList {
                users: copy,
            };
            // Add new contents
            confy::store_path(CONFIG, newconfig).unwrap();
        },
        "list" => {
            for user in cfg.users {
                if user.name == request || user.id.to_string() == request {
                    println!("{:#?}", user);
                }
            }
        },
        "isthere" => {
            println!("{}", isthere(&request, &cfg.users));
        }
        "whois" => {
            let request_is_number = request.parse::<i64>().is_ok();
            for user in cfg.users {
                if (!request_is_number && user.name == request) || (request_is_number && user.id.to_string() == request) {
                    println!("{}:{}", user.id, user.name);
                }
            }
        }
        _ => {
            eprintln!("Unknown action!");
            process::exit(1);
        }
    }
}

fn isthere(request:&String, users_list:&[User]) -> bool {
    for (count, user) in users_list.iter().enumerate() {
        if user.name == *request || user.id.to_string() == *request {
            return  true;
        }
        if count == 0 && count == users_list.len() {
            return false;
        }
    }
    false
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}