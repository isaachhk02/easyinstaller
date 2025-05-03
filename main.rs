use std::env;
mod language;
mod window;
use dialog::DialogBox;
use window::welcome_screen;

fn main() {
    let mut language = String::new();
    let mut args = env::args();
    let mut path = String::new();
    if let Some(arg) = args.nth(1) {
        path = arg;
        match language::get_system_language() {
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
            welcome_screen(language.as_str(),"apt", &path);
        } 
        if path.ends_with(".rpm") {
            println!("RPM file detected!");
            welcome_screen(language.as_str(),"dnf", &path);
        }
    } else {
        println!("No arguments provided. Please provide a file path.");
        let file_chooser: Result<Option<String>, dialog::Error> = dialog::FileSelection::new("Please select a file").mode(dialog::FileSelectionMode::Open).show();
                println!("File chooser: {:?}", file_chooser);
            match file_chooser {
                Ok(file) => {
                    if let Some(file_path) = file {
                        path = file_path.to_string();
                        if path.ends_with(".deb") {
                            println!("Deb file detected!");
                            welcome_screen(language.as_str(),"apt", &path);
                        } else if path.ends_with(".rpm") {
                            println!("RPM file detected!");
                            welcome_screen(language.as_str(),"dnf", &path);
                        } else {
                            println!("ERROR : Please select a deb or rpm file!");
                            return;
                        }
                    } else {
                        println!("ERROR : No file selected!");
                        return;
                    }

                }
                Err(e) => {
                    println!("ERROR : {}", e);
                    return;
                }
            }
        
        println!("Language: {}", language);
    }
}