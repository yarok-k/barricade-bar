use gtk::prelude::*;
use crate::widgets::{clock::Clock, audio::Audio};

// Enum готовых GTK-объектов
#[derive(Clone)]
pub enum BuiltWidget {
    Clock(Clock),
    Audio(Audio),
    // Battery(Battery),
}

impl BuiltWidget {
    // Возвращаем gtk::Widget по значению
    pub fn gtk_widget(&self) -> gtk::Widget {
        match self {
            BuiltWidget::Clock(clock) => clock.trigger.clone().upcast(),
            BuiltWidget::Audio(audio) => audio.trigger.clone().upcast(),
            _ => gtk::Button::builder().label("err").build().upcast(),
        }
    }
}
