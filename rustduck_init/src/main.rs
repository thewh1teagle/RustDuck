use std::{path::PathBuf, io::Write, process, fs::File};
use std::process::{Command, Stdio};
use std::{fs, panic};

use serde::{Serialize, Deserialize};
use auto_launch::*;
use home;

use sysinfo::{System, SystemExt, ProcessExt};


#[cfg(not(windows))]
macro_rules! get_binary_bytes {
    () => {
        include_bytes!("../../rustduck/target/release/rustduck")
    };
}

#[cfg(windows)]
macro_rules! get_binary_bytes {
    () => {
        include_bytes!("..\\..\\rustduck\\target\\release\\rustduck.exe")
    };
}


#[cfg(not(windows))]
macro_rules! get_binary_name {
    () => {
        "rustduck"
    };
}

#[cfg(windows)]
macro_rules! get_binary_name {
    () => {
        "rustduck.exe"
    };
}



#[derive(Debug, Deserialize, Serialize)]
struct Config {
    token: String,
    duration: String,
    domains: Vec<String>,
}

fn get_home() -> PathBuf {
    let home_path: Option<PathBuf> = home::home_dir();
    return home_path.unwrap();
}

fn create_directory(path: &str, exist_ok: bool) -> std::io::Result<()> {
    if exist_ok && fs::metadata(path).is_ok() {
        // Directory already exists, and exist_ok is true, so we return Ok.
        Ok(())
    } else {
        fs::create_dir_all(path)?;
        Ok(())
    }
}

fn start_program(dir_path: &PathBuf) {
    let program_path = dir_path.join(get_binary_name!());
    Command::new(program_path)
    .current_dir(dir_path)
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .spawn()
    .expect("Failed to start rustduck");
    println!("Rustduck started...");
    println!("Adding to boot programs...");
}




fn create_config(config_path: &PathBuf) {
        // get input
        input("Please open https://duckdns.org and copy your token and find your domains.\nPress Enter To Continue");
        clear_terminal();
        let token = input("token: ");
        let domains = input("domains (Comma separated): ");
        let domains: Vec<String> = domains
        .trim()
        .split(',')
        .map(|domain| domain.trim().to_string())
        .collect();
        
    
        // create config struct
        let config = Config {
            domains: domains,
            token: token.to_string(),
            duration: String::from("10m")
        };
    
        // write config
        let mut config_file = File::create(config_path).unwrap();
        let config_str = serde_json::to_string(&config).unwrap();
        config_file.write_all(config_str.as_bytes()).unwrap();
}


fn kill_proc_by_name(name: &str) {
    let mut sys = System::default();
    sys.refresh_all();
    for (_pid, proc) in sys.processes() {
        if proc.name() == name {
            println!("Found running instance.\nStopping...");
            proc.kill();
        }
    }
}


fn installer() -> Result<()> {
    clear_terminal();
    kill_proc_by_name(get_binary_name!());
    let binary_bytes = get_binary_bytes!();
    let rustduck_dir = get_home().join(".rustduck");
    let config_path = rustduck_dir.join("duckdns.config.json");
    let binary_path = rustduck_dir.join(get_binary_name!());

    // create folders
    create_directory(rustduck_dir.to_str().unwrap(), true)?;
    create_config(&config_path);

    // write binary and close file (scop closing)
    #[cfg(windows)]
    {
        use std::fs::OpenOptions;
        let mut binary_file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&binary_path)
            .unwrap();
        binary_file.write_all(binary_bytes).unwrap();
    }
    
    // Add permissions in non-Windows environments
    #[cfg(not(windows))]
    {
        use std::fs::OpenOptions;
        use std::os::unix::fs::OpenOptionsExt;
        
        let mut binary_file = OpenOptions::new()
            .create(true)
            .write(true)
            .mode(0o777) // Set the mode to allow read, write, and execute for all
            .open(&binary_path)
            .unwrap();
        binary_file.write_all(binary_bytes).unwrap();
    }

    println!("Config files written to {}", rustduck_dir.to_str().unwrap());

    start_program(&rustduck_dir);
    let builder = AutoLaunchBuilder::new()
    .set_app_name("rustduck")
    .set_app_path(binary_path.as_os_str().to_str().unwrap())
    .set_use_launch_agent(true)
    .build()
    .unwrap();
    builder.enable().unwrap();
    Ok(())
}

fn clear_terminal() {
    print!("{}[2J", 27 as char);
}

fn input(prompt: &str) -> String {
    let mut line = String::new();
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).unwrap();
    return line.trim().to_string();
}

fn ask_reinstall() -> Result<()> {
    let yes_no = input("Looks like rustduck already installed.\nDo you want to reinstall it? (y/n): ");
    match yes_no.trim() {
        "n" => {
            process::exit(0);
        }
        _ => {
            installer()?;
        }
    }
    Ok(())
}



fn main() -> Result<()> {
    // clear_terminal();
    println!("Welcome to RustDuck Installer\n");

    let home = get_home();
    let rustduck_dir = home.join(".rustduck");

    let result = panic::catch_unwind(|| {
        match rustduck_dir.exists() {
            true => {
                ask_reinstall()?;
            }
            _ => {
                return installer();
            }
        }
        Ok(())
    });
    if let Err(err) = result {
        eprintln!("Panic occurred: {:?}", err);
        // You can add custom error handling here
    }
    input("Press ENTER to Continue");
    Ok(())
}
