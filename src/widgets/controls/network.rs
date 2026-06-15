use crate::lib::network::NetworkLib;
use gtk::prelude::*;

#[derive(Clone)]
pub struct NetworkWidget {
    pub trigger: gtk::Button,
    pub indicator: gtk::Image,
    net_lib: NetworkLib,
}

impl NetworkWidget {
    pub fn new() -> Self {
        let internal_container = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        let indicator = gtk::Image::builder()
            .icon_name("network-wired-symbolic")
            .build();

        // Собираем всё в internal_container
        internal_container.pack_start(&indicator, false, false, 0);

        // Основной контейнер модуля
        let trigger = gtk::Button::new();
        trigger.add(&internal_container);

        Self {
            trigger,
            indicator,
            net_lib: NetworkLib::new(),
        }
    }
    pub fn update(&mut self) -> Result<(), anyhow::Error> {
        self.net_lib.update()?;
        self.indicator
            .set_from_icon_name(Some(self.net_lib.get_icon_name()), gtk::IconSize::Button);
        Ok(())
    }
}
