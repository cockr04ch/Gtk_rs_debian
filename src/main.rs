use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Builder, StackSidebar, ToggleButton};

const APP_ID: &str = "org.gtk_rs.HelloWorld2";
const UI_FILE: &str = "ui/app.ui"; // Ruta al archivo UI

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    // Cargar el archivo UI
    let builder = Builder::from_file(UI_FILE);

    // Obtener la ventana principal
    let window: ApplicationWindow = builder
        .object("main_window")
        .expect("No se encontró 'main_window' en el archivo UI");

    // Configurar la aplicación para la ventana
    window.set_application(Some(app));

    // Obtener los widgets que necesitamos controlar
    let sidebar: StackSidebar = builder
        .object("sidebar")
        .expect("No se encontró 'sidebar' en el archivo UI");
    let toggle_button: ToggleButton = builder
        .object("sidebar_toggle_button")
        .expect("No se encontró 'sidebar_toggle_button' en el archivo UI");

    // Conectar la señal del botón de toggle
    toggle_button.connect_toggled(glib::clone!(@weak sidebar => move |btn| {
        sidebar.set_visible(btn.is_active());
    }));

    // Mostrar la ventana
    window.present();
}
