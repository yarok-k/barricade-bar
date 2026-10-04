use anyhow::Result;
use gtk::prelude::*;
use std::cell::RefCell;
use crate::lib::core::widget::WidgetTrait;

#[derive(Clone)]
pub struct Clock {
    pub trigger: gtk::Button,
    pub text: gtk::Label,
    pub time: RefCell<String>,
}

impl WidgetTrait for Clock {
    fn new() -> Result<Self> {
        let trigger = gtk::Button::new();
        let text = gtk::Label::new(Some("00:00:00"));
        trigger.set_child(Some(&text));

        let clock = Self {
            trigger,
            text,
            time: RefCell::new(String::new()),
        };
        clock.update()?;
        Ok(clock)
    }

    fn update(&self) -> Result<()> {
        let now = chrono::Local::now()
            .format("%H:%M:%S | %d-%m-%Y")
            .to_string();

        self.text.set_label(&now);
        *self.time.borrow_mut() = now;
        Ok(())
    }
}
