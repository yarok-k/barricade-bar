use gtk::prelude::*;
#[derive(Clone)]
pub struct PowerWidget {
    pub trigger: gtk::Button,
    pub indicator: gtk::Image,
}

impl PowerWidget {
    pub fn new() -> Self {
        let internal_container = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        let indicator = gtk::Image::builder().icon_name("system-settings").build();

        // Собираем всё в internal_container
        internal_container.pack_start(&indicator, false, false, 0);

        // Основной контейнер модуля
        let trigger = gtk::Button::new();
        trigger.add(&internal_container);

        trigger.connect_clicked(|_| {
            if let Err(e) = std::process::Command::new(
                "/home/yarok/projects/barricade-lock/target/release/barricade-lock",
            )
            .spawn()
            {
                eprintln!("Не удалось запустить reboot: {}", e);
            }
        });

        Self { trigger, indicator }
    }
}
