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
    fn new(popup: Option<String>) -> Result<Self> {
        let trigger = gtk::Button::new();
        let trigger_style_context = trigger.style_context().add_class("barricade_statusbar_button");
        let text = gtk::Label::new(Some("00:00:00"));
        trigger.set_child(Some(&text));

        let mut clock = Self {
            trigger,
            text,
            time: RefCell::new(String::new()),
        };
        clock.update()?;
        Ok(clock)
    }

    fn update(&mut self) -> Result<()> {
        let now = chrono::Local::now()
            .format("%H:%M:%S | %d-%m-%Y")
            .to_string();

        self.text.set_label(&now);
        *self.time.borrow_mut() = now;
        Ok(())
    }
}
