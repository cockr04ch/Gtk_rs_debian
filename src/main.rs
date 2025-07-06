use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Builder, Button, Spinner, StackSidebar, Switch, ToggleButton,
    glib,
};
use reqwest::get;
use std::fs::File;
use std::io::copy;

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
    let loading_spinner_clone = loading_spinner.clone();

    apply_button.connect_clicked(move |_| {
        let facebook_active = switch_facebook_clone.is_active();
        let steven_active = switch_steven_clone.is_active();
        let spinner = loading_spinner_clone.clone();

        glib::spawn_future_local(async move {
            spinner.set_visible(true);
            if facebook_active {
                println!("Descargando archivo de Facebook...");
                if let Err(e) = download_file(
                    "https://proof.ovh.net/files/100Mio.dat",
                    "facebook_download.dat",
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
                if let Err(e) = download_file(
                    "https://proof.ovh.net/files/10Mio.dat",
                    "steven_download.dat",
                )
                .await
                {
                    eprintln!("Error al descargar el archivo de Steven: {}", e);
                } else {
                    println!("Descarga de Steven completada.");
                }
            }
            spinner.set_visible(false);
        });
    });

    window.present();
}

async fn download_file(url: &str, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = get(url).await?;
    let mut dest = File::create(file_name)?;
    let content = response.bytes().await?;
    copy(&mut content.as_ref(), &mut dest)?;
    Ok(())
}
