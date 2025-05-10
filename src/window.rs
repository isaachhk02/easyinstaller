use dialog::backends::{Backend, Zenity};
use dialog::{DialogBox, Message, Question, Choice};
pub fn installation_type(language: &str, package_manager: &str, package_path: &str) {
    let uninstall;
    let install_or_remove_package_msgs = match language {
        "en_US" => "Do you want to install or remove a package ? (press yes to install or press no to uninstall)",
        "fr_FR" => "Voulez-vous installer ou supprimer un paquet?",
        "es_ES" => "¿Quieres instalar o eliminar un paquete? (Pulsa Si para instalar el paquete de lo contrario pulsa No para eliminarlo",
        _ => "Do you want to install or remove a package ? (press yes to install or press no to uninstall)",
    };

    let install_or_remove_package = Question::new(install_or_remove_package_msgs).title("Do you want?").show();
    if install_or_remove_package.is_ok() {
        
        match install_or_remove_package {
            Ok(Choice::Yes) => {
                uninstall = 1;
                welcome_screen(language, package_manager, package_path, uninstall);
                ()
            }
            Ok(Choice::No) | Ok(Choice::Cancel) => {
                uninstall = 0;
                welcome_screen(language, package_manager, package_path, uninstall);
                ()
            }
            Err(_) => {
                uninstall = 0;
                welcome_screen(language, package_manager, package_path, uninstall);
                ()
            }
        }
    } else {
        uninstall = 0;
        welcome_screen(language, package_manager, package_path, uninstall);
        }
}


pub fn welcome_screen(language: &str, package_manager: &str, package_path: &str, uninstall: i32) {
    match package_manager {
        "apt" => "/usr/bin/apt",
        "dnf" => "/usr/bin/dnf",
        _ => "/usr/bin/apt",
    };
    let package_path = match package_manager {
        "apt" => format!("file://{}", package_path),
        "dnf" => format!("file://{}", package_path),
        _ => format!("file://{}", package_path),
    };
    let title = match language {
        "en_US" => "Welcome to easyInstaller",
        "fr_FR" => "Bienvenue dans easyInstaller",
        "es_ES" => "Bienvenido a easyInstaller",
        _ => "Welcome to easyInstaller",
    };
    let message = match language {
        "en_US" => format!("Welcome to easyInstaller\n\nThis is a simple GUI for installing packages on Linux.\n\nPlease click to install for install the specified package."),
        "fr_FR" => format!("Bienvenue dans easyInstaller\n\nCeci est une interface graphique simple pour installer des paquets sur Linux.\n\nVeuillez cliquer pour installer le paquet spécifié."),
        "es_ES" => format!("Bienvenido a easyInstaller\n\nEsta es una interfaz gráfica simple para instalar paquetes en Linux.\n\nHaga clic para instalar el paquete especificado."),
        _ => format!("Welcome to easyInstaller\n\nThis is a simple GUI for installing packages on Linux.\n\nPlease click to install for install the specified package."),
    };
    let uninstall_msgs = match language {
        "en_US" => format!("Do you want to uninstall the package {}?", package_path),
        "fr_FR" => format!("Voulez-vous désinstaller le paquet {}?", package_path),
        "es_ES" => format!("¿Desea desinstalar el paquete {}?", package_path),
        _ => format!("Do you want to uninstall the package {}?", package_path),
    };
    let msg_confirm = match language {
        "en_US" => "Do you want to install the package?",
        "fr_FR" => "Voulez-vous installer le paquet ?",
        "es_ES" => "¿Quieres instalar el paquete?",
        _ => "Do you want to install the package?",
    };
    let welcomemsg = Message::new(message.clone()).title(title).show();
    if welcomemsg.is_ok() {
        if uninstall == 1 {
            let uninstallmsg = Question::new(uninstall_msgs).show();
            match uninstallmsg {
                Ok(Choice::Yes) => {
                    uninstall_package(language, package_manager, &package_path);
                }
                Ok(Choice::No) | Ok(Choice::Cancel) => {
                    println!("Installation canceled");
                }
                Err(_) => {
                    println!("Failed to get uninstall confirmation");
                }
            }
        } else {
            let installmsg = Question::new(msg_confirm).show();
            match installmsg {
                Ok(Choice::Yes) => {
                    install_package(language, package_manager, &package_path);
                }
                Ok(Choice::No) | Ok(Choice::Cancel) => {
                    println!("Installation cancelled");
                }
                Err(_) => {
                    println!("Failed to get install confirmation");
                    ()
                }
            }
        }
    }




pub fn uninstall_package(language: &str, package_manager: &str, package_path: &str) {
    let command = format!("{} remove -y {}", package_manager, package_path);
    let output = std::process::Command::new("pkexec")
        .arg("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Failed to execute command");
    if output.status.success() {
        let success_message = match language {
            "en_US" => "Package uninstalled successfully!",
            "fr_FR" => "Paquet desinstallé avec succès !",
            "es_ES" => "¡Paquete desinstalado con éxito!",
            _ => "Package uninstalled successfully!",
        };
        let success_message_obj = Message::new(success_message);
        let _ = Zenity::new().show_message(&success_message_obj);
    } else {
        let error_message = match language {
            "en_US" => "Failed to uninstall package!",
            "fr_FR" => "Échec de l'desinstaller du paquet !",
            "es_ES" => "¡Error al desinstalar el paquete!",
            _ => "Failed to uninstall package!",
        };
        let error_message_obj = Message::new(error_message);
        let _ = Zenity::new().show_message(&error_message_obj);
    }
}
pub fn install_package(language: &str, package_manager: &str, package_path: &str) {
    let command = format!("{} install -y {}", package_manager, package_path);
    let output = std::process::Command::new("pkexec")
        .arg("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Failed to execute command");
    if output.status.success() {
        let success_message = match language {
            "en_US" => "Package installed successfully!",
            "fr_FR" => "Paquet installé avec succès !",
            "es_ES" => "¡Paquete instalado con éxito!",
            _ => "Package installed successfully!",
        };
        let success_message_obj = Message::new(success_message);
        let _ = Zenity::new().show_message(&success_message_obj);
    } else {
        let error_message = match language {
            "en_US" => "Failed to install package!",
            "fr_FR" => "Échec de l'installation du paquet !",
            "es_ES" => "¡Error al instalar el paquete!",
            _ => "Failed to install package!",
        };
        let error_message_obj = Message::new(error_message);
        let _ = Zenity::new().show_message(&error_message_obj);
    }
    }
}
