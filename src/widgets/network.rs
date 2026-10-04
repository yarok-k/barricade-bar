use anyhow::Result;
use gtk::prelude::*;
use crate::lib::core::widget::WidgetTrait;
use crate::lib::network::NetworkLib;

#[derive(Clone)]
pub struct Network {
    pub trigger: gtk::Button,
    pub indicator: gtk::Image,
    net_lib: NetworkLib,
}

impl WidgetTrait for Network {
    fn new() -> Result<Self> {
        let internal_container = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        let indicator = gtk::Image::builder()
            .icon_name("network-wired-symbolic")
            .build();

        // Собираем всё в internal_container
        internal_container.pack_start(&indicator, false, false, 0);

        // Основной контейнер модуля
        let trigger = gtk::Button::new();
        trigger.add(&internal_container);

        Ok(Self {
            trigger,
            indicator,
            net_lib: NetworkLib::new(),
        })
    }
    fn update(&mut self) -> Result<()> {
        self.net_lib.update()?;
        self.indicator
            .set_from_icon_name(Some(self.net_lib.get_icon_name()), gtk::IconSize::Button);
        Ok(())
    }
}
