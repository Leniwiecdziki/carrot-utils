use carrot_libs::args;
use std::process;
use serde_derive::{Serialize, Deserialize};

// List of all users
#[derive(Serialize, Deserialize, Debug)]
struct UsersList {
    users: Vec<User>,
}

// One, particular user
#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    name: String,
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

impl ::std::default::Default for UsersList {
    fn default() -> Self { 
        Self {
            users: Vec::from([
                User {
                    id:0,
                    name:String::from("root"),
                    groups: Vec::from([0]),
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

    if opts.len() < 2 {
        eprintln!("Missing action and user name!");
        process::exit(1);
    }
    if opts.len() > 2 {
        eprintln!("Only one user can be modified at the same time!");
        process::exit(1);
    }

    let action = opts[0].clone();
    let request = opts[1].clone();

    let mut id = 1000;
    let mut password = "".to_string();
    let password_change_date = chrono::offset::Utc::now().timestamp();
    let mut password_expiration_date = 0_i64;
    let mut can_change_password = true;
    let creation_date = chrono::offset::Utc::now().timestamp();
    let mut locked = false;
    let mut lock_date = 0_i64;
    let mut profile_dir = format!("/home/{request}");
    let mut shell = "/bin/rush".to_string();
    
    let mut index = 0;
    while index < swcs.len() {
        let s = swcs[index].clone();
        let v = vals[index].clone();

        if s == "id" {
            id = match v.parse::<u32>() {
                Err(e) => {
                    eprintln!("Requested ID cannot be parsed: {:?}!", e.kind());
                    process::exit(1);
                }
                Ok(e) => e,
            };
        };

        if s == "pass" {
            todo!();
        }

        if s == "expire" {
            password_expiration_date = match v.parse::<i64>() {
                Err(e) => {
                    eprintln!("Requested password expiration date cannot be parsed: {:?}!", e.kind());
                    process::exit(1);
                }
                Ok(e) => e,
            };
        }

        if s == "chpass" {
            can_change_password = match v.parse::<bool>() {
                Err(e) => {
                    eprintln!("Requested password modifcation status cannot be parsed: {:?}!", e);
                    process::exit(1);
                }
                Ok(e) => e,
            };
        }

        if s == "lock" {
            locked = match v.parse::<bool>() {
                Err(e) => {
                    eprintln!("Requested lock status cannot be parsed: {:?}!", e);
                    process::exit(1);
                }
                Ok(e) => e,
            };
        }

        if s == "lockdate" {
            lock_date = match v.parse::<i64>() {
                Err(e) => {
                    eprintln!("Requested account lock date cannot be parsed: {:?}!", e.kind());
                    process::exit(1);
                }
                Ok(e) => e,
            };
        }

        if s == "profile" {profile_dir.clone_from(&v)}

        if s == "shell" {shell.clone_from(&v)}

        index += 1;
    }

    let cfg:UsersList = match confy::load_path(CONFIG) {
        Err(e) => {
            eprintln!("Failed to open configuration. Probably, you don't have sufficient permissions: {}", e);
            process::exit(1);
        },
        Ok(e) => {
            e
        }
    };

    match action.as_str() {
        "init" => {
            confy::store_path(CONFIG, UsersList::default()).unwrap();
        }
        "add" => {
            if isthere(&request, &cfg.users) {
                eprintln!("This user already exists!");
                process::exit(1);
            }
            let request_is_number = request.parse::<i64>().is_ok();
            if request_is_number {
                eprintln!("User ID is not accepted as an option while adding!");
                process::exit(1);
            }
            
            let newuser = User {
                id,
                name: request,
                groups: Vec::from([]),
                password,
                password_change_date,
                password_expiration_date,
                can_change_password,
                creation_date,
                locked,
                lock_date,
                profile_dir,
                shell,
            };

            confy::store_path(CONFIG, newuser).unwrap();
        },
        "del" => {
            if isthere(&request, &cfg.users) {
                eprintln!("This user already exists!");
                process::exit(1);
            }
            let request_is_number = request.parse::<i64>().is_ok();
            if request_is_number {
                eprintln!("User ID is not accepted as an option while deleting!");
                process::exit(1);
            }

            // Find the line where configuration of particular user starts
            // Remove 14 lines from file including [[user]] table above it and reduntant white line.

            todo!();

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
        if count == 0 {
            return false;
        }
    }
    false
}