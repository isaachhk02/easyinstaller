use std::env;
mod lang;
mod window;
use dialog::DialogBox;
use window::welcome_screen;

fn main() {
    let language;
    let mut args = env::args();
    let mut path;
    if let Some(arg) = args.nth(1) {
        
        path = arg;
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
            welcome_screen(language.as_str(),"apt", &path);
        } else if path.ends_with(".rpm") {
            println!("RPM file detected!");
            welcome_screen(language.as_str(),"dnf", &path);
        } else {
            let file_chooser = dialog::FileSelection::new("Please select a file").show();
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
                Err(_) => {
                    println!("ERROR : Please select a deb or rpm file!");
                    return;
                }
            }
        }
        println!("Language: {}", language);
    } 
    
}
