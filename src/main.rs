use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Builder, Button, Spinner, StackSidebar, Switch, ToggleButton,
    glib,
};
use reqwest::get;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::env;
use std::path::PathBuf;

const APP_ID: &str = "org.gtk_rs.HelloWorld2";
const UI_FILE: &str = "ui/app.ui"; // Ruta al archivo UI

#[tokio::main]
async fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let builder = Builder::from_file(UI_FILE);

    let window: ApplicationWindow = builder
        .object("main_window")
        .expect("No se encontró 'main_window' en el archivo UI");
    window.set_application(Some(app));

    let switch_facebook: Switch = builder
        .object("switch_facebook")
        .expect("No se pudo encontrar el switch de Facebook");
    let switch_steven: Switch = builder
        .object("switch_steven")
        .expect("No se pudo encontrar el switch de Steven");
    let switch_multi_pro: Switch = builder
        .object("switch_multi_pro")
        .expect("No se pudo encontrar el switch de Multi PRO");
    let apply_button: Button = builder
        .object("apply_button")
        .expect("No se pudo encontrar el botón de aplicar");
    let loading_spinner: Spinner = builder
        .object("loading_spinner")
        .expect("No se pudo encontrar el spinner de carga");
    let sidebar: StackSidebar = builder
        .object("sidebar")
        .expect("No se encontró 'sidebar' en el archivo UI");
    let toggle_button: ToggleButton = builder
        .object("sidebar_toggle_button")
        .expect("No se encontró 'sidebar_toggle_button' en el archivo UI");

    let sidebar_clone = sidebar.clone();
    toggle_button.connect_toggled(move |btn| {
        sidebar_clone.set_visible(btn.is_active());
    });

    let switch_facebook_clone = switch_facebook.clone();
    let switch_steven_clone = switch_steven.clone();
    let switch_multi_pro_clone = switch_multi_pro.clone();
    let loading_spinner_clone = loading_spinner.clone();

    apply_button.connect_clicked(move |_| {
        let facebook_active = switch_facebook_clone.is_active();
        let steven_active = switch_steven_clone.is_active();
        let multi_pro_active = switch_multi_pro_clone.is_active();
        let spinner = loading_spinner_clone.clone();

        glib::spawn_future_local(async move {
            spinner.set_visible(true);

            let system_root = env::var("SystemRoot").unwrap_or_else(|_| "C:\\Windows".to_string());
            let mut hosts_path = PathBuf::from(system_root);
            hosts_path.push("system32");
            hosts_path.push("drivers");
            hosts_path.push("etc");
            hosts_path.push("hosts");

            let mut file = match File::create(&hosts_path) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Error al crear el archivo hosts: {}. Asegúrate de ejecutar como administrador.", e);
                    spinner.set_visible(false);
                    return;
                }
            };

            if facebook_active {
                println!("Descargando archivo de Facebook...");
                if let Err(e) = download_and_append_file(
                    "https://raw.githubusercontent.com/anudeepND/blacklist/master/facebook.txt",
                    &mut file,
                )
                .await
                {
                    eprintln!("Error al descargar el archivo de Facebook: {}", e);
                } else {
                    println!("Descarga de Facebook completada.");
                }
            }
            if steven_active {
                println!("Descargando archivo de Steven...");
                if let Err(e) = download_and_append_file(
                    "https://proof.ovh.net/files/10Mio.dat",
                    &mut file,
                )
                .await
                {
                    eprintln!("Error al descargar el archivo de Steven: {}", e);
                } else {
                    println!("Descarga de Steven completada.");
                }
            }
            if multi_pro_active {
                println!("Descargando archivo de Multi PRO...");
                if let Err(e) = download_and_append_file(
                    "https://raw.githubusercontent.com/hagezi/dns-blocklists/main/hosts/pro.txt",
                    &mut file,
                )
                .await
                {
                    eprintln!("Error al descargar el archivo de Multi PRO: {}", e);
                } else {
                    println!("Descarga de Multi PRO completada.");
                }
            }

            println!("Limpiando la caché de DNS...");
            let output = Command::new("ipconfig")
                .arg("/flushdns")
                .output();

            match output {
                Ok(output) => {
                    if output.status.success() {
                        println!("Caché de DNS limpiada correctamente.");
                    } else {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        eprintln!("Error al limpiar la caché de DNS: {}", stderr);
                    }
                }
                Err(e) => {
                    eprintln!("Error al ejecutar ipconfig /flushdns: {}", e);
                }
            }

            spinner.set_visible(false);
        });
    });

    window.present();
}

async fn download_and_append_file(
    url: &str,
    dest: &mut File,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = get(url).await?;
    let content = response.bytes().await?;
    dest.write_all(content.as_ref())?;
    Ok(())
}
