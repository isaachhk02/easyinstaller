use std::env::{self};
mod lang;
mod window;
use dialog::DialogBox;
use window::welcome_screen;

fn main() {
    let language;
    let mut args = env::args();
    let mut _action: String = String::new();
    let mut path;
    match args.nth(1) {
        Some(action) => {
            _action = action;
            _action = if _action.is_empty() {
                "install".to_string()
            } else {
                _action.clone()
            };
            path = match args.next() {
                Some(arg) => arg,
                None => {
                    eprintln!("ERROR: No package specified!");
                    return;
                }
            };
            match lang::get_system_language() {
                Some(lang) => {
                    language = lang.to_string();
                    println!("System language detected: {}", lang);
                }
                None => {
                    println!("ERROR : Please set the language in the code!");
                    return;
                }
            }
            if path.ends_with(".deb") {
                println!("Deb file detected!");
                welcome_screen(language.as_str(), "apt", &path, &_action);
            } else if path.ends_with(".rpm") {
                println!("RPM file detected!");
                welcome_screen(language.as_str(), "dnf", &path, &_action);
            } else {
                let file_chooser = dialog::FileSelection::new("Please select a file").show();
                println!("File chooser: {:?}", file_chooser);
                match file_chooser {
                    Ok(file) => {
                        if let Some(file_path) = file {
                            path = file_path.to_string();
                            if path.ends_with(".deb") {
                                println!("Deb file detected!");
                                welcome_screen(language.as_str(), "apt", &path, &_action);
                            } else if path.ends_with(".rpm") {
                                println!("RPM file detected!");
                                welcome_screen(language.as_str(), "dnf", &path, &_action);
                            } else {
                                println!("ERROR : Please select a deb or rpm file!");
                                return;
                            }
                        } else {
                            println!("ERROR : No file selected!");
                            return;
                        }
                    }
                    Err(_) => {
                        println!("ERROR : Please select a deb or rpm file!");
                        return;
                    }
                }
            }
            println!("Language: {}", language);
        }
        None => eprintln!("easyInstaller 0.1 by isaachhk02\nSyntax:\neasyinstall [action] [package]\nActions:\ninstall:Install the package\nremove:Uninstall package\n")
    } 
    
}
