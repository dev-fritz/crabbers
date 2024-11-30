use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write, Read};
use std::process::{Command, Output};
use std::env;

use clap::CommandFactory;
use crossterm::{execute, style::{Color, Print, ResetColor, SetForegroundColor}, cursor::MoveTo};
use crossterm::terminal::{Clear, ClearType};
use serde::{Serialize, Deserialize};
use crate::service::errors::ServiceError;

#[derive(Serialize, Deserialize, Debug)]
pub struct Service {
    pub id: usize,
    pub name: String,
    pub color: String,
    pub start_command: Option<String>,
    pub stop_command: Option<String>,
    pub restart_command: Option<String>,
}

fn get_user_name() -> String {
    if cfg!(target_os = "windows") {
        env::var("USERNAME").expect("Failed to get username")
    } else {
        env::var("USER").expect("Failed to get username")
    }
}

fn get_path() -> String {
    let user = get_user_name();
    let path = format!("/home/{user}/.crabbers/services.json");

    path.to_string()
}

fn create_dir() {
    let user = get_user_name();
    let path = format!("/home/{user}/.crabbers");
    std::fs::create_dir_all(path).expect("Failed to create directory");
}

fn create_json_file() -> Result<File, std::io::Error> {
    create_dir();
    File::create(get_path())
}

pub fn read_services_from_json() -> Result<Vec<Service>, ServiceError> {
    let path = get_path();

    match File::open(&path) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).map_err(ServiceError::Io)?;

            let services: Vec<Service> = serde_json::from_str(&contents).map_err(ServiceError::SerdeJson)?;
            Ok(services)
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
            Ok(vec![])
        }
        Err(e) => {
            println!("Error on try access services file: {}", e);
            Err(ServiceError::Io(e))
        }
    }
}

pub fn write_service_to_json(service: Service) {
    let mut services = read_services_from_json().expect("Failed to read services");
    services.push(service);

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(get_path())
        .unwrap_or_else(|_| create_json_file().expect("Failed to create file"));

    let mut buffer = BufWriter::new(file);
    let json_data = serde_json::to_string_pretty(&services).expect("Failed to serialize services");

    todo!("Implement logic to write msg in screen of TUI, in PopUP");
    buffer.write_all(json_data.as_bytes()).expect("Failed to write data to JSON file");
    buffer.flush().expect("Failed to flush buffer");
}

fn rewrite_services_to_json(services: &[Service]) -> Result<(), std::io::Error> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(get_path())?;

    let mut buffer = BufWriter::new(file);
    let json_data = serde_json::to_string_pretty(services)?;

    buffer.write_all(json_data.as_bytes())?;
    buffer.flush()?;

    Ok(())
}

fn reorganize_ids() {
    let mut services = read_services_from_json().expect("Failed to read services");

    for (index, service) in services.iter_mut().enumerate() {
        service.id = index + 1;
    }

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(get_path())
        .unwrap_or_else(|_| create_json_file().expect("Failed to create file"));

    let mut buffer = BufWriter::new(file);
    let json_data = serde_json::to_string_pretty(&services).expect("Failed to serialize services");

    buffer.write_all(json_data.as_bytes()).expect("Failed to write data to JSON file");
    buffer.flush().expect("Failed to flush buffer");
}

pub fn add_service(
    name: String,
    color: String,
    start_command: Option<String>,
    stop_command: Option<String>,
    restart_command: Option<String>
) {
    let services = read_services_from_json().expect("Failed to read services");

    let service = Service {
        id: services.len() + 1,
        name,
        color,
        start_command,
        stop_command,
        restart_command
    };

    write_service_to_json(service);
    println!("Service added.");
}

pub fn list_services() -> Vec<Service> {
    let services = read_services_from_json().expect("Failed to read services");

    services
}

pub fn edit_service_in_json(
    id: usize,
    name: Option<String>,
    color: Option<String>,
    start_command: Option<String>,
    stop_command: Option<String>,
    restart_command: Option<String>
) {
    let mut services = read_services_from_json().expect("Failed to read services");

    if let Some(service) = services.iter_mut().find(|s| s.id == id) {
        if let Some(new_name) = name {
            service.name = new_name;
        }
        if let Some(new_color) = color {
            service.color = new_color;
        }
        if start_command != None { service.start_command = start_command; };
        if stop_command != None { service.stop_command = stop_command; };
        if restart_command != None { service.restart_command = restart_command; };
    }

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(get_path())
        .expect("Failed to open JSON file");

    let mut buffer = BufWriter::new(file);
    let json_data = serde_json::to_string_pretty(&services).expect("Failed to serialize services");

    buffer.write_all(json_data.as_bytes()).expect("Failed to write data to JSON file");
    buffer.flush().expect("Failed to flush buffer");

    todo!("Print success message");
    println!("Service edited.");
}

pub fn remove_service(id: Option<usize>, name: Option<String>) {
    let mut services = match read_services_from_json() {
        Ok(services) => services,
        Err(e) => {
            eprintln!("Error reading services: {}", e);
            return;
        }
    };

    let original_len = services.len();
    services.retain(|s| {
        let id_matches = id.map_or(true, |id| s.id != id);
        let name_matches = name.as_ref().map_or(true, |n| s.name != *n);
        id_matches && name_matches
    });

    if services.len() == original_len {
        println!("No matching service found to remove.");
        return;
    }

    if let Err(e) = rewrite_services_to_json(&services) {
        todo!("Print error message");
        eprintln!("Error writing services to JSON file: {}", e);
        return;
    }

    reorganize_ids();

    println!("Service removed.");
}

pub fn execute(mut id: Option<usize>, name: Option<String>, command_type: i8) {
    let services = read_services_from_json().expect("Failed to read services");

    let service: &Service = if let Some(service_id) = id {
        match services.iter().find(|s| s.id == service_id) {
            Some(s) => s,
            None => {
                println!("Service not found.");
                return;
            }
        }
    } else if let Some(ref service_name) = name {
        match services.iter().find(|s| s.name == *service_name) {
            Some(s) => s,
            None => {
                println!("Service not found.");
                return;
            }
        }
    } else {
        unreachable!();
    };
    let output: Output;

    match command_type {
        1 => {
            if let Some(start_command) = &service.start_command {
                println!("Start service '{}' with command: '{}'", service.name, start_command);
                output = Command::new("sh")
                    .arg("-c")
                    .arg(start_command)
                    .output()
                    .expect("Error on execute command.");
            } else {
                println!("Command not implemented.");
                return;
            }
        },
        2 => {
            if let Some(stop_command) = &service.stop_command {
                println!("Stop service '{}' with command: '{}'", service.name, stop_command);
                output = Command::new("sh")
                    .arg("-c")
                    .arg(stop_command)
                    .output()
                    .expect("Error on execute command.");
            } else {
                println!("Command not implemented.");
                return;
            }
        },
        3 => {
            if let Some(restart_command) = &service.restart_command {
                println!("Restart service '{}' with command: '{}'", service.name, restart_command);
                output = Command::new("sh")
                    .arg("-c")
                    .arg(restart_command)
                    .output()
                    .expect("Error on execute command.");
            } else {
                println!("Command not implemented.");
                return;
            }
        },
        _ => {
            output = Command::new("sh")
                .arg("-c")
                .arg("echo 'Command not fount.'")
                .output()
                .expect("Error on execute command.");
        }
    }

    if output.status.success() {
        todo!("Print success message ");
        println!("Commmand executed with success.\n {}", String::from_utf8_lossy(&output.stdout));
    } else {
        todo!("Print error message");
        eprintln!("Error on execute command: {}", String::from_utf8_lossy(&output.stderr));
    }
}