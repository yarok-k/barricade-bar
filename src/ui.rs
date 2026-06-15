use crate::widgets::clock::Clock;
use crate::widgets::controls::controls::Controls;
use crate::widgets::layout::Layout;
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Ui {}

impl Ui {
    pub fn new(
        window: &gtk::ApplicationWindow,
        app: &gtk::Application,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let main_container = gtk::Grid::new();
        main_container.set_column_homogeneous(true);
        main_container.set_hexpand(true);

        let left_container = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        left_container.set_halign(gtk::Align::Start);
        let center_container = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        center_container.set_halign(gtk::Align::Center);
        center_container.set_hexpand(true);

        let right_container = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        right_container.set_halign(gtk::Align::End);

        // Настройки выравнивания для колонок

        let butt = gtk::Button::new();
        left_container.add(&butt);

        main_container.attach(&left_container, 0, 0, 1, 1);
        main_container.attach(&center_container, 1, 0, 1, 1);
        main_container.attach(&right_container, 2, 0, 1, 1);
        window.add(&main_container);

        let clock = Clock::new();
        center_container.add(&clock.trigger);

        let layout = Layout::new()?;
        right_container.pack_start(&layout.trigger, false, false, 0);

        let controls = Controls::new()?;
        right_container.pack_end(&controls.container, false, false, 0);

        let clock_rc = Rc::new(RefCell::new(clock));
        let layout_rc = Rc::new(RefCell::new(layout));
        let controls_rc = Rc::new(RefCell::new(controls));

        let clock_clone = clock_rc.clone();
        let layout_clone = layout_rc.clone();
        let controls_clone = controls_rc.clone();
        let mut update_flag: i8 = 1;
        gtk::glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
            match update_flag {
                2 => {
                    layout_clone.borrow_mut().update();
                    controls_clone.borrow_mut().update();
                    update_flag = update_flag + 1;
                }
                8 => {
                    clock_clone.borrow_mut().update();
                    update_flag = update_flag + 1;
                }
                _ => {
                    update_flag = update_flag + 1;
                }
            }
            gtk::glib::ControlFlow::Continue
        });
        Ok(Self {})
    }
}
