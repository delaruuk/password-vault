use serde::{Deserialize, Serialize};
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::BufRead;
use std::io::Write;
#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub service: String,
    pub username: String,
    pub password: String,
}
impl ServiceInfo {
    pub fn new(service: &str, username: &str, password: &str) -> Self {
        ServiceInfo {
            service,
            username,
            password,
        }
    }
    pub fn from_json(json_string: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_string)
    }
    #[allow(dead_code)]
    pub fn from_user_input() -> Self {
        println!("Enter Password Entry");
        let mut service = String::new();
        io::stdin()
            .read_line(&mut service)
            .expect("Failed to read line");

        println!("Enter Username");
        let mut username = String::new();
        io::stdin()
            .read_line(&mut username)
            .expect("Failed to read line");

        println!("Enter Password");
        let mut password = String::new();
        io::stdin()
            .read_line(&mut password)
            .expect("Failed to read line");

        ServiceInfo::new(service.trim().to_string(),
                     username.trim().to_string(),
                     password.trim().to_string()
                    )
    }
    pub fn to_json(&self) -> String {
        serde_json::to_str(&self).expect("Failed to serialize JSON")
    }
    pub fn write_to_file(&self) {
        let json_output = format!("{}\n", self.to_json());
    }
}