use std::process::exit;

use dialog::backends::{Backend, Zenity};
use dialog::{DialogBox, Message, Question};



pub fn welcome_screen(language: &str, package_manager: &str, package_path: &str) {
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
    let msg_confirm = match language {
        "en_US" => "Do you want to install the package?",
        "fr_FR" => "Voulez-vous installer le paquet ?",
        "es_ES" => "¿Quieres instalar el paquete?",
        _ => "Do you want to install the package?",
    };
    let welcomemsg = Message::new(message.clone()).title(title).show();
    if welcomemsg.is_ok() {
        let install_question = Question::new(msg_confirm).title(title).show();
        if install_question.is_ok() {
            install_package(language, package_manager, &package_path)
        }
        if install_question.is_err() {
            let cancel_message = match language {
                "en_US" => "Installation cancelled!",
                "fr_FR" => "Installation annulée !",
                "es_ES" => "¡Instalación cancelada!",
                _ => "Installation cancelled!",
            };
            let cancel_message_obj = Message::new(cancel_message);
            let _ = Zenity::new().show_message(&cancel_message_obj);
        }
    } else {
        let error_message = match language {
            "en_US" => "Failed to show message!",
            "fr_FR" => "Échec de l'affichage du message !",
            "es_ES" => "¡Error al mostrar el mensaje!",
            _ => "Failed to show message!",
        };
        let error_message_obj = Message::new(error_message);
        let _ = Zenity::new().show_message(&error_message_obj);
        exit(-1);
    }
}
pub fn install_package(language: &str, package_manager: &str, package_path: &str) {
    let update_cmd = format!("{} update", package_manager);
    let install_cmd = format!("{} install {}", package_manager, package_path);
    let output = std::process::Command::new("pkexec")
        .arg("sh")
        .arg("-c")
        .arg(update_cmd)
        .arg("&&")
        .arg(install_cmd)
        .arg("-y")
        .output()
        .expect("Failed to execute command");
    match output.status.success() {
        true => {
            let success_message = match language {
                    "en_US" => "Package installed successfully!",
                    "fr_FR" => "Paquet installé avec succès !",
                    "es_ES" => "¡Paquete instalado con éxito!",
                    _ => "Package installed successfully!",
                };
            let success_message_obj = Message::new(success_message);
            let _ = Zenity::new().show_message(&success_message_obj);
        }
        false => {
            let error_message = match language {
                "en_US" => "Failed to install package!", 
                "fr_FR" => "Échec de l'installation du paquet !",
                "es_ES" => "¡Error al instalar el paquete!",
                _ => "Failed to install package!",
        
            };
            eprintln!{"Error code: {}",output.status};
            let error_message_obj = Message::new(error_message);
            let _ = Zenity::new().show_message(&error_message_obj);
        
        }
    }
}