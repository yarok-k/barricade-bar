use gtk::prelude::*;
use std::cell::RefCell;

#[derive(Clone)]
pub struct Clock {
    pub trigger: gtk::Button,
    pub text: gtk::Label,
    pub time: RefCell<String>,
}
impl Clock {
    pub fn new() -> Self {
        let trigger = gtk::Button::new();
        let text = gtk::Label::new(Some("00:00:00"));
        trigger.set_child(Some(&text));
        let clock = Self {
            trigger,
            text,
            time: RefCell::new("".to_string()),
        };
        clock.update();
        clock
    }
    pub fn update(&self) {
        let now = chrono::Local::now()
            .format("%H:%M:%S | %d-%m-%Y")
            .to_string();

        self.text.set_label(&now);
        *self.time.borrow_mut() = now;
    }
}
