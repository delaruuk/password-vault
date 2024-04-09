mod pentry;
use crate::pentry::prompt;
use crate::pentry::read_passwords_from_file;
use crate::pentry::ServiceInfo;

fn clr() {
    println!("\x1B[2J\x1B[0;0H");
}
fn main() {
    clr();
    let ascii = r#"
    __________                                               .___ ____   ____            .__   __
\______   \_____    ______ ________  _  _____________  __| _/ \   \ /   /____   __ __|  |_/  |_
 |     ___/\__  \  /  ___//  ___/\ \/ \/ /  _ \_  __ \/ __ |   \   Y   /\__  \ |  |  \  |\   __\
 |    |     / __ \_\___ \ \___ \  \     (  <_> )  | \/ /_/ |    \     /  / __ \|  |  /  |_|  |
 |____|    (____  /____  >____  >  \/\_/ \____/|__|  \____ |     \___/  (____  /____/|____/__|
                \/     \/     \/                          \/                 \/
    "#;

    println!("{ascii}");
    loop {
        println!("Password manager menu:");
        println!("1. Add a new password");
        println!("2. List passwords");
        println!("3. Search passwords");
        println!("4. Exit");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                clr();
                let entry = ServiceInfo::new(
                    prompt("Service :"),
                    prompt("Username :"),
                    prompt("Password :"),
                );
                println!("Entry added successfully!");
                entry.write_to_file();
            }
            "2" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|error| {
                    eprintln!("Error reading passwords: {}", error);
                    Vec::new()
                });
                for item in services {
                    println!("Service = {}
                            - Username : {}
                            - Password : {}",
                            item.service,
                            item.username,
                            item.password,
                        );
                }
            }
            "3" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|error| {
                    eprintln!("Error reading passwords: {}", error);
                    Vec::new()
                });
                let search = prompt("Search :");
                for item in &services{
                    if item.service.as_str() == search.as_str(){
                        println!(
                            "service = {}
                            - Username : {}
                            - Password : {}",
                            item.service,
                            item.username,
                            item.password,
                            );
                    }
                }
            }
            "4" => {
                clr();
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice");
            }
        }
        println!("\n\n");
    }
}
