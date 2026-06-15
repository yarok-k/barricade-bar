use crate::lib::battery::BatteryLib;
use gtk::glib;
use gtk::glib::clone;
use gtk::prelude::*;
#[derive(Clone)]
pub struct BatteryWidget {
    pub trigger: gtk::Button,
    pub indicator: gtk::Image,
    // pub revealer: gtk::Revealer,
    pub text: gtk::Label,
    battery_lib: BatteryLib,
}

impl BatteryWidget {
    pub fn new() -> Self {
        let battery_lib = BatteryLib::new();
        let internal_container = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        let indicator = gtk::Image::builder()
            .icon_name("battery-full-symbolic")
            .build();

        let revealer = gtk::Revealer::new();
        revealer.set_transition_type(gtk::RevealerTransitionType::SlideRight);
        revealer.set_transition_duration(300);
        revealer.set_reveal_child(false); // Изначально скрыто

        let text = gtk::Label::builder().label("100%").build();

        revealer.add(&text);

        // Собираем всё в internal_container
        internal_container.pack_start(&indicator, false, false, 0);
        internal_container.pack_start(&revealer, false, false, 0);

        // Основной контейнер модуля
        let trigger = gtk::Button::new();
        trigger.add(&internal_container);

        // Сигналы
        trigger.connect_enter_notify_event(
            clone!(@weak revealer => @default-return glib::Propagation::Proceed, move |_, _| {
                revealer.set_reveal_child(true);
                glib::Propagation::Proceed
            }),
        );

        trigger.connect_leave_notify_event(
            clone!(@weak revealer => @default-return glib::Propagation::Proceed, move |_, _| {
                revealer.set_reveal_child(false);
                glib::Propagation::Proceed
            }),
        );
        Self {
            trigger,
            indicator,
            // revealer,
            text,
            battery_lib,
        }
    }
    pub fn update(&mut self) -> Result<(), anyhow::Error> {
        self.battery_lib.update()?;
        if self.battery_lib.capacity == -1 {
            self.trigger.set_visible(false);
            return Ok(());
        } else {
            self.trigger.set_visible(true);
            self.text
                .set_label(&format!("{}%", self.battery_lib.capacity));
            self.indicator.set_from_icon_name(
                Some(&self.battery_lib.get_icon_name().to_string()),
                gtk::IconSize::Button,
            );
        }
        Ok(())
    }
}
