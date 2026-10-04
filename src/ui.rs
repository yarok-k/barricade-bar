use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use crate::lib::core::placer::{Placer, ElementPos};
pub struct Ui {}

impl Ui {
    pub fn new(
        window: &gtk::ApplicationWindow,
        app: &gtk::Application,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // создадим контейнер для добавления в него всей остальной порнографии
        // применим grid чтобы можно было ровно рапсполложить элементы слева, справа и по центру
        let main_container = gtk::Grid::new();
        main_container.set_column_homogeneous(true);
        main_container.set_hexpand(true);
        window.add(&main_container);
        // генерируем контейнеры для расположения в них виджетов, далее обращаемся только к ним
        let left_container = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        left_container.set_halign(gtk::Align::Start);
        main_container.attach(&left_container, 0, 0, 1, 1);
        let center_container = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        center_container.set_halign(gtk::Align::Center);
        center_container.set_hexpand(true);
        main_container.attach(&center_container, 1, 0, 1, 1);
        let right_container = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        right_container.set_halign(gtk::Align::End);
        main_container.attach(&right_container, 2, 0, 1, 1);


        // карта нашей панели для плейсера
        let containers = [
            (ElementPos::Left, &left_container),
            (ElementPos::Center, &center_container),
            (ElementPos::Right, &right_container),
        ];
        let placer = Placer::new(&containers)?;
        // управление таймерами

        let placer_rc = Rc::new(RefCell::new(placer));
        gtk::glib::timeout_add_local(std::time::Duration::from_millis(500), move || {
            placer_rc.borrow_mut().update_widgets();
            gtk::glib::ControlFlow::Continue
        });
        Ok(Self {})
    }
}
