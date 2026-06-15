use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use ui::Ui;

mod lib;
mod ui;
mod widgets;

fn main() -> Result<(), anyhow::Error> {
    let app = Application::builder()
        .application_id("org.example.Barricade.desktop")
        .build();
    app.connect_activate(move |app| {
        // Получаем дисплей по умолчанию
        let display = gtk::gdk::Display::default().expect("Could not get default display");

        // Итерируемся по всем подключенным мониторам
        let monitor_count = display.n_monitors();
        for i in 0..monitor_count {
            if let Some(monitor) = display.monitor(i) {
                create_window_for_monitor(app, &monitor);
            }
        }
        let app_clone = app.clone();
        display.connect_monitor_added(move |_, monitor| {
            create_window_for_monitor(&app_clone, monitor);
        });
    });

    let exit_code = app.run();
    if exit_code.value() != 0 {}
    Ok(())
}

fn create_window_for_monitor(app: &Application, monitor: &gtk::gdk::Monitor) {
    let win = ApplicationWindow::builder()
        .application(app)
        .decorated(false)
        .build();

    gtk_layer_shell::init_for_window(&win);
    gtk_layer_shell::set_monitor(&win, monitor);
    gtk_layer_shell::set_layer(&win, gtk_layer_shell::Layer::Top);
    gtk_layer_shell::set_anchor(&win, gtk_layer_shell::Edge::Top, true);
    gtk_layer_shell::set_anchor(&win, gtk_layer_shell::Edge::Left, true);
    gtk_layer_shell::set_anchor(&win, gtk_layer_shell::Edge::Right, true);
    gtk_layer_shell::set_exclusive_zone(&win, 35);
    let _ui = Ui::new(&win, app);

    win.show_all();
}
